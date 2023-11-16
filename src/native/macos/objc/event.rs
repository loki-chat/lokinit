//! Code for handling `NSEvent`s. The `NSEvent`s will (almost always) be converted into Lokinit
//! `Event`s and returned so they can be handled. However, some events must be forwarded to
//! AppKit to function correctly, or need to be handled by Lokinit.
//!
//! Examples of events that will not be forwarded:
//! - The app being activated
//! - Mouse pressed, moved, and released events, if the window is being resized (resize events
//! will be sent instead)

use super::{macros::*, *};

impl MacosBackend {
    pub fn handle(&mut self, event: NSEvent) -> Option<Event> {
        let (
            nsevent,
            event_type,
            event_subtype,
            window_id,
            mouse_pos,
            mouse_btn,
            nsapp,
            send_event,
        ) = (
            self.vtables.nsevent.class,
            self.vtables.nsevent.type_sel,
            self.vtables.nsevent.subtype_sel,
            self.vtables.nsevent.window_number_sel,
            self.vtables.nsevent.mouse_location_sel,
            self.vtables.nsevent.button_number_sel,
            self.vtables.nsapp.shared,
            self.vtables.nsapp.send_event_sel,
        );
        let instance = event.ptr;

        let window_id: isize = msg_ret![instance window_id];
        let window_id = window_id as usize;
        let event_type: NSEventType = msg_ret![instance event_type];

        // If we should change the window that's in front because this event was triggered.
        let mut change_frontmost_window = false;

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
                        None
                    }
                    NSEventSubtype::WindowMoved => {
                        let window = self.windows.get_mut(&window_id).unwrap();
                        // Yes, we will get window moved events while trying to resize... pretty cringe ngl
                        if window.resize_direction.is_some() {
                            return None;
                        }

                        let event = event.ptr;
                        msg![nsapp send_event sendEvent:event];
                        window.recalculate_window_rect(&self.vtables);

                        Some(Event {
                            time: Duration::ZERO,
                            window: WindowHandle(window_id),
                            kind: EventKind::Moved(
                                window.rect.origin.x as i32,
                                window.rect.origin.y as i32,
                            ),
                        })
                    }
                    _ => None,
                }
            }

            NSEventType::MouseMoved
            | NSEventType::LeftMouseDragged
            | NSEventType::RightMouseDragged => {
                let mouse_pos: NSPoint = msg_ret![nsevent mouse_pos];

                for window in self.windows.values_mut() {
                    let mouse_pos = window.screen_point_to_local_point(mouse_pos.clone());

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
                        window.apply_size(&self.vtables);
                        window.recalculate_window_borders(&self.vtables);

                        return Some(Event {
                            time: Duration::ZERO,
                            window: WindowHandle(window_id),
                            kind: EventKind::Resized(
                                window.rect.size.width as u32,
                                window.rect.size.height as u32,
                            ),
                        });
                    } else {
                        window.update_hover_border(&mouse_pos);
                        if let Some(border) = window.hover_border {
                            self.set_cursor(match border {
                                NSWindowBorder::TopLeft | NSWindowBorder::BottomRight => {
                                    MacOsCursor::ResizeNorthWestSouthEast
                                }
                                NSWindowBorder::TopRight | NSWindowBorder::BottomLeft => {
                                    MacOsCursor::ResizeNorthEastSouthWest
                                }
                                NSWindowBorder::Top | NSWindowBorder::Bottom => {
                                    MacOsCursor::ResizeNorthSouth
                                }
                                NSWindowBorder::Left | NSWindowBorder::Right => {
                                    MacOsCursor::ResizeEastWest
                                }
                            });

                            // TODO: Should this return none?
                            return None;
                        } else if window.stoplight_buttons_rect.contains(&mouse_pos) {
                            // I spent several hours trying, but ultimately couldn't set the stoplight buttons'
                            // state myself. I think it uses undocumented/private APIs, which, again, we can't
                            // use if we want to be accepted onto the app store. So instead we pass this event
                            // to the OS for it to handle & update the buttons' states.
                            println!("eek");
                            let event = event.ptr;
                            let get_btn = sel!("standardWindowButton:");
                            let super_ = sel!("superview");
                            let ptr = window.ptr;
                            for btn in 0..=2 {
                                let btn = msg_ret![ptr get_btn standardWindowButton:btn];
                                let btn = msg_ret![btn super_];
                                msg![btn send_event sendEvent:event];
                            }
                            msg![nsapp send_event sendEvent:event];
                        }
                    }
                }

                self.set_cursor(MacOsCursor::Arrow);

                let (window, mouse_pos) = if let Some(window) = self.windows.get(&window_id) {
                    (
                        WindowHandle(window_id),
                        window.screen_point_to_local_point(mouse_pos.clone()),
                    )
                } else {
                    let Some(window_id) = self.frontmost_window else {
                        return None;
                    };
                    let window = self.windows.get(&window_id).unwrap();
                    (
                        WindowHandle(window_id),
                        window.screen_point_to_local_point(mouse_pos.clone()),
                    )
                };

                Some(Event {
                    time: Duration::ZERO,
                    window,
                    kind: EventKind::Mouse(MouseEvent::CursorMove(
                        mouse_pos.x as i32,
                        mouse_pos.y as i32,
                    )),
                })
            }

            NSEventType::LeftMouseDown => {
                let window = self.windows.get_mut(&window_id).unwrap();

                if window.hover_border.is_some() {
                    self.in_resize = true;
                    window.resize_direction = window.hover_border;
                    None
                } else {
                    let mouse_pos: NSPoint = msg_ret![nsevent mouse_pos];

                    if window.stoplight_buttons_rect.contains(&mouse_pos) {
                        // I spent several hours trying, but ultimately couldn't set the stoplight buttons'
                        // state myself. I think it uses undocumented/private APIs, which, again, we can't
                        // use if we want to be accepted onto the app store. So instead we pass this event
                        // to the OS for it to handle & update the buttons' states.
                        let get_btn = sel!("standardWindowButton");
                        let ptr = window.ptr;
                        let event = event.ptr;
                        for btn in 0..=2 {
                            let btn = msg_ret![ptr get_btn standardWindowButton:btn];
                            msg![btn send_event sendEvent:event];
                        }
                        // msg![nsapp send_event sendEvent:event];
                        return None;
                    }

                    let NSPoint { x, y } = window.screen_point_to_local_point(mouse_pos);

                    change_frontmost_window = true;

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
                change_frontmost_window = true;

                let window = self.windows.get_mut(&window_id).unwrap();

                if window.resize_direction.is_some() {
                    self.in_resize = false;
                    window.resize_direction = None;
                    self.set_cursor(MacOsCursor::Arrow);

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
                let window = self.windows.get(&window_id).unwrap();
                let mouse_pos: NSPoint = msg_ret![nsevent mouse_pos];
                let NSPoint { x, y } = window.screen_point_to_local_point(mouse_pos);

                change_frontmost_window = true;

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
                let window = self.windows.get(&window_id).unwrap();
                let mouse_pos: NSPoint = msg_ret![nsevent mouse_pos];
                let NSPoint { x, y } = window.screen_point_to_local_point(mouse_pos);

                change_frontmost_window = true;

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

            NSEventType::OtherMouseDown => {
                let window = self.windows.get(&window_id).unwrap();
                let mouse_pos: NSPoint = msg_ret![nsevent mouse_pos];
                let mouse_btn: isize = msg_ret![nsevent mouse_btn];
                let NSPoint { x, y } = window.screen_point_to_local_point(mouse_pos);

                change_frontmost_window = true;

                Some(Event {
                    time: Duration::ZERO,
                    window: WindowHandle(window_id),
                    kind: EventKind::Mouse(MouseEvent::ButtonPress(
                        // TODO: One of the mouse button numbers is for the middle mouse button, need to figure out
                        // which, and use MouseButton::Middle for it instead of Other.
                        MouseButton::Other(mouse_btn as u16),
                        x as i32,
                        y as i32,
                    )),
                })
            }
            NSEventType::OtherMouseUp => {
                let window = self.windows.get(&window_id).unwrap();
                let mouse_pos: NSPoint = msg_ret![nsevent mouse_pos];
                let mouse_btn: isize = msg_ret![nsevent mouse_btn];
                let NSPoint { x, y } = window.screen_point_to_local_point(mouse_pos);

                change_frontmost_window = true;

                Some(Event {
                    time: Duration::ZERO,
                    window: WindowHandle(window_id),
                    kind: EventKind::Mouse(MouseEvent::ButtonRelease(
                        // TODO: One of the mouse button numbers is for the middle mouse button, need to figure out
                        // which, and use MouseButton::Middle for it instead of Other.
                        MouseButton::Other(mouse_btn as u16),
                        x as i32,
                        y as i32,
                    )),
                })
            }

            // TODO:
            // Handling these fixes mouse cursor bugs after setting resize icon
            // Handling these also breaks the stoplight buttons
            // Ideally would forward `MouseEntered`/`MouseExited` event directly instead of
            // forwarding mouse moved event for stoplight? Edit: Won't work due to hover on fullscreen btns.
            NSEventType::MouseEntered => {
                // let event = event.ptr;
                // msg![nsapp send_event sendEvent:event];
                None
            }
            NSEventType::MouseExited => {
                // let event = event.ptr;
                // msg![nsapp send_event sendEvent:event];
                None
            }

            e @ (NSEventType::KeyDown | NSEventType::KeyUp) => {
                let keycode = sel!("keyCode");
                let event = event.ptr;
                let keycode: u32 = msg_ret![event keycode];

                let keycode = super::super::keysym::to_keycode(keycode)?;
                let key_event = match e {
                    NSEventType::KeyDown => KeyboardEvent::KeyPress(keycode),
                    NSEventType::KeyUp => KeyboardEvent::KeyRelease(keycode),
                    _ => unreachable!(),
                };

                Some(Event {
                    time: Duration::ZERO,
                    window: WindowHandle(window_id),
                    kind: EventKind::Keyboard(key_event),
                })
            }

            _ => {
                // println!("Type: {}", event_type as usize);
                // println!("Subtype: {}", event_subtype as i16);
                let event = event.ptr;

                // We can't handle undocumented events or we'll be rejected from the app store,
                // so we instead have to hand those to macOS.
                // msg![nsapp send_event sendEvent:event];
                None
            }
        };

        if change_frontmost_window {
            self.set_frontmost_window(window_id);
        }

        event
    }
}
