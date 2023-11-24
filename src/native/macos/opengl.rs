use super::objc::{NSPoint, NSRect, NSSize};

use {
    super::{
        objc::{ffi::*, macros::*},
        MacosBackend,
    },
    crate::prelude::*,
    core::ffi::c_void,
};

pub struct OpenGLVtables {
    /// https://developer.apple.com/documentation/appkit/nsopenglview?language=objc
    opengl_view: *mut c_void,
    /// https://developer.apple.com/documentation/appkit/nsopenglview/1414944-defaultpixelformat?language=objc
    default_pixel_format_sel: *mut c_void,
    /// https://developer.apple.com/documentation/appkit/nsopenglcontext?language=objc
    opengl_context: *mut c_void,
    /// https://developer.apple.com/documentation/appkit/nsopenglcontext/1436178-initwithformat?language=objc
    context_init_sel: *mut c_void,
    /// https://developer.apple.com/documentation/appkit/nsopenglcontext/1436212-makecurrentcontext?language=objc
    make_current_context_sel: *mut c_void,
    /// https://developer.apple.com/documentation/appkit/nsopenglcontext/1436106-clearcurrentcontext?language=objc
    clear_current_context_sel: *mut c_void,
    /// https://developer.apple.com/documentation/appkit/nsopenglcontext/1436204-view?language=objc
    set_view_sel: *mut c_void,
    /// https://developer.apple.com/documentation/appkit/nswindow/1419160-contentview?language=objc
    content_view_sel: *mut c_void,
}
impl Default for OpenGLVtables {
    fn default() -> Self {
        Self {
            opengl_view: class!(NSOpenGLView),
            default_pixel_format_sel: sel!("defaultPixelFormat"),
            opengl_context: class!(NSOpenGLContext),
            context_init_sel: sel!("initWithFormat:shareContext:"),
            make_current_context_sel: sel!("makeCurrentContext"),
            clear_current_context_sel: sel!("clearCurrentContext"),
            set_view_sel: sel!("setView:"),
            content_view_sel: sel!("contentView"),
        }
    }
}

/// Functions for working with OpenGL in Lokinit. Use this to create OpenGL surfaces
/// and contexts to draw on.
pub struct GLDisplay {
    /// A handle to the OpenGL framework/module.
    hmodule: *mut c_void,
    /// Pointers to Objective-C OpenGL classes.
    vtables: OpenGLVtables,
    /// The OpenGL context.
    context: *mut c_void,
}
impl Default for GLDisplay {
    fn default() -> Self {
        let hmodule = unsafe {
            dlopen(
                b"/System/Library/Frameworks/OpenGL.framework/Versions/Current/OpenGL\0".as_ptr()
                    as _,
                1,
            )
        };
        if hmodule.is_null() {
            panic!("Lokinit failed to load macOS' OpenGL framework.");
        }

        let vtables = OpenGLVtables::default();

        let (opengl_view, ctx, default_pixel_format, init, make_main) = (
            vtables.opengl_view,
            vtables.opengl_context,
            vtables.default_pixel_format_sel,
            vtables.context_init_sel,
            vtables.make_current_context_sel,
        );
        let pixel_format: *mut c_void = msg_ret![opengl_view default_pixel_format];
        if pixel_format.is_null() {
            panic!("Bruh");
        }
        let alloc = sel!("alloc");
        let ctx: *mut c_void = msg_ret![ctx alloc];
        let null: *mut c_void = std::ptr::null_mut();
        let context: *mut c_void = msg_ret![ctx init initWithFormat:pixel_format shareContext:null];
        msg![context make_main];

        Self {
            hmodule,
            vtables,
            context,
        }
    }
}
impl Drop for GLDisplay {
    fn drop(&mut self) {
        let (ctx, drop) = (
            self.vtables.opengl_context,
            self.vtables.clear_current_context_sel,
        );
        msg![ctx drop];
    }
}

fn str_to_nsstring(string: &str) -> *const c_void {
    let nsstring = class!(NSString);
    let alloc = sel!("alloc");
    let init = sel!("initWithBytes:length:encoding:");

    let nsstring = msg_ret![nsstring alloc];
    let len = string.len();
    let bytes = string.as_ptr();
    msg_ret![nsstring init initWithBytes:bytes length:len encoding:4usize]
}

// TODO: Standardise this API and make it a trait for all platforms to impl
impl GLDisplay {
    pub fn load_proc_address(&self, proc: &str) -> *const c_void {
        let bundle =
            unsafe { CFBundleGetBundleWithIdentifier(str_to_nsstring("com.apple.opengl")) };
        // if bundle.is_null() {
        //     panic!("eee");
        // }
        let result = unsafe { CFBundleGetFunctionPointerForName(bundle, str_to_nsstring(proc)) };
        // let result = unsafe { dlsym(self.hmodule, (proc.to_string() + "\0").as_ptr() as _) };
        // if result.is_null() {
        //     panic!("yyyy")
        // }
        result
    }

    pub fn create_window_surface(
        &self,
        window: WindowHandle,
        backend: &MacosBackend,
    ) -> GLWindowSurface {
        let window = backend
            .windows
            .get(&window.0)
            .expect("Tried to create window surface for window that doesn't exist")
            .ptr;
        // let content_view = self.vtables.content_view_sel;
        // let nsview = msg_ret![window content_view];
        let nsview = class!(NSView);
        let init = sel!("initWithFrame:");
        let alloc = sel!("alloc");
        let nsview = msg_ret![nsview alloc];
        let rect = NSRect {
            origin: NSPoint { x: 0., y: 0. },
            size: NSSize {
                width: 600.,
                height: 400.,
            },
        };
        let nsview = msg_ret![nsview init initWithFrame:rect];
        let set_content_view = sel!("setContentView:");
        msg![window set_content_view setContentView:nsview];

        GLWindowSurface { nsview }
    }

    pub fn draw_on(&self, surface: &GLWindowSurface) {
        let set_view = self.vtables.set_view_sel;
        let ctx = self.context;
        let view = surface.nsview;

        msg![ctx set_view setView:view];
    }
}

pub struct GLWindowSurface {
    nsview: *mut c_void,
}
