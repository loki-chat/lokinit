use lokinit::prelude::*;

fn main() {
    let mut core = LokinitCore::init();

    core.create_window(
        WindowBuilder::new()
            .title("Hello")
            .transparent(false)
            .centered(true)
            .size(1280, 720)
    );

    while let Some((_window, event)) = core.poll_event() {
        println!("New event: {event:?}");
    }
}
