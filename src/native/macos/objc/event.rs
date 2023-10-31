//! Code for handling `NSEvent`s. The `NSEvent`s will (almost always) be converted into Lokinit
//! `Event`s and returned so they can be handled. However, some events must be forwarded to
//! AppKit to function correctly, or need to be handled by Lokinit.
//!
//! Events that will not be forwarded:
//! - The app being activated
//! - Mouse pressed, moved, and released events, if the window is being resized (resized events
//! will be sent instead)

use super::{macros::*, *};

impl NSApp {
    pub fn handle(&self, event: NSEvent, backend: &mut MacosBackend) -> Option<Event> {
        let (nsevent, event_type, event_subtype, window_id, mouse_pos, nsapp, send_event) =
            VTables::with(|vtables| {
                (
                    vtables.nsevent.class,
                    vtables.nsevent.type_sel,
                    vtables.nsevent.subtype_sel,
                    vtables.nsevent.window_number_sel,
                    vtables.nsevent.mouse_location_sel,
                    vtables.nsapp.shared,
                    vtables.nsapp.send_event_sel,
                )
            });
        let instance = event.ptr;

        let window_id: isize = msg_ret![instance window_id];
        let window_id = window_id as usize;
        let event_type: NSEventType = msg_ret![instance event_type];
        // If we should change the window that's in front because this event was triggered.
        // This defaults to true, and is only false for some events like mouse moved.
        let mut change_frontmost_window = true;

        let event = match event_type {
            // Several of these event types use private APIs, forcing us to forward the event to
            // AppKit (trying to access those APIs could result in apps using Lokinit getting
            // rejected from the app store).
            NSEventType::AppKitDefined => {
                let event_subtype: NSEventSubtype = msg_ret![instance event_subtype];

                match event_subtype {
                    NSEventSubtype::ApplicationActivated
                    | NSEventSubtype::ApplicationDeactivated => {
                        let event = event.ptr;
                        msg![nsapp send_event sendEvent:event];
                        change_frontmost_window = false;
                        None
                    }
                    NSEventSubtype::WindowMoved => {
                        let window = backend.windows.get_mut(&window_id).unwrap();
                        // Yes, we will get window moved events while trying to resize... pretty cringe ngl
                        if window.resize_direction.is_some() {
                            return None;
                        }

                        let event = event.ptr;
                        msg![nsapp send_event sendEvent:event];
                        window.recalculate_window_rect();

                        Some(Event {
                            time: Duration::ZERO,
                            window: WindowHandle(window_id),
                            kind: EventKind::Moved(
                                window.rect.origin.x as i32,
                                window.rect.origin.y as i32,
                            ),
                        })
                    }
                    _ => {
                        change_frontmost_window = false;
                        None
                    }
                }
            }

            NSEventType::MouseMoved
            | NSEventType::LeftMouseDragged
            | NSEventType::RightMouseDragged => {
                // Mouse moved events don't have an associated window
                let window_id = if event_type == NSEventType::MouseMoved {
                    change_frontmost_window = false;
                    backend.frontmost_window.unwrap()
                } else {
                    window_id
                };

                let window = backend.windows.get_mut(&window_id).unwrap();
                let mouse_pos: NSPoint = msg_ret![nsevent mouse_pos];
                let mouse_pos = window.screen_point_to_local_point(mouse_pos);

                if let Some(border) = window.resize_direction {
                    match border {
                        NSWindowBorder::Top => {
                            window.rect.size.height = mouse_pos.y;
                        }
                        NSWindowBorder::Bottom => {
                            window.rect.origin.y += mouse_pos.y;
                            window.rect.size.height -= mouse_pos.y;
                        }
                        NSWindowBorder::Left => {
                            window.rect.origin.x += mouse_pos.x;
                            window.rect.size.width -= mouse_pos.x;
                        }
                        NSWindowBorder::Right => {
                            window.rect.size.width = mouse_pos.x;
                        }
                        NSWindowBorder::TopLeft => {
                            window.rect.size.height = mouse_pos.y;
                            window.rect.origin.x += mouse_pos.x;
                            window.rect.size.width -= mouse_pos.x;
                        }
                        NSWindowBorder::TopRight => {
                            window.rect.size.height = mouse_pos.y;
                            window.rect.size.width = mouse_pos.x;
                        }
                        NSWindowBorder::BottomLeft => {
                            window.rect.origin.y += mouse_pos.y;
                            window.rect.size.height -= mouse_pos.y;
                            window.rect.origin.x += mouse_pos.x;
                            window.rect.size.width -= mouse_pos.x;
                        }
                        NSWindowBorder::BottomRight => {
                            window.rect.origin.y += mouse_pos.y;
                            window.rect.size.height -= mouse_pos.y;
                            window.rect.size.width = mouse_pos.x;
                        }
                    }
                    window.apply_size();
                    window.recalculate_window_borders();

                    None
                } else {
                    window.update_hover_border(&mouse_pos);

                    Cursors::with(|cursors| {
                        if let Some(border) = window.hover_border {
                            match border {
                                NSWindowBorder::TopLeft | NSWindowBorder::BottomRight => {
                                    cursors.get(MacOsCursor::ResizeNorthWestSouthEast)
                                }
                                NSWindowBorder::TopRight | NSWindowBorder::BottomLeft => {
                                    cursors.get(MacOsCursor::ResizeNorthEastSouthWest)
                                }
                                NSWindowBorder::Top | NSWindowBorder::Bottom => {
                                    cursors.get(MacOsCursor::ResizeNorthSouth)
                                }
                                NSWindowBorder::Left | NSWindowBorder::Right => {
                                    cursors.get(MacOsCursor::ResizeEastWest)
                                }
                            }
                            .set();

                            // TODO: Should this return none?
                            None
                        } else {
                            if window.stoplight_buttons_rect.contains(&mouse_pos) {
                                // I spent several hours trying, but ultimately couldn't set the stoplight buttons'
                                // state myself. I think it uses undocumented/private APIs, which, again, we can't
                                // use if we want to be accepted onto the app store. So instead we pass this event
                                // to the OS for it to handle & update the buttons' states.
                                let event = event.ptr;
                                msg![nsapp send_event sendEvent:event];
                            }

                            cursors.get(MacOsCursor::Arrow).set();

                            Some(Event {
                                time: Duration::ZERO,
                                window: WindowHandle(window_id),
                                kind: EventKind::Mouse(MouseEvent::CursorMove(
                                    mouse_pos.x as i32,
                                    mouse_pos.y as i32,
                                )),
                            })
                        }
                    })
                }
            }

            NSEventType::LeftMouseDown => {
                let window = backend.windows.get_mut(&window_id).unwrap();

                if window.hover_border.is_some() {
                    window.resize_direction = window.hover_border;
                    None
                } else {
                    let mouse_pos: NSPoint = msg_ret![nsevent mouse_pos];

                    if window.stoplight_buttons_rect.contains(&mouse_pos) {
                        // I spent several hours trying, but ultimately couldn't set the stoplight buttons'
                        // state myself. I think it uses undocumented/private APIs, which, again, we can't
                        // use if we want to be accepted onto the app store. So instead we pass this event
                        // to the OS for it to handle & update the buttons' states.
                        let event = event.ptr;
                        msg![nsapp send_event sendEvent:event];
                        return None;
                    }

                    let NSPoint { x, y } = window.screen_point_to_local_point(mouse_pos);

                    Some(Event {
                        time: Duration::ZERO,
                        window: WindowHandle(window_id),
                        kind: EventKind::Mouse(MouseEvent::ButtonPress(
                            MouseButton::Left,
                            x as i32,
                            y as i32,
                        )),
                    })
                }
            }
            NSEventType::LeftMouseUp => {
                let window = backend.windows.get_mut(&window_id).unwrap();

                if window.resize_direction.is_some() {
                    window.resize_direction = None;
                    Cursors::with(|cursors| cursors.get(MacOsCursor::Arrow).set());
                    println!("Bruh");
                    None
                } else {
                    let mouse_pos: NSPoint = msg_ret![nsevent mouse_pos];
                    let NSPoint { x, y } = window.screen_point_to_local_point(mouse_pos);

                    Some(Event {
                        time: Duration::ZERO,
                        window: WindowHandle(window_id),
                        kind: EventKind::Mouse(MouseEvent::ButtonRelease(
                            MouseButton::Left,
                            x as i32,
                            y as i32,
                        )),
                    })
                }
            }

            NSEventType::RightMouseDown => {
                let window = backend.windows.get(&window_id).unwrap();
                let mouse_pos: NSPoint = msg_ret![nsevent mouse_pos];
                let NSPoint { x, y } = window.screen_point_to_local_point(mouse_pos);

                Some(Event {
                    time: Duration::ZERO,
                    window: WindowHandle(window_id),
                    kind: EventKind::Mouse(MouseEvent::ButtonPress(
                        MouseButton::Right,
                        x as i32,
                        y as i32,
                    )),
                })
            }
            NSEventType::RightMouseUp => {
                let window = backend.windows.get(&window_id).unwrap();
                let mouse_pos: NSPoint = msg_ret![nsevent mouse_pos];
                let NSPoint { x, y } = window.screen_point_to_local_point(mouse_pos);

                Some(Event {
                    time: Duration::ZERO,
                    window: WindowHandle(window_id),
                    kind: EventKind::Mouse(MouseEvent::ButtonRelease(
                        MouseButton::Right,
                        x as i32,
                        y as i32,
                    )),
                })
            }

            _ => {
                change_frontmost_window = false;
                let event = event.ptr;
                // We can't handle undocumented events or we'll be rejected from the app store,
                // so we instead have to hand those to macOS.
                msg![nsapp send_event sendEvent:event];
                None
            }
        };

        if change_frontmost_window {
            backend.set_frontmost_window(window_id);
        }

        event
    }
}
