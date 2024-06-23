use {
    loki_draw::{
        drawer::{Drawer, RectBlueprint},
        rect::Rect,
        OpenglDrawer,
    },
    lokinit::prelude::*,
    std::{ffi::CString, ptr},
};

fn main() {
    println!("Loading Lokinit");
    lok::init();

    println!("Loading GL");
    gl::load_with(|func| {
        println!("  Loading GL function {func}");
        let name = CString::new(func).unwrap();

        let ptr = lok::load_opengl_func(name.as_ptr()).unwrap_or(ptr::null_mut());
        if ptr.is_null() {
            println!("      WARNING: Was null");
        }

        ptr
    });

    println!("Creating window");
    let window = lok::create_window(
        WindowBuilder::new()
            .title("Hello")
            .transparent(false)
            .centered(true)
            .size(600, 400)
            .position(200, 400)
            .resizable(true),
    )
    .unwrap();
    println!("Creating surface");
    let surface = window.create_surface(OpenGlConfig::default());
    println!("Making surface active");
    surface.make_active();

    println!("Making drawer");
    let mut drawer = OpenglDrawer::new(600, 400, 1.0);
    drawer.clear();
    drawer.begin_frame();
    drawer.draw_rect(&RectBlueprint {
        rect: Rect {
            x: 0.0,
            y: 0.0,
            w: 100.0,
            h: 100.0,
        },
        color: 0x2a2939,
        border_color: 0xff84c6,
        border_width: 4.,
        corner_radius: 10.,
        borders: [true, true, true, true],
        alpha: 1.,
    });
    drawer.end_frame();
    surface.flush();

    while let Some(event) = lok::poll_event() {}
}
