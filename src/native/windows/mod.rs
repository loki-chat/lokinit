use std::time::Duration;

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
    pub window_handles: Vec<WindowHandle>,
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
            self.window_handles.push(window_handle);
            Ok(window_handle)
        }
    }

    fn poll_event(&mut self) -> Option<Event> {
        unsafe {
            let mut msg: MSG = std::mem::zeroed();
            GetMessageW(&mut msg, NULL as _, 0, 0);

            match msg.message {
                WM_PAINT => {}
                WM_NCMOUSEMOVE => {}
                WM_NCLBUTTONDOWN => {}
                LOKI_WM_SIZE => println!("Something good happened!"),
                LOKI_WM_MOVE => println!("Something moved!"),
                _ => println!("message: {:x}", msg.message),
            }

            TranslateMessage(&msg);
            DispatchMessageW(&msg);
            Some(Event {
                time: Duration::from_millis(1),
                window: WindowHandle(msg.hwnd as usize),
                kind: EventKind::Redraw,
            })
        }
    }

    fn close_window(&mut self, handle: WindowHandle) {
        todo!()
    }

    fn fetch_monitors(&mut self) -> Vec<Monitor> {
        todo!()
    }
}

const LOKI_WM_SIZE: UINT = WM_APP + 0x01;
const LOKI_WM_MOVE: UINT = WM_APP + 0x02;

unsafe extern "system" fn win32_wndproc(
    hwnd: HWND,
    umsg: UINT,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    // Send a corresponding custom event when GetMessageW doesn't retrieve it itself
    let success = match umsg {
        WM_SIZE => PostMessageW(hwnd, LOKI_WM_SIZE, wparam, lparam) != 0,
        WM_MOVE => PostMessageW(hwnd, LOKI_WM_MOVE, wparam, lparam) != 0,
        _ => true,
    };

    if !success {
        // TODO: call GetLastError... maybe...
        eprintln!("Error (call GetLastError)?");
    }

    DefWindowProcW(hwnd, umsg, wparam, lparam)
}
