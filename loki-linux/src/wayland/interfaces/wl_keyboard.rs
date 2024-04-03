use std::ffi::{c_int, c_void};

use crate::wayland::{LibWaylandClient, WlArray, WL_MARSHAL_FLAG_DESTROY};

use super::wl_surface::WlSurface;

#[repr(C)]
pub struct WlKeyboard([u8; 0]);

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WlKeyboardKeymapFormat {
    /// no keymap; client must understand how to interpret the raw keycode
    NoKeymap = 0,
    /// libxkbcommon compatible, null-terminated string; to determine the xkb keycode, clients must add 8 to the key event keycode
    XkbV1 = 1,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WlKeyboardKeyState {
    /// key is not pressed
    Released = 0,
    /// key is pressed
    Pressed = 1,
}

pub const WL_KEYBOARD_RELEASE: u32 = 0;
pub const WL_KEYBOARD_KEYMAP_SINCE_VERSION: u32 = 1;
pub const WL_KEYBOARD_ENTER_SINCE_VERSION: u32 = 1;
pub const WL_KEYBOARD_LEAVE_SINCE_VERSION: u32 = 1;
pub const WL_KEYBOARD_KEY_SINCE_VERSION: u32 = 1;
pub const WL_KEYBOARD_MODIFIERS_SINCE_VERSION: u32 = 1;
pub const WL_KEYBOARD_REPEAT_INFO_SINCE_VERSION: u32 = 4;
pub const WL_KEYBOARD_RELEASE_SINCE_VERSION: u32 = 3;

pub struct WlKeyboardListener {
    pub keymap: Option<
        unsafe extern "C" fn(
            data: *mut c_void,
            wl_keyboard: *mut WlKeyboard,
            format: u32,
            fd: i32,
            size: u32,
        ),
    >,

    pub enter: Option<
        unsafe extern "C" fn(
            data: *mut c_void,
            wl_keyboard: *mut WlKeyboard,
            serial: u32,
            surface: *mut WlSurface,
            keys: *mut WlArray,
        ),
    >,

    pub leave: Option<
        unsafe extern "C" fn(
            data: *mut c_void,
            wl_keyboard: *mut WlKeyboard,
            serial: u32,
            surface: *mut WlSurface,
        ),
    >,

    pub key: Option<
        unsafe extern "C" fn(
            data: *mut c_void,
            wl_keyboard: *mut WlKeyboard,
            serial: u32,
            time: u32,
            key: u32,
            state: u32,
        ),
    >,

    pub modifiers: Option<
        unsafe extern "C" fn(
            data: *mut c_void,
            wl_keyboard: *mut WlKeyboard,
            serial: u32,
            mods_depressed: u32,
            mods_latched: u32,
            mods_locked: u32,
            group: u32,
        ),
    >,

    pub repeat_info: Option<
        unsafe extern "C" fn(
            data: *mut c_void,
            wl_keyboard: *mut WlKeyboard,
            rate: i32,
            delay: i32,
        ),
    >,
}

#[allow(clippy::missing_safety_doc)]
impl LibWaylandClient {
    pub unsafe fn wl_keyboard_add_listener(
        &self,
        wl_keyboard: *mut WlKeyboard,
        listener: *const WlKeyboardListener,
        data: *mut c_void,
    ) -> c_int {
        (self.wl_proxy_add_listener)(wl_keyboard as _, listener as _, data)
    }

    pub unsafe fn wl_keyboard_set_user_data(
        &self,
        wl_keyboard: *mut WlKeyboard,
        user_data: *mut c_void,
    ) {
        (self.wl_proxy_set_user_data)(wl_keyboard as _, user_data)
    }

    pub unsafe fn wl_keyboard_get_user_data(&self, wl_keyboard: *mut WlKeyboard) -> *mut c_void {
        (self.wl_proxy_get_user_data)(wl_keyboard as _)
    }

    pub unsafe fn wl_keyboard_get_version(&self, wl_keyboard: *mut WlKeyboard) -> u32 {
        (self.wl_proxy_get_version)(wl_keyboard as _)
    }

    pub unsafe fn wl_keyboard_destroy(&self, wl_keyboard: *mut WlKeyboard) {
        (self.wl_proxy_destroy)(wl_keyboard as _)
    }

    pub unsafe fn wl_keyboard_release(&self, wl_keyboard: *mut WlKeyboard) {
        (self.wl_proxy_marshal_flags)(
            wl_keyboard as _,
            WL_KEYBOARD_RELEASE,
            std::ptr::null(),
            (self.wl_proxy_get_version)(wl_keyboard as _),
            WL_MARSHAL_FLAG_DESTROY,
        );
    }
}
