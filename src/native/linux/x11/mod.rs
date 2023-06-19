#![allow(unused)]

use std::borrow::Cow;
use std::collections::{BTreeMap, VecDeque};
use std::ffi::{c_int, c_void, CString};
use std::ptr::{null, null_mut, NonNull};
use std::time::{Duration, Instant, SystemTime};

use crate::event::{Event, EventKind, KeyboardEvent, MouseButton, MouseEvent};
use crate::keycode::KeyCode;
use crate::library;
use crate::native::linux::x11::ffi::{LibX11, XEvent};
use crate::prelude::{WindowBuilder, WindowHandle, WindowPos, WindowSize};

use self::ffi::{
    et, xclass, xcw, xevent_mask, xim, xn, Status, XConfigureEvent, XDisplay, XErrorEvent,
    XKeyEvent, XPoint, XSetWindowAttributes, XWindow, XID, X_BUFFER_OVERFLOW, _XIC, _XIM,
};

use super::locale::{setlocale, LC_CTYPE};
use super::LoadingError;

mod ffi;
mod keysym;

pub type NativeCoreError = X11NativeCoreError;
pub type CreateWindowError = X11CreateWindowError;

#[derive(Clone, Debug)]
pub enum X11NativeCoreError {
    LibLoading(LoadingError),
    CannotOpenDisplay,
    CannotOpenInputMethod,
}

impl From<LoadingError> for X11NativeCoreError {
    fn from(value: LoadingError) -> Self {
        Self::LibLoading(value)
    }
}

#[derive(Clone, Debug)]
pub enum X11CreateWindowError {
    CannotOpenInputContext,
}

pub struct X11NativeWindow {
    window: XWindow,
    position: WindowPos,
    size: WindowSize,
    wm_delete_message: u64,
    xic: NonNull<_XIC>,
}

pub struct LokinitCore {
    x11: LibX11,
    root: XWindow,
    xim: NonNull<_XIM>,
    display: NonNull<XDisplay>,
    windows: BTreeMap<WindowHandle, X11NativeWindow>,
    event_queue: VecDeque<Event>,
    prev_key: Option<KeyCode>,
    is_composing: bool,
    str_buffer: Vec<u8>,
    n_windows: u32,
}

impl LokinitCore {
    pub fn init() -> Result<Self, X11NativeCoreError> {
        unsafe {
            let x11 = LibX11::new()?;

            (x11.XSetErrorHandler)(Some(x11_error_handler));

            // Prepare locale for IME to work properly
            let empty_string = CString::new("").unwrap();
            setlocale(LC_CTYPE, empty_string.as_ptr());
            (x11.XSetLocaleModifiers)(empty_string.as_ptr());

            (x11.XInitThreads)();
            (x11.XrmInitialize)();

            // Open the default X11 display
            let display = (x11.XOpenDisplay)(null());
            let display = NonNull::new(display).ok_or(X11NativeCoreError::CannotOpenDisplay)?;

            let root = (x11.XDefaultRootWindow)(display.as_ptr());

            // https://linux.die.net/man/3/xkbsetdetectableautorepeat
            // TLDR: Xkb allows clients to request detectable auto-repeat.
            // If a client requests and the server supports DetectableAutoRepeat,
            // Xkb generates KeyRelease events only when the key is physically
            // released. If DetectableAutoRepeat is not supported or has not been
            // requested, the server synthesizes a KeyRelease event for each
            // repeating KeyPress event it generates.
            (x11.XkbSetDetectableAutoRepeat)(display.as_ptr(), true as _, null_mut());

            // Initialize IME
            let xim = (x11.XOpenIM)(display.as_ptr(), null_mut(), null(), null());
            let xim = NonNull::new(xim).ok_or(X11NativeCoreError::CannotOpenInputMethod)?;

            (x11.XFlush)(display.as_ptr());

            Ok(Self {
                x11,
                root,
                xim,
                display,
                windows: BTreeMap::new(),
                event_queue: VecDeque::new(),
                prev_key: None,
                is_composing: false,
                str_buffer: vec![0; 16],
                n_windows: 0,
            })
        }
    }

    pub fn n_windows(&self) -> u32 {
        self.n_windows
    }

    pub fn create_window(
        &mut self,
        builder: WindowBuilder,
    ) -> Result<WindowHandle, X11CreateWindowError> {
        unsafe {
            let mut attributes = XSetWindowAttributes {
                event_mask: xevent_mask::EXPOSURE
                    | xevent_mask::STRUCTURE_NOTIFY
                    | xevent_mask::VISIBILITY_CHANGE
                    | xevent_mask::KEY_PRESS
                    | xevent_mask::KEY_RELEASE
                    | xevent_mask::BUTTON_PRESS
                    | xevent_mask::BUTTON_RELEASE
                    | xevent_mask::POINTER_MOTION
                    | xevent_mask::FOCUS_CHANGE
                    | xevent_mask::ENTER_WINDOW
                    | xevent_mask::LEAVE_WINDOW,
                ..Default::default()
            };

            let window_attributes = xcw::EVENT_MASK;

            let window = (self.x11.XCreateWindow)(
                self.display.as_ptr(),
                self.root,
                builder.position.x,
                builder.position.y,
                builder.size.width,
                builder.size.height,
                0,
                0,
                xclass::INPUT_OUTPUT,
                null_mut(),
                window_attributes,
                &mut attributes,
            );

            // register interest in the delete window message
            let atom_name = CString::new("WM_DELETE_WINDOW").unwrap();
            let wm_delete_message =
                (self.x11.XInternAtom)(self.display.as_ptr(), atom_name.as_ptr(), 0);
            (self.x11.XSetWMProtocols)(self.display.as_ptr(), window, &wm_delete_message, 1);

            // spawn window on the screen
            (self.x11.XMapWindow)(self.display.as_ptr(), window);

            // create IME context for this window
            let xic = (self.x11.XCreateIC)(
                self.xim.as_ptr(),
                xn::INPUT_STYLE,
                xim::PREEDIT_NOTHING | xim::STATUS_NOTHING,
                xn::CLIENT_WINDOW,
                window,
                null_mut::<c_void>(),
            );
            let xic = NonNull::new(xic).ok_or(X11CreateWindowError::CannotOpenInputContext)?;

            // select IME and position it
            (self.x11.XSetICFocus)(xic.as_ptr());
            place_ime(&self.x11, xic, XPoint::new(0, 0));

            (self.x11.XFlush)(self.display.as_ptr());

            let handle = window.into_window_handle();

            // save window in core
            self.windows.insert(
                handle,
                X11NativeWindow {
                    window,
                    position: builder.position,
                    size: builder.size,
                    wm_delete_message,
                    xic,
                },
            );

            self.n_windows += 1;
            Ok(handle)
        }
    }

    pub fn close_window(&mut self, handle: WindowHandle) {
        let window = self.windows.remove(&handle).unwrap();
        unsafe { (self.x11.XUnmapWindow)(self.display.as_ptr(), window.window) };
        self.n_windows -= 1;
    }

    pub fn poll_event(&mut self) -> Option<Event> {
        if self.n_windows == 0 {
            // No events to poll if there are no windows
            return None;
        }

        if let Some(win_event) = self.event_queue.pop_front() {
            return Some(win_event);
        }

        unsafe {
            // make sure we always poll a Lokinit event by skipping unknown ones
            let mut win_event = None;
            while win_event.is_none() {
                let count = (self.x11.XPending)(self.display.as_ptr());

                // get all pending events or wait for the next one
                for _ in 0..count.max(1) {
                    let mut xevent = XEvent { type_id: 0 };
                    (self.x11.XNextEvent)(self.display.as_ptr(), &mut xevent);

                    // Apparently, this forwards the event to the IME and returns whether the event was consumed.
                    // I know, weird. The name of the function is even weirder.
                    if (self.x11.XFilterEvent)(&mut xevent, XWindow::NONE) > 0 {
                        continue;
                    }

                    self.process_event(&xevent);
                }

                (self.x11.XFlush)(self.display.as_ptr());

                win_event = self.event_queue.pop_front();
            }

            win_event
        }
    }

    /// Transform an `XEvent` into one or more Lokinit `Event`s and push them into the event queue.
    unsafe fn process_event(&mut self, xevent: &XEvent) {
        match xevent.type_id {
            et::KEY_PRESS | et::KEY_RELEASE => {
                let xevent = xevent.xkey;
                let time = Duration::from_millis(xevent.time);

                let handle = xevent.window.into_window_handle();
                let window = self.windows.get(&handle).unwrap();

                let (keysym, text) =
                    utf8_lookup_string(&self.x11, &mut self.str_buffer, window, xevent);

                if let Some(keycode) = keysym::to_keycode(keysym.0 as u32) {
                    let kb_event = match (xevent.type_id, self.prev_key) {
                        (et::KEY_PRESS, Some(k)) if k == keycode => {
                            KeyboardEvent::KeyRepeat(keycode)
                        }
                        (et::KEY_PRESS, _) => {
                            self.prev_key = Some(keycode);
                            KeyboardEvent::KeyPress(keycode)
                        }
                        (et::KEY_RELEASE, Some(k)) if k == keycode => {
                            self.prev_key = None;
                            KeyboardEvent::KeyRelease(keycode)
                        }
                        (et::KEY_RELEASE, _) => KeyboardEvent::KeyRelease(keycode),
                        _ => unreachable!(),
                    };

                    // Send IME commit only on a non-repeated key press
                    let do_ime = matches!(kb_event, KeyboardEvent::KeyPress(_));

                    self.event_queue.push_back(Event {
                        time,
                        window: handle,
                        kind: EventKind::Keyboard(kb_event),
                    });

                    if !do_ime {
                        return;
                    }
                };

                // Handle IME commit
                if let Some(text) = text {
                    place_ime(&self.x11, window.xic, XPoint::new(0, 0));
                    self.event_queue.push_back(Event {
                        time,
                        window: handle,
                        kind: EventKind::Keyboard(KeyboardEvent::ImeCommit(text.into_owned())),
                    });
                }
            }

            et::BUTTON_PRESS | et::BUTTON_RELEASE => {
                let xevent = xevent.xbutton;
                let time = Duration::from_millis(xevent.time);

                let handle = xevent.window.into_window_handle();
                let window = self.windows.get(&handle).unwrap();

                let mouse_button = match xevent.button {
                    1 => MouseButton::Left,
                    2 => MouseButton::Middle,
                    3 => MouseButton::Right,
                    b => MouseButton::Other(b as u16),
                };

                let kind = if xevent.type_id == et::BUTTON_PRESS {
                    EventKind::Mouse(MouseEvent::ButtonPress(mouse_button, xevent.x, xevent.y))
                } else {
                    EventKind::Mouse(MouseEvent::ButtonRelease(mouse_button, xevent.x, xevent.y))
                };

                self.event_queue.push_back(Event {
                    time,
                    window: handle,
                    kind,
                });
            }

            et::CONFIGURE_NOTIFY => {
                let xevent = xevent.xconfigure;
                let time = Duration::from_millis(0);

                let handle = xevent.window.into_window_handle();
                let window = self.windows.get_mut(&handle).unwrap();

                let xwin_pos = WindowPos::new(xevent.x, xevent.y);
                if xwin_pos != window.position {
                    window.position = xwin_pos;

                    self.event_queue.push_back(Event {
                        time,
                        window: handle,
                        kind: EventKind::Moved(window.position.x, window.position.y),
                    });
                }

                let xwin_size = WindowSize::new(xevent.width as u32, xevent.height as u32);
                if xwin_size != window.size {
                    window.size = xwin_size;

                    self.event_queue.push_back(Event {
                        time,
                        window: handle,
                        kind: EventKind::Resized(window.size.width, window.size.height),
                    });
                }
            }

            et::DESTROY_NOTIFY => {
                let xevent = xevent.xdestroywindow;
                let time = Duration::from_millis(0);

                let handle = xevent.window.into_window_handle();
                let window = self.windows.get(&handle).unwrap();

                self.event_queue.push_back(Event {
                    time,
                    window: handle,
                    kind: EventKind::Destroyed,
                });
            }

            et::MOTION_NOTIFY => {
                let xevent = xevent.xmotion;
                let time = Duration::from_millis(xevent.time);

                let (&handle, _window) = (self.windows.iter())
                    .find(|(_h, w)| w.window == xevent.window)
                    .unwrap();

                self.event_queue.push_back(Event {
                    time,
                    window: handle,
                    kind: EventKind::Mouse(MouseEvent::CursorMove(xevent.x, xevent.y)),
                });
            }

            et::ENTER_NOTIFY | et::LEAVE_NOTIFY => {
                let xevent = xevent.xcrossing;
                let time = Duration::from_millis(xevent.time);

                let handle = xevent.window.into_window_handle();
                let Some(window) = self.windows.get(&handle) else {
                    // A LEAVE_NOTIFY event will often be emitted right after a window has been closed.
                    return;
                };

                let kind = if xevent.type_id == et::ENTER_NOTIFY {
                    EventKind::Mouse(MouseEvent::CursorIn(xevent.x, xevent.y))
                } else {
                    EventKind::Mouse(MouseEvent::CursorOut(xevent.x, xevent.y))
                };

                self.event_queue.push_back(Event {
                    time,
                    window: handle,
                    kind,
                });
            }

            et::CLIENT_MESSAGE => {
                let xevent = xevent.xclient;
                let time = Duration::from_millis(0);

                let handle = xevent.window.into_window_handle();
                let window = self.windows.get(&handle).unwrap();

                // if client requests to quit
                if xevent.data.l[0] as u64 == window.wm_delete_message {
                    self.event_queue.push_back(Event {
                        time,
                        window: handle,
                        kind: EventKind::CloseRequested,
                    })
                }
            }

            _ => (),
        }
    }

    fn get_window(&self, window: WindowHandle) -> &X11NativeWindow {
        self.windows.get(&window).unwrap()
    }

    pub fn window_pos(&self, window: WindowHandle) -> WindowPos {
        self.get_window(window).position
    }

    pub fn window_size(&self, window: WindowHandle) -> WindowSize {
        self.get_window(window).size
    }
}

unsafe extern "C" fn x11_error_handler(_display: *mut XDisplay, event: *mut XErrorEvent) -> i32 {
    if let Some(event) = event.as_ref() {
        println!("X11: error (code {})", event.error_code);
    } else {
        println!("X11 called the error handler without an error event or a display, somehow");
    }

    0
}

unsafe fn place_ime(x11: &LibX11, xic: NonNull<_XIC>, place: XPoint) {
    let preedit_attr =
        (x11.XVaCreateNestedList)(0, xn::SPOT_LOCATION, &place, null_mut::<c_void>());

    (x11.XSetICValues)(
        xic.as_ptr(),
        xn::PREEDIT_ATTRIBUTES,
        preedit_attr,
        null_mut::<c_void>(),
    );

    (x11.XFree)(preedit_attr as *mut c_void);
}

unsafe fn utf8_lookup_string<'a>(
    x11: &LibX11,
    str_buffer: &'a mut Vec<u8>,
    window: &X11NativeWindow,
    mut xpress: XKeyEvent,
) -> (XID, Option<Cow<'a, str>>) {
    let mut keysym = XID(0);
    let mut status: Status = 0;

    xpress.type_id = et::KEY_PRESS;

    let mut char_count = (x11.Xutf8LookupString)(
        window.xic.as_ptr(),
        &xpress,
        str_buffer.as_mut_ptr() as *mut _,
        str_buffer.len() as c_int,
        &mut keysym,
        &mut status,
    );

    // reallocating lookup string buffer if it wasn't big enough
    if status == X_BUFFER_OVERFLOW {
        *str_buffer = vec![0; char_count as usize + 1];

        char_count = (x11.Xutf8LookupString)(
            window.xic.as_ptr(),
            &xpress,
            str_buffer.as_mut_ptr() as *mut _,
            str_buffer.len() as c_int,
            &mut keysym,
            &mut status,
        );
    }

    let text = (char_count > 0).then(|| {
        let zeroidx = char_count as usize;
        str_buffer[zeroidx] = 0;

        String::from_utf8_lossy(&str_buffer[..zeroidx])
    });

    (keysym, text)
}
