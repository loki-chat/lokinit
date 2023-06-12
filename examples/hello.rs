use lokinit::prelude::*;

fn main() {
    let mut core = LokinitCore::init().unwrap();

    let main_win = core
        .create_window(
            WindowBuilder::new()
                .title("Hello")
                .transparent(false)
                .centered(true)
                .size(1280, 720),
        )
        .unwrap();

        let mut count: usize = 0;
        loop {
            if let Some((window, event)) = core.poll_event() {
                if main_win == window {
                    println!("New main event: {event:?}");
                }
                println!("New non-main event: {event:?}");
                count = 0;
            } else {
                count += 1;
                println!("no new event ({count})");
            }
        }
}
