use {
    loki_draw::{
        drawer::{Drawer, RectBlueprint},
        rect::Rect,
        OpenglDrawer,
    },
    lokinit::prelude::*,
    std::ffi::CString,
};

fn main() {
    lok::init();

    gl::load_with(|func| {
        let name = CString::new(func).unwrap();
        lok::load_opengl_func(name.as_ptr())
    });

    let window = lok::create_window(
        WindowBuilder::new()
            .title("OpenGL")
            .centered(true)
            .size(600, 400),
    )
    .unwrap();
    println!("Creating surface");
    let surface = window.create_surface(OpenGlConfig::default());
    println!("Making surface active");
    window.make_surface_active(surface);

    let mut drawer = OpenglDrawer::new(600, 400, 1.0);
    draw(&mut drawer, window, surface);

    while let Some(event) = lok::poll_event() {
        match event.kind {
            EventKind::Resized(x, y) => {
                drawer.resize(
                    glam::Vec2 {
                        x: x as _,
                        y: y as _,
                    },
                    1.0,
                );
                draw(&mut drawer, window, surface);
            }
            _ => {}
        }
    }
}

fn draw(drawer: &mut OpenglDrawer, window: WindowHandle, surface: WindowSurface) {
    drawer.begin_frame();
    drawer.clear();
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
    window.flush_surface(surface);
}
