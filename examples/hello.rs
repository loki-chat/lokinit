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
        match event.kind {
            EventKind::Mouse(event) => match event {
                MouseEvent::ButtonPress(btn, x, y) => {
                    println!("Mouse button {btn:?} pressed at ({x}, {y})")
                }
                MouseEvent::ButtonRelease(btn, x, y) => {
                    println!("Mouse button {btn:?} released at ({x}, {y})")
                }
                MouseEvent::CursorIn(x, y) => println!("Cursor entered window at ({x}, {y})"),
                MouseEvent::CursorOut(x, y) => println!("Cursor exited window at ({x}, {y})"),
                // Log spam warning: it's commented for a reason
                // MouseEvent::CursorMove(x, y) => println!("Cursor moved to ({x}, {y})"),
                _ => {}
            },
            EventKind::Resized(width, height) => println!("Window resized to ({width}, {height})"),
            _ => {}
        }
    }

    println!("Event loop ended, quitting!");
}
