use loki_mac::*;

fn main() {
    let mut nsapp = NSApp::shared();

    let size = ffi::NSRect {
        size: ffi::NSSize {
            width: 590.0,
            height: 600.0,
        },
        origin: ffi::NSPoint { x: 0.0, y: 0.0 },
    };
    let style = NSWindowStyleMask::default().closable().resizable().titled();
    let mut window = NSWindow::new(size, style);
    window.make_main();

    nsapp.run();
}
