use lokinit::prelude::*;

fn main() {
    core::create_window(
        WindowBuilder::new()
            .title("Hello")
            .transparent(false)
            .centered(true)
            .size(1280, 720),
    )
    .unwrap();
    core::create_window(
        WindowBuilder::new()
            .title("Hello")
            .transparent(false)
            .centered(true)
            .size(1280, 720),
    )
    .unwrap();

    loop {
        if let Some(event) = core::poll_event() {
            println!("New event: {:?}", event);
        }
    }

    println!("Quitting!");
}
