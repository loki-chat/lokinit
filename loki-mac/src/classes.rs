pub mod core;
pub mod metal;
pub mod opengl;
pub mod windowing;

pub mod all {
    pub use super::{core::*, metal::*, opengl::*, windowing::*};
}

mod classes_prelude {
    pub use {
        super::all::*,
        crate::enums::*,
        objective_rust::prelude::*,
        std::ptr::{self, NonNull},
    };
}
