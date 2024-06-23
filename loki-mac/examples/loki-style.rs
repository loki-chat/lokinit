use loki_mac::*;

fn main() {
    let mut nsapp = NSApplication::shared();
    nsapp.activate();
    nsapp.finish_launching();

    let size = NSRect {
        size: NSSize {
            width: 590.0,
            height: 600.0,
        },
        origin: NSPoint { x: 0.0, y: 0.0 },
    };
    let style = NSWindowStyleMask::default()
        .closable()
        .miniaturizable()
        .resizable()
        .titled();
    let mut window = NSWindow::new(size, style);
    window.focus();

    loop {
        let event = nsapp.next_event(
            NSEventMask::Any,
            NSDate::distant_future(),
            NSRunLoopMode::default(),
            true.into(),
        );

        if let Some(event) = event {
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
}
