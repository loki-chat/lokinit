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

    while let Some(event) = core::poll_event() {
        if let EventKind::Mouse(event) = event.kind {
            match event {
                MouseEvent::ButtonPress(_, _, _) => println!("Mouse button pressed"),
                MouseEvent::ButtonRelease(_, _, _) => println!("Mouse button released"),
                _ => {}
            }
        }
        println!("Event: {event:?}");
    }

    println!("Quitting!");
}
