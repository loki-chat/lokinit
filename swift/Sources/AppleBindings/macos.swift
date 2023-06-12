#if os(macOS)

import AppKit;

@_cdecl("setup")
func setup() {
    let nsApp = NSApplication.shared
    nsApp.setActivationPolicy(NSApplication.ActivationPolicy.regular)
    nsApp.activate(ignoringOtherApps: true)
    nsApp.finishLaunching()
}

@_cdecl("create_window")
func createWindow(width: Int64, height: Int64, title: UnsafePointer<CChar>) -> UInt64 {
    let title = String.init(cString: title)
    let size = NSRect.init(x: 0, y: 0, width: Int(width), height: Int(height))
    let window = MacOSWindow.init(size, title)
    return UInt64(window.windowNumber)
}

@_cdecl("next_event")
func nextEvent() {
    while true {
        let event = NSApp.nextEvent(
            matching: NSEvent.EventTypeMask.any,
            until: nil,
            inMode: RunLoop.Mode.default,
            dequeue: true
        )
        if event == nil {
            break
        }
        NSApp.sendEvent(event!)
    }
    for window in NSApp.windows {
        print("Redrawing window \(window.windowNumber)")
        window.contentView!.needsDisplay = true
        window.orderFront(nil)
        window.makeKeyAndOrderFront(nil)
    }
}

public class MacOSWindow: NSWindow {
    static let masks = 
        NSWindow.StyleMask.titled.rawValue | 
        NSWindow.StyleMask.closable.rawValue |
        NSWindow.StyleMask.miniaturizable.rawValue |
        NSWindow.StyleMask.resizable.rawValue
        
    init(_ size: NSRect, _ title: String) {
        super.init(
            contentRect: size,
            styleMask: NSWindow.StyleMask.init(rawValue: Self.masks),
            backing: NSWindow.BackingStoreType.buffered,
            defer: false
        )

        self.acceptsMouseMovedEvents = true
        self.title = title
        self.center()

        let view = MacOSView.init(size, UInt64(self.windowNumber))
        self.contentView = view
        self.makeFirstResponder(view)
    }
}

public class MacOSView: NSView {
    // The window ID this view corresponds to in Rust
    let id: UInt64

    init(_ frame: NSRect, _ id: UInt64) {
        self.id = id
        super.init(frame: frame)
    }

    required init?(coder: NSCoder) {
        fatalError("init(coder:) has not been implemented")
    }

    // Allows the user to interact with elements on the window,
    // even if it isn't focused, and focus at the same time
    override public func acceptsFirstMouse(for event: NSEvent?) -> Bool {
        return true
    }

    // The points macOS gives us for click events aren't in the View's
    // local coordinate system. They aren't scaled for DPI, and their Y
    // coordinate is inverted. This method adjusts points to correct that.
    func translateMousePoint(_ point: NSPoint) -> (Float64, Float64) {
        let scaled_point = self.convertToBacking(point)
        let y = Float64(self.bounds.height) - Float64(scaled_point.y) - 1.0
        return (Float64(scaled_point.x), y)
    }

    // Mouse down events
    override public func mouseDown(with event: NSEvent) {
        let point = self.translateMousePoint(event.locationInWindow)
        rust_mouse_callback(MouseButton.Left, MouseEvent.Down, point.0, point.1)
    }
    override public func rightMouseDown(with event: NSEvent) {
        let point = self.translateMousePoint(event.locationInWindow)
        rust_mouse_callback(MouseButton.Right, MouseEvent.Down, point.0, point.1)
    }
    override public func otherMouseDown(with event: NSEvent) {
        let point = self.translateMousePoint(event.locationInWindow)
        rust_mouse_callback(MouseButton.Middle, MouseEvent.Down, point.0, point.1)
    }

    // Mouse up events
    override public func mouseUp(with event: NSEvent) {
        let point = self.translateMousePoint(event.locationInWindow)
        rust_mouse_callback(MouseButton.Left, MouseEvent.Up, point.0, point.1)
    }
    override public func rightMouseUp(with event: NSEvent) {
        let point = self.translateMousePoint(event.locationInWindow)
        rust_mouse_callback(MouseButton.Right, MouseEvent.Up, point.0, point.1)
    }
    override public func otherMouseUp(with event: NSEvent) {
        let point = self.translateMousePoint(event.locationInWindow)
        rust_mouse_callback(MouseButton.Middle, MouseEvent.Up, point.0, point.1)
    }

    // Mouse movement events
    override public func mouseMoved(with event: NSEvent) {
        let point = self.translateMousePoint(event.locationInWindow)
        rust_mouse_callback(MouseButton.Left, MouseEvent.Moved, point.0, point.1)
    }
}

#endif