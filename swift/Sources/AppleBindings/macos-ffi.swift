#if os(macOS)

import Foundation
import AppKit

public var EventBuffer: Array<LokEvent> = Array()
var LastKeySym: Int32? = nil

@_cdecl("setup")
func ffiSetup() {
    // Init NSApplication
    let NSApp = NSApplication.shared
    NSApp.setActivationPolicy(.regular)
    NSApp.activate(ignoringOtherApps: true)
    NSApp.finishLaunching()
}

@_cdecl("create_window")
func ffiCreateWindow(x: Int, y: Int, width: Int, height: Int, centered: Bool, title: UnsafePointer<CChar>) -> UInt64 {
    let title = String(cString: title)
    let size = NSRect(x: x, y: y, width: width, height: height)
    let window = BSWindow(size, centered, title)
    return UInt64(window.windowNumber)
}

@_cdecl("update")
func ffiUpdate() -> LokEvent { 
    while true {
        if NSApp.windows.count == 0 {
            return LokEvent(.AppQuit, 0)
        }

        if EventBuffer.first != nil {
            return EventBuffer.removeFirst()
        }

        let event = NSApp.nextEvent(
            matching: NSEvent.EventTypeMask.any,
            until: Date.distantFuture,
            inMode: RunLoop.Mode.default,
            dequeue: true
        )!

        switch event.type {
        case .appKitDefined:
            switch event.subtype {
            // So far as I can tell, these events use private APIs, so we have to use sendEvent
            case .applicationActivated:
                NSApp.sendEvent(event)
            case .applicationDeactivated:
                NSApp.sendEvent(event)
            case .screenChanged:
                fatalError("screenChanged event not yet handled")
            case .windowExposed:
                fatalError("windowExposed event not yet implemented")
            case .windowMoved:
                let window = event.window!
                window.sendEvent(event)
                return LokEvent(
                    .WindowMoved,
                    Int32(window.frame.origin.x),
                    Int32(window.frame.origin.y),
                    UInt(window.windowNumber)
                )
            default:
                continue
            }
        case .systemDefined:
            switch event.subtype {
            case .powerOff:
                fatalError("powerOff event not yet handled")
            default:
                continue
            }
        case .leftMouseDown:
            let window = event.window! as! BSWindow
            let handled = window.leftButtonDownHandler(event)
            if !handled {
                let mousePos = window.getMouseLocation()
                return LokEvent(.MouseDownLeft, Int32(mousePos.x), Int32(mousePos.y), UInt(window.windowNumber))
            }
        case .leftMouseDragged:
            let window = event.window! as! BSWindow
            let forwardEvent = window.leftButtonDraggedHandler(event)
            if let event = forwardEvent {
                return event
            }
        case .leftMouseUp:
            let window = event.window! as! BSWindow
            let handled = window.leftButtonUpHandler(event)
            if !handled {
                let mousePos = window.getMouseLocation()
                return LokEvent(.MouseUpLeft, Int32(mousePos.x), Int32(mousePos.y), UInt(window.windowNumber))
            }
        case .rightMouseDown:
            let window = event.window! as! BSWindow
            let mousePos = window.getMouseLocation()
            return LokEvent(.MouseDownRight, Int32(mousePos.x), Int32(mousePos.y), UInt(window.windowNumber))
        case .rightMouseDragged:
            let window = event.window! as! BSWindow
            let mousePos = window.getMouseLocation()
            return LokEvent(.MouseMoved, Int32(mousePos.x), Int32(mousePos.y), UInt(window.windowNumber))
        case .rightMouseUp:
            let window = event.window! as! BSWindow
            let mousePos = window.getMouseLocation()
            return LokEvent(.MouseUpRight, Int32(mousePos.x), Int32(mousePos.y), UInt(window.windowNumber))
        case .otherMouseDown:
            let window = event.window! as! BSWindow
            let mousePos = window.getMouseLocation()
            let mouseBtn = event.buttonNumber

            if mouseBtn == 2 {
                return LokEvent(.MouseDownMiddle, Int32(mousePos.x), Int32(mousePos.y), UInt(window.windowNumber))
            } else {
                return LokEvent(.MouseDownOther, Int32(mousePos.x), Int32(mousePos.y), Int32(mouseBtn), UInt(window.windowNumber))
            }
        case .otherMouseDragged:
            let window = event.window! as! BSWindow
            let mousePos = window.getMouseLocation()

            return LokEvent(.MouseMoved, Int32(mousePos.x), Int32(mousePos.y), UInt(window.windowNumber))
        case .otherMouseUp:
            let window = event.window! as! BSWindow
            let mousePos = window.getMouseLocation()
            let mouseBtn = event.buttonNumber

            if mouseBtn == 2 {
                return LokEvent(.MouseUpMiddle, Int32(mousePos.x), Int32(mousePos.y), UInt(window.windowNumber))
            } else {
                return LokEvent(.MouseUpOther, Int32(mousePos.x), Int32(mousePos.y), Int32(event.buttonNumber), UInt(window.windowNumber))
            }
        case .mouseMoved:
            let window = NSApp.frontWindow as! BSWindow
            let handled = window.mouseMovedHandler(event)
            if !handled {
                let mousePos = window.getMouseLocation()
                return LokEvent(.MouseMoved, Int32(mousePos.x), Int32(mousePos.y), UInt(window.windowNumber))
            }
        case .keyDown:
            let keySym = Int32(event.keyCode)

            if keySym == LastKeySym {
                return LokEvent(.KeyRepeated, keySym, UInt(event.windowNumber))
            } else {
                LastKeySym = keySym
                return LokEvent(.KeyPressed, keySym, UInt(event.windowNumber))
            }
        case .keyUp:
            let keySym = Int32(event.keyCode)

            if keySym == LastKeySym {
                LastKeySym = nil
            }

            return LokEvent(.KeyReleased, keySym, UInt(event.windowNumber))
        default:
            continue
        }
    }
}

#endif
