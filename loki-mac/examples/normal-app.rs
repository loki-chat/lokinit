use loki_mac::*;

fn main() {
    let mut nsapp = NSApplication::shared();

    let size = NSRect {
        size: NSSize {
            width: 590.0,
            height: 600.0,
        },
        origin: NSPoint { x: 0.0, y: 0.0 },
    };
    let style = NSWindowStyleMask::default().closable().resizable().titled();
    let mut window = NSWindow::new(size, style);
    window.focus();

    nsapp.run();
}
