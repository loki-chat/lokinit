use lokinit::prelude::*;

fn main() {
    let mut core = LokinitCore::init().unwrap();

    core.create_window(
        WindowBuilder::new()
            .title("Hello")
            .transparent(false)
            .centered(true)
            .size(1280, 720),
    )
    .unwrap();

    while let Some(event) = core.poll_event() {
        println!("New event: {:?}", event);
    }

    println!("Quitting!");
}
