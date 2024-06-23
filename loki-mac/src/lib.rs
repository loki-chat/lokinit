pub mod classes;
pub mod dynload;
pub mod enums;
pub mod wrappers;

pub mod prelude {
    pub use {
        crate::{classes::all::*, enums::*},
        objective_rust::prelude::*,
    };
}

mod _link {
    use crate::classes::core::NSRunLoopMode;

    // Without these links, AppKit Objective-C classes won't get loaded.
    #[link(name = "AppKit", kind = "framework")]
    #[allow(improper_ctypes)]
    extern "C" {
        /// https://developer.apple.com/documentation/foundation/nsrunloopcommonmodes?language=objc
        pub static NSRunLoopCommonModes: NSRunLoopMode;
        /// https://developer.apple.com/documentation/foundation/nsdefaultrunloopmode?language=objc
        pub static NSDefaultRunLoopMode: NSRunLoopMode;
        /// https://developer.apple.com/documentation/appkit/nseventtrackingrunloopmode?language=objc
        pub static NSEventTrackingRunLoopMode: NSRunLoopMode;
        /// https://developer.apple.com/documentation/appkit/nsmodalpanelrunloopmode?language=objc
        pub static NSModalPanelRunLoopMode: NSRunLoopMode;
    }
}
