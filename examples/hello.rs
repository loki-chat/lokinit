use lokinit::prelude::*;

fn main() {
    // hehe
    lok::init();

    lok::create_window(
        WindowBuilder::new()
            .title("Hello")
            .transparent(false)
            .centered(true)
            .size(600, 400),
    )
    .unwrap();

    lok::create_window(
        WindowBuilder::new()
            .title("World")
            .transparent(false)
            .size(400, 600),
    )
    .unwrap();

    while let Some(event) = lok::poll_event() {
        let win = event.window;

        match event.kind {
            EventKind::Mouse(event) => match event {
                MouseEvent::ButtonPress(btn, x, y) => {
                    println!("[{win:?}] Mouse button {btn:?} pressed at ({x}, {y})")
                }
                MouseEvent::ButtonRelease(btn, x, y) => {
                    println!("[{win:?}] Mouse button {btn:?} released at ({x}, {y})")
                }
                MouseEvent::CursorIn(x, y) => {
                    println!("[{win:?}] Cursor entered window at ({x}, {y})")
                }
                MouseEvent::CursorOut(x, y) => {
                    println!("[{win:?}] Cursor exited window at ({x}, {y})")
                }
                // Log spam warning: it's commented for a reason
                // MouseEvent::CursorMove(x, y) => println!("Cursor moved to ({x}, {y})"),
                _ => {}
            }
            EventKind::Keyboard(event) => match event {
                KeyboardEvent::KeyPress(keycode) => println!("[{win:?}] Key {keycode:?} pressed"),
                KeyboardEvent::KeyRelease(keycode) => {
                    println!("[{win:?}] Key {keycode:?} released")
                }
                KeyboardEvent::KeyRepeat(keycode) => println!("[{win:?}] Key {keycode:?} repeated"),
                KeyboardEvent::ImeCommit(commit) => println!("[{win:?}] IME commit -> {commit:?}"),
            }
            EventKind::Resized(width, height) => {
                println!("[{win:?}] Window resized to ({width}, {height})")
            }
            EventKind::Moved(x, y) => {
                println!("[{win:?}] Window moved to ({x}, {y})")
            }
            EventKind::CloseRequested => {
                lok::close_window(win);
                println!("[{win:?}] Closed upon request");
            }
            _ => {}
        }
    }

    println!("Event loop ended, quitting!");

}
