use std::{
    cell::RefCell,
    collections::{BTreeSet, VecDeque},
    time::Duration,
};

use crate::{
    event::{Event, EventKind},
    lok::{CreateWindowError, LokinitBackend, Monitor},
    window::{ScreenMode, WindowBuilder, WindowHandle},
};

use winapi::{
    shared::{
        minwindef::{DWORD, LPARAM, LRESULT, UINT, WPARAM},
        ntdef::NULL,
        windef::{HWND, RECT},
    },
    um::{libloaderapi::GetModuleHandleW, winuser::*},
};

#[derive(Default)]
pub struct WindowsBackend {
    window_handles: BTreeSet<WindowHandle>,
}

impl LokinitBackend for WindowsBackend {
    fn init() -> Self {
        Self::default()
    }

    fn create_window(&mut self, builder: WindowBuilder) -> Result<WindowHandle, CreateWindowError> {
        let class_name = "LOKINIT\0".encode_utf16().collect::<Vec<u16>>();
        let mut window_name = builder.title.encode_utf16().collect::<Vec<u16>>();
        window_name.push(0);
        let fullscreen = builder.screen_mode == ScreenMode::Fullscreen;
        let resizable = builder.resizable;
        unsafe {
            let mut window_class: WNDCLASSW = std::mem::zeroed();
            window_class.hInstance = GetModuleHandleW(NULL as _);
            window_class.lpszClassName = class_name.as_ptr() as _;
            window_class.lpfnWndProc = Some(win32_wndproc);
            window_class.style = CS_HREDRAW | CS_VREDRAW;
            RegisterClassW(&window_class);

            let win_style: DWORD;
            let win_ex_style: DWORD = WS_EX_APPWINDOW | WS_EX_WINDOWEDGE;
            let mut rect = RECT {
                left: 0,
                top: 0,
                right: 0,
                bottom: 0,
            };
            rect.left = builder.position.x;
            rect.top = builder.position.y;

            if fullscreen {
                win_style = WS_POPUP | WS_SYSMENU | WS_VISIBLE;
                rect.right = GetSystemMetrics(SM_CXSCREEN);
                rect.bottom = GetSystemMetrics(SM_CYSCREEN);
            } else {
                win_style = if resizable {
                    WS_CLIPSIBLINGS
                        | WS_CLIPCHILDREN
                        | WS_CAPTION
                        | WS_SYSMENU
                        | WS_MINIMIZEBOX
                        | WS_MAXIMIZEBOX
                        | WS_SIZEBOX
                } else {
                    WS_CLIPSIBLINGS | WS_CLIPCHILDREN | WS_CAPTION | WS_SYSMENU | WS_MINIMIZEBOX
                };

                rect.right = builder.size.width as i32 + builder.position.x;
                rect.bottom = builder.size.height as i32 + builder.position.y;
            }

            AdjustWindowRectEx(&rect as *const _ as _, win_style, false as _, win_ex_style);
            let win_width = rect.right - rect.left;
            let win_height = rect.bottom - rect.top;

            let hwnd = CreateWindowExW(
                win_ex_style,                // dwExStyle
                class_name.as_ptr(),         // lpClassName
                window_name.as_ptr(),        // lpWindowName
                win_style,                   // dwStyle
                builder.position.x,          // X
                builder.position.y,          // Y
                win_width,                   // nWidth
                win_height,                  // nHeight
                NULL as _,                   // hWndParent
                NULL as _,                   // hMenu
                GetModuleHandleW(NULL as _), // hInstance
                NULL as _,                   // lparam
            );

            ShowWindow(hwnd, SW_SHOW);
            let window_handle = WindowHandle(hwnd as usize);
            self.window_handles.insert(window_handle);
            Ok(window_handle)
        }
    }

    fn poll_event(&mut self) -> Option<Event> {
        loop {
            let n_events = n_events();
            if n_events != 0 {
                // println!("Events in poll_event: {}", n_events);
            }
            if let Some(event) = recv_event() {
                return Some(event);
            }

            unsafe {
                let mut msg: MSG = std::mem::zeroed();
                if PeekMessageW(&mut msg, NULL as _, 0, 0, PM_REMOVE) != 0 {
                    let window = WindowHandle(msg.hwnd as usize);
                    println!("peeking message");

                    if WM_QUIT == msg.message {
                        println!("quitting");
                        return Some(Event {
                            time: Duration::from_millis(1),
                            window,
                            kind: EventKind::CloseRequested,
                        });
                    } else {
                        TranslateMessage(&mut msg as *mut _ as _);
                        DispatchMessageW(&mut msg as *mut _ as _);
                    }
                }
            }

            // println!("Remaining windows: {}", self.window_handles.len());
            if self.window_handles.is_empty() {
                // We quit once all windows have quit
                return None;
            }
        }
    }

    fn close_window(&mut self, handle: WindowHandle) {
        self.window_handles.remove(&handle);
        println!("Remaining windows: {}", self.window_handles.len());
        unsafe { DestroyWindow(handle.0 as _) };
    }

    fn fetch_monitors(&mut self) -> Vec<Monitor> {
        todo!()
    }
}

thread_local! {
    // The event queue! Because Windows is stupid and blocks on some important events
    // (like resizing and moving) because it handles them with internal loops. -_-
    static EVENT_QUEUE: RefCell<VecDeque<Event>> = RefCell::new(VecDeque::new());
}

fn n_events() -> usize {
    EVENT_QUEUE.with(|event_queue| event_queue.borrow().len())
}

fn send_event(event: Event) {
    EVENT_QUEUE.with(move |event_queue| event_queue.borrow_mut().push_back(event))
}

fn recv_event() -> Option<Event> {
    EVENT_QUEUE.with(|event_queue| event_queue.borrow_mut().pop_front())
}

unsafe extern "system" fn win32_wndproc(
    hwnd: HWND,
    umsg: UINT,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    let time = Duration::from_millis(1);
    let window = WindowHandle(hwnd as usize);
    match umsg {
        WM_SIZE => {
            let width = ((lparam >> (isize::BITS / 2)) & 0xffff) as u32;
            let height = (lparam & 0xffff) as u32;
            send_event(Event {
                time,
                window,
                kind: EventKind::Resized(width, height),
            });
        }
        WM_MOVE => {
            let x: i32 = std::mem::transmute(((lparam >> (isize::BITS / 2)) & 0xffff) as u32);
            let y: i32 = std::mem::transmute((lparam & 0xffff) as u32);
            send_event(Event {
                time,
                window,
                kind: EventKind::Moved(x, y),
            });
        }
        WM_QUIT => {
            PostMessageW(hwnd, WM_CLOSE, 0, 0);
        }
        WM_CLOSE => {
            println!("closing from wndproc");
        }
        _ => send_event(Event {
            time,
            window,
            kind: EventKind::Redraw,
        }),
    }
    println!("Events in wndproc: {}", n_events());

    DefWindowProcW(hwnd, umsg, wparam, lparam)
}
