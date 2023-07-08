use crate::{lok::LokinitBackend, prelude::WindowHandle, window::ScreenMode};

use winapi::{
    shared::{
        hidusage::{HID_USAGE_GENERIC_MOUSE, HID_USAGE_PAGE_GENERIC},
        minwindef::{DWORD, HIWORD, LOWORD, LPARAM, LRESULT, UINT, WPARAM},
        ntdef::NULL,
        windef::{HCURSOR, HDC, HICON, HWND, POINT, RECT},
        windowsx::{GET_X_LPARAM, GET_Y_LPARAM},
    },
    um::{
        libloaderapi::{GetModuleHandleW, GetProcAddress},
        shellscalingapi::*,
        wingdi::*,
        winuser::*,
    },
};

pub struct WindowsBackend;

impl LokinitBackend for WindowsBackend {
    fn init() -> Self {
        Self
    }

    fn create_window(&mut self, builder: crate::prelude::WindowBuilder) -> Result<crate::prelude::WindowHandle, crate::lok::CreateWindowError> {
        let class_name = "LOKINIT\0".encode_utf16().collect::<Vec<u16>>();
        let window_name = builder.title.encode_utf16().collect::<Vec<u16>>();
        let fullscreen = builder.screen_mode == ScreenMode::Fullscreen;
        let resizable = builder.resizable;
        unsafe {
            let mut window_class: WNDCLASSW = std::mem::zeroed();
            window_class.hInstance = GetModuleHandleW(NULL as _);
            window_class.lpszClassName = class_name.as_ptr() as _;
            window_class.lpfnWndProc = Some(win32_wndproc);
            RegisterClassW(&window_class);

            let win_style: DWORD;
            let win_ex_style: DWORD = WS_EX_APPWINDOW | WS_EX_WINDOWEDGE;
            let mut rect = RECT {
                left: 0,
                top: 0,
                right: 0,
                bottom: 0,
            };

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

                rect.right = builder.size.width as i32;
                rect.bottom = builder.size.height as i32;
            }
            
            AdjustWindowRectEx(&rect as *const _ as _, win_style, false as _, win_ex_style);
            let win_width = rect.right - rect.left;
            let win_height = rect.bottom - rect.top;
            
            let window_handle = CreateWindowExW(
                win_ex_style,                // dwExStyle
                class_name.as_ptr(),         // lpClassName
                window_name.as_ptr(),        // lpWindowName
                win_style,                   // dwStyle
                CW_USEDEFAULT,               // X
                CW_USEDEFAULT,               // Y
                win_width,                   // nWidth
                win_height,                  // nHeight
                NULL as _,                   // hWndParent
                NULL as _,                   // hMenu
                GetModuleHandleW(NULL as _), // hInstance
                NULL as _,                   // lparam
            );

            Ok(WindowHandle(window_handle as usize))
        }
    }

    fn poll_event(&mut self) -> Option<crate::event::Event> {
        todo!()
    }

    fn close_window(&mut self, handle: crate::prelude::WindowHandle) {
        todo!()
    }

    fn fetch_monitors(&mut self) -> Vec<crate::prelude::Monitor> {
       todo!() 
    }
}

unsafe extern "system" fn win32_wndproc(
    hwnd: HWND,
    umsg: UINT,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    return DefWindowProcW(hwnd, umsg, wparam, lparam);
}
