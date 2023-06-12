#![allow(unused)]

use std::collections::{BTreeMap, VecDeque};
use std::ffi::{c_int, c_void, CString};
use std::ptr::{null, null_mut, NonNull};

use crate::event::{Event, KeyboardEvent};
use crate::library;
use crate::native::linux::x11::ffi::{LibX11, XEvent};
use crate::prelude::{WindowBuilder, WindowHandle, WindowPos, WindowSize};

use self::ffi::{
    et, xclass, xcw, xevent_mask, xim, xn, Status, XDisplay, XErrorEvent, XPoint,
    XSetWindowAttributes, XWindow, XID, X_BUFFER_OVERFLOW, _XIC, _XIM,
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

pub struct X11NativeCore {
    x11: LibX11,
    root: XWindow,
    xim: NonNull<_XIM>,
    display: NonNull<XDisplay>,
    windows: BTreeMap<WindowHandle, X11NativeWindow>,
    windex: usize,
    event_queue: VecDeque<(WindowHandle, Event)>,
    is_composing: bool,
    str_buffer: Vec<u8>,
}

impl X11NativeCore {
    pub unsafe fn init() -> Result<Self, X11NativeCoreError> {
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
            windex: 0,
            event_queue: VecDeque::new(),
            is_composing: false,
            str_buffer: vec![0; 16],
        })
    }

    pub unsafe fn create_window(
        &mut self,
        builder: WindowBuilder,
    ) -> Result<WindowHandle, X11CreateWindowError> {
        self.windex += 1;

        let handle = WindowHandle(self.windex);

        let mut attributes = XSetWindowAttributes {
            event_mask: xevent_mask::EXPOSURE
                | xevent_mask::STRUCTURE_NOTIFY
                | xevent_mask::VISIBILITY_CHANGE
                | xevent_mask::KEY_PRESS
                | xevent_mask::KEY_RELEASE
                | xevent_mask::KEYMAP_STATE
                | xevent_mask::BUTTON_PRESS
                | xevent_mask::BUTTON_RELEASE
                | xevent_mask::POINTER_MOTION
                | xevent_mask::FOCUS_CHANGE,
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
        (self.x11.XSelectInput)(self.display.as_ptr(), window, xevent_mask::KEY_PRESS);
        place_ime(&self.x11, xic, XPoint::new(0, 0));

        (self.x11.XFlush)(self.display.as_ptr());

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

        Ok(handle)
    }

    pub unsafe fn poll_event(&mut self) -> Option<(WindowHandle, Event)> {
        // TODO: use this quit var somehow
        let mut quit = false;
        
        if let Some(win_event) = self.event_queue.pop_front() {
            return Some(win_event);
        }

        let count = (self.x11.XPending)(self.display.as_ptr());
        for _ in 0..count {
            let mut xevent = XEvent { type_id: 0 };
            (self.x11.XNextEvent)(self.display.as_ptr(), &mut xevent);

            // Apparently, this forwards the event to the IME and returns whether the event was consumed.
            // I know, weird. The name of the function is even weirder.
            if (self.x11.XFilterEvent)(&mut xevent, XWindow::NONE) > 0 {
                continue;
            }

            if let Some(handle) = self.process_event(&xevent) {
                let window = self.get_window(handle);
                quit |= xevent.xclient.data.l[0] as u64 == window.wm_delete_message;
            }
        }
        (self.x11.XFlush)(self.display.as_ptr());
        
        self.event_queue.pop_front()
    }

    unsafe fn process_event(&mut self, xevent: &XEvent) -> Option<WindowHandle> {
        match xevent.type_id {
            et::KEY_PRESS | et::KEY_RELEASE => {
                let mut keysym = XID(0);
                let mut status: Status = 0;

                // TODO: having to traverse the tree to get the window is dumb, find another way to do it directly
                let (&handle, window) = (self.windows.iter())
                    .find(|(_h, w)| w.window == xevent.xkey.window)
                    .unwrap();

                if xevent.type_id == et::KEY_PRESS {
                    // Handle IME commit

                    let mut char_count = (self.x11.Xutf8LookupString)(
                        window.xic.as_ptr(),
                        &xevent.xkey,
                        self.str_buffer.as_mut_ptr() as *mut _,
                        self.str_buffer.len() as c_int,
                        &mut keysym,
                        &mut status,
                    );

                    // reallocating lookup string buffer if it wasn't big enough
                    if status == X_BUFFER_OVERFLOW {
                        self.str_buffer = vec![0; char_count as usize + 1];

                        char_count = (self.x11.Xutf8LookupString)(
                            window.xic.as_ptr(),
                            &xevent.xkey,
                            self.str_buffer.as_mut_ptr() as *mut _,
                            self.str_buffer.len() as c_int,
                            &mut keysym,
                            &mut status,
                        );
                    }

                    if char_count > 0 {
                        place_ime(&self.x11, window.xic, XPoint::new(0, 0));
                        let zeroidx = char_count as usize;
                        self.str_buffer[zeroidx] = 0;

                        let text = String::from_utf8_lossy(&self.str_buffer[..zeroidx]).into_owned();
                        let event = Event::Keyboard(KeyboardEvent::ImeCommit(text));
                        self.event_queue.push_back((handle, event));
                    }
                }

                let Some(keycode) = keysym::to_keycode(keysym.0 as u32) else {
                    return Some(handle);
                };

                let event = if xevent.type_id == et::KEY_PRESS {
                    Event::Keyboard(KeyboardEvent::KeyPress(keycode))
                } else {
                    Event::Keyboard(KeyboardEvent::KeyRelease(keycode))
                };
                self.event_queue.push_back((handle, event));

                Some(handle)
            }
            _ => {
                self.event_queue.push_back((WindowHandle(0), Event::Unknown));
                None
            },
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
