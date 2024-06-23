//! macOS OpenGL types.

use super::classes_prelude::*;

#[objrs]
mod ffi {
    use super::*;

    extern "objc" {
        type NSOpenGLView;

        fn alloc() -> NonNull<Self>;
        #[selector = "defaultPixelFormat"]
        fn default_pixel_format() -> ObjcRc<NSOpenGLPixelFormat>;
        #[selector = "initWithFrame:pixelFormat:"]
        fn init(&mut self, frame: NSRect, pixel_format: ObjcRc<NSOpenGLPixelFormat>);
    }

    extern "objc" {
        type NSOpenGLPixelFormat;

        fn alloc() -> NonNull<NSOpenGLPixelFormatInstance>;
        #[selector = "initWithAttributes:"]
        fn init(&mut self, attributes: *const u32);
        #[selector = "numberOfVirtualScreens"]
        fn number_of_virtual_screens(&self) -> i32;
    }
    impl NSOpenGLPixelFormat {
        pub fn new(attributes: &[u32]) -> ObjcRc<Self> {
            let mut this = unsafe { Self::from_raw(Self::alloc()) };
            this.init(attributes as *const [u32] as *const u32);

            this
        }
    }

    extern "objc" {
        type NSOpenGLContext;

        fn alloc() -> NonNull<NSOpenGLContextInstance>;
        #[selector = "initWithFormat:shareContext:"]
        fn init(
            &mut self,
            format: ObjcRc<NSOpenGLPixelFormat>,
            share_context: Option<ObjcRc<NSOpenGLContext>>,
        );
        #[selector = "setView:"]
        fn set_view(&mut self, view: ObjcRc<NSView>);
        #[selector = "makeCurrentContext"]
        fn make_current(&self);
        #[selector = "setFullScreen"]
        fn set_fullscreen(&self);
        fn update(&self);
        #[selector = "flushBuffer"]
        fn flush_buffer(&self);
    }
    impl NSOpenGLContext {
        pub fn new(format: ObjcRc<NSOpenGLPixelFormat>) -> ObjcRc<Self> {
            let mut this = unsafe { Self::from_raw(Self::alloc()) };
            this.init(format, None);

            this
        }
    }
}
pub use ffi::*;
