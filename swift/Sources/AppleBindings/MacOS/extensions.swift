#if os(macOS)

import Foundation
import AppKit

// Used to get the currently frontmost window
extension NSApplication {
    public var frontWindow: NSWindow {
        self.orderedWindows[0]
    }
}

// Adds an append method, since `.appending` isn't available on older macOS versions
extension URL {
    public func append(_ path: String) -> URL {
        return URL(string: path, relativeTo: self)!
    }
}

// Adds convenience initializers so the events aren't so annoying to make
extension LokEvent {
    init(_ event: LokEventType, _ window: UInt) {
        self.init(
            type: event,
            data1: 0,
            data2: 0,
            data3: 0,
            window: window
        )
    }

    init(_ event: LokEventType, _ data1: Int32, _ window: UInt) {
        self.init(
            type: event,
            data1: data1,
            data2: 0,
            data3: 0,
            window: window
        )
    }

    init(_ event: LokEventType, _ data1: Int32, _ data2: Int32, _ window: UInt) {
        self.init(
            type: event,
            data1: data1,
            data2: data2,
            data3: 0,
            window: window
        )
    }

    init(_ event: LokEventType, _ data1: Int32, _ data2: Int32, _ data3: Int32, _ window: UInt) {
        self.init(
            type: event,
            data1: data1,
            data2: data2,
            data3: data3,
            window: window
        )
    }
}

#endif
