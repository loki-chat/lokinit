use super::classes_prelude::*;

#[objrs]
mod ffi {
    use super::*;

    extern "ObjC" {
        type MTKView;
    }

    extern "ObjC" {
        type MTLDevice;
    }
    impl Default for MTLDevice {
        fn default() -> Self {
            unsafe { Self::from_raw_no_rc(MTLCreateSystemDefaultDevice()) }
        }
    }

    extern "C" {
        fn MTLCreateSystemDefaultDevice() -> NonNull<MTLDeviceInstance>;
    }
}
pub use ffi::*;
