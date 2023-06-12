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

    let mut noevent_count: usize = 0;
    loop {
        if let Some(event) = core.poll_event() {
            println!("New event: {:?}", event);
            noevent_count = 0;
        } else {
            noevent_count += 1;
            println!("no new event ({})", noevent_count);
        }
    }
}
