use loki_mac::{ffi::NSDate, *};

fn main() {
    let mut nsapp = NSApp::shared();
    nsapp.activate();
    nsapp.finish_launching();

    let size = ffi::NSRect {
        size: ffi::NSSize {
            width: 590.0,
            height: 600.0,
        },
        origin: ffi::NSPoint { x: 0.0, y: 0.0 },
    };
    let style = NSWindowStyleMask::default()
        .closable()
        .miniaturizable()
        .resizable()
        .titled();
    let mut window = NSWindow::new(size, style);
    window.make_main();

    loop {
        let event = nsapp.next_event(
            NSEventMask::Any,
            NSDate::distant_future(),
            NSRunLoopMode::Default,
            true,
        );

        println!("Event: {:?}", event.event_type());
        match event.event_type() {
            NSEventType::AppKitDefined
            | NSEventType::ApplicationDefined
            | NSEventType::SystemDefined => {
                println!("Event subtype: {:?}", event.event_subtype());
            }
            _ => {}
        }
    }
}
