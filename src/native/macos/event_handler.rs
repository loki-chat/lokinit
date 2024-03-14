use {
    super::{window::Point, *},
    crate::event::*,
};

impl MacosBackend {
    pub fn handle_raw_event(&mut self, raw_event: NSEvent) {
        let event_window = WindowHandle(raw_event.window_number() as usize);
        match raw_event.event_type() {
            NSEventType::AppKitDefined => match raw_event.event_subtype() {
                NSEventSubtype::WindowMoved => {
                    self.nsapp.send_event(raw_event);
                    let window = self.windows.get_mut(&event_window.0).unwrap();
                    window.recalculate_borders();
                    let origin = window.frame().origin;

                    self.event_queue.push_back(Event {
                        time: Duration::ZERO,
                        window: event_window,
                        kind: EventKind::Moved(origin.x as i32, origin.y as i32),
                    });
                }
                NSEventSubtype::ApplicationActivated | NSEventSubtype::ApplicationDeactivated => {
                    self.nsapp.send_event(raw_event);
                    self.windows
                        .get_mut(self.frontmost_window.as_ref().unwrap())
                        .unwrap()
                        .make_main();
                }
                NSEventSubtype::Undocumented(_) | NSEventSubtype::WindowExposed => {
                    self.nsapp.send_event(raw_event);
                }
                _ => {}
            },
            NSEventType::SystemDefined => {
                if let NSEventSubtype::Undocumented(_) = raw_event.event_subtype() {
                    self.nsapp.send_event(raw_event)
                }
            }

            NSEventType::MouseMoved
            | NSEventType::LeftMouseDragged
            | NSEventType::RightMouseDragged
            | NSEventType::OtherMouseDragged => {
                self.nsapp.send_event(raw_event);

                let NSPoint { x, y } = NSEvent::mouse_location();

                if let Some(resize_border) = self.resize_direction {
                    self.windows
                        .get_mut(&event_window.0)
                        .unwrap()
                        .resize_border(resize_border, Point::Screen(x as i32, y as i32))
                }

                self.event_queue.push_back(Event {
                    // TODO: Time
                    time: Duration::ZERO,
                    window: event_window,
                    kind: EventKind::Mouse(MouseEvent::CursorMove(x as i32, y as i32)),
                });
            }
            NSEventType::MouseEntered => {
                let NSPoint { x, y } = self.windows.get(&event_window.0).unwrap().mouse_location();
                self.event_queue.push_back(Event {
                    // TODO: Time
                    time: Duration::ZERO,
                    window: event_window,
                    kind: EventKind::Mouse(MouseEvent::CursorIn(x as i32, y as i32)),
                });
            }
            NSEventType::MouseExited => {
                let NSPoint { x, y } = self.windows.get(&event_window.0).unwrap().mouse_location();
                self.event_queue.push_back(Event {
                    // TODO: Time
                    time: Duration::ZERO,
                    window: event_window,
                    kind: EventKind::Mouse(MouseEvent::CursorOut(x as i32, y as i32)),
                });
            }

            NSEventType::LeftMouseDown => {
                let window = self.windows.get_mut(&event_window.0).unwrap();
                let mouse_loc = window.mouse_location();
                self.frontmost_window = Some(event_window.0);

                self.resize_direction =
                    window.point_in_border(Point::Window(mouse_loc.x as i32, mouse_loc.y as i32));
                if self.resize_direction.is_none() {
                    self.nsapp.send_event(raw_event);
                }

                self.event_queue.push_back(Event {
                    // TODO: Time
                    time: Duration::ZERO,
                    window: event_window,
                    kind: EventKind::Mouse(MouseEvent::ButtonPress(
                        MouseButton::Left,
                        mouse_loc.x as i32,
                        mouse_loc.y as i32,
                    )),
                });
            }
            NSEventType::LeftMouseUp => {
                self.resize_direction = None;
                let NSPoint { x, y } = self.windows.get(&event_window.0).unwrap().mouse_location();
                self.event_queue.push_back(Event {
                    // TODO: Time
                    time: Duration::ZERO,
                    window: event_window,
                    kind: EventKind::Mouse(MouseEvent::ButtonRelease(
                        MouseButton::Left,
                        x as i32,
                        y as i32,
                    )),
                });
            }
            NSEventType::RightMouseDown => {
                let NSPoint { x, y } = self.windows.get(&event_window.0).unwrap().mouse_location();
                self.event_queue.push_back(Event {
                    // TODO: Time
                    time: Duration::ZERO,
                    window: event_window,
                    kind: EventKind::Mouse(MouseEvent::ButtonPress(
                        MouseButton::Right,
                        x as i32,
                        y as i32,
                    )),
                });
            }
            NSEventType::RightMouseUp => {
                let NSPoint { x, y } = self.windows.get(&event_window.0).unwrap().mouse_location();
                self.event_queue.push_back(Event {
                    // TODO: Time
                    time: Duration::ZERO,
                    window: event_window,
                    kind: EventKind::Mouse(MouseEvent::ButtonRelease(
                        MouseButton::Right,
                        x as i32,
                        y as i32,
                    )),
                });
            }
            NSEventType::OtherMouseDown => {
                let NSPoint { x, y } = self.windows.get(&event_window.0).unwrap().mouse_location();
                let button_number = raw_event.mouse_button_number();
                let button = if button_number == 2 {
                    MouseButton::Middle
                } else {
                    MouseButton::Other(button_number as _)
                };

                self.event_queue.push_back(Event {
                    // TODO: Time
                    time: Duration::ZERO,
                    window: event_window,
                    kind: EventKind::Mouse(MouseEvent::ButtonPress(button, x as i32, y as i32)),
                });
            }
            NSEventType::OtherMouseUp => {
                let NSPoint { x, y } = self.windows.get(&event_window.0).unwrap().mouse_location();
                let button_number = raw_event.mouse_button_number();
                let button = if button_number == 2 {
                    MouseButton::Middle
                } else {
                    MouseButton::Other(button_number as _)
                };

                self.event_queue.push_back(Event {
                    // TODO: Time
                    time: Duration::ZERO,
                    window: event_window,
                    kind: EventKind::Mouse(MouseEvent::ButtonRelease(button, x as i32, y as i32)),
                });
            }

            NSEventType::KeyDown => {
                let key = super::keysym::to_keycode(raw_event.key_code()).unwrap();
                let repeat: bool = raw_event.is_repeat().into();

                let event_type = if repeat {
                    KeyboardEvent::KeyRepeat(key)
                } else {
                    KeyboardEvent::KeyPress(key)
                };
                self.event_queue.push_back(Event {
                    // TODO: Time
                    time: Duration::ZERO,
                    window: event_window,
                    kind: EventKind::Keyboard(event_type),
                });
            }
            NSEventType::KeyUp => {
                let key = super::keysym::to_keycode(raw_event.key_code()).unwrap();

                self.event_queue.push_back(Event {
                    // TODO: Time
                    time: Duration::ZERO,
                    window: event_window,
                    kind: EventKind::Keyboard(KeyboardEvent::KeyRelease(key)),
                });
            }
            NSEventType::FlagsChanged => {
                let Some(keycode) = super::keysym::to_keycode(raw_event.key_code()) else {
                    println!(
                        "Lokinit macOS: Warning: Unknown key flag `{}`",
                        raw_event.key_code()
                    );
                    return;
                };
                match self.active_modifiers.contains(&keycode) {
                    true => {
                        self.active_modifiers.remove(&keycode);
                        self.event_queue.push_back(Event {
                            // TODO: Time
                            time: Duration::ZERO,
                            window: event_window,
                            kind: EventKind::Keyboard(KeyboardEvent::KeyRelease(keycode)),
                        });
                    }
                    false => {
                        self.active_modifiers.insert(keycode);
                        self.event_queue.push_back(Event {
                            // TODO: Time
                            time: Duration::ZERO,
                            window: event_window,
                            kind: EventKind::Keyboard(KeyboardEvent::KeyPress(keycode)),
                        });
                    }
                }
            }

            _ => {}
        }
    }
}
