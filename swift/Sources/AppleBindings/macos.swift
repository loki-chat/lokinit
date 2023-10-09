#if os(macOS)

import AppKit;
import CoreFoundation;

// APP: MacOSApplication = MacOSApplication()
//
// public class MacOSApplication {
//     // 
//     public func observe(_ observer: CFRunLoopObserver?, _ activity: CFRunLoopActivity) {
//         
//     }
// }

public class MacOSWindow: NSWindow, NSWindowDelegate {
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
        
        // Window settings
        self.acceptsMouseMovedEvents = true
        self.title = title
        self.delegate = self
        self.center()

        // Add an observer for the app's run loop
        // let cfRunLoop = NSRunLoop.currentRunLoop.getCFRunLoop()
        //func CFRunLoopAddObserver(CFRunLoop!, CFRunLoopObserver!, CFRunLoopMode!)
        // let cfRunLoop = CFRunLoopGetCurrent()
        let observer = CFRunLoopObserverCreateWithHandler(
            nil,
            CFRunLoopActivity.allActivities.rawValue,
            true,
            0,
            self.observer
        )
        CFRunLoopAddObserver(
            CFRunLoopGetCurrent(),
            observer,
            CFRunLoopMode.commonModes
        )

        // The window's view
        let view = MacOSView.init(size, UInt64(self.windowNumber))
        self.contentView = view
        self.makeFirstResponder(view)

        // Open the window
        self.makeKeyAndOrderFront(nil)
    }
    
    // Observer for the app's run loop (this lets us perform async callbacks)
    public func observer(_ observer: CFRunLoopObserver?, _ activity: CFRunLoopActivity) {

    }

    // Window resize event
    public func windowWillResize(_ sender: NSWindow, to frameSize: NSSize) -> NSSize {
        rust_window_resize_callback(
            UInt(sender.windowNumber),
            UInt32(frameSize.width),
            UInt32(frameSize.height)
        )

        return frameSize
    }
}

public class MacOSView: NSView {
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
        rust_mouse_callback(Int32(event.windowNumber), MouseButton.Left, MouseEvent.Pressed, point.0, point.1)
    }
    override public func rightMouseDown(with event: NSEvent) {
        let point = self.translateMousePoint(event.locationInWindow)
        rust_mouse_callback(Int32(event.windowNumber), MouseButton.Right, MouseEvent.Pressed, point.0, point.1)
    }
    override public func otherMouseDown(with event: NSEvent) {
        let point = self.translateMousePoint(event.locationInWindow)
        rust_mouse_callback(Int32(event.windowNumber), MouseButton.Middle, MouseEvent.Pressed, point.0, point.1)
    }

    // Mouse up events
    override public func mouseUp(with event: NSEvent) {
        let point = self.translateMousePoint(event.locationInWindow)
        rust_mouse_callback(Int32(event.windowNumber), MouseButton.Left, MouseEvent.Released, point.0, point.1)
    }
    override public func rightMouseUp(with event: NSEvent) {
        let point = self.translateMousePoint(event.locationInWindow)
        rust_mouse_callback(Int32(event.windowNumber), MouseButton.Right, MouseEvent.Released, point.0, point.1)
    }
    override public func otherMouseUp(with event: NSEvent) {
        let point = self.translateMousePoint(event.locationInWindow)
        rust_mouse_callback(Int32(event.windowNumber), MouseButton.Middle, MouseEvent.Released, point.0, point.1)
    }

    // Mouse movement events
    override public func mouseMoved(with event: NSEvent) {
        let point = self.translateMousePoint(event.locationInWindow)
        rust_mouse_callback(Int32(event.windowNumber), MouseButton.Left, MouseEvent.Moved, point.0, point.1)
    }
}

#endif
