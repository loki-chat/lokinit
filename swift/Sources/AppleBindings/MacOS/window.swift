#if os(macOS)

import Foundation
import AppKit

// All of the sides of a window, and its corners
public enum WindowBorderLocation {
    case Left
    case Right
    case Top
    case Bottom
    case TopLeft
    case TopRight
    case BottomLeft
    case BottomRight
}

// A customized NSWindow, with sensible defaults and helper functions/variables
public class BSWindow: NSWindow {
    // Window style masks to make it resizable and have a title bar
    static let masks = NSWindow.StyleMask.init(rawValue:
        NSWindow.StyleMask.titled.rawValue |
        NSWindow.StyleMask.closable.rawValue |
        NSWindow.StyleMask.miniaturizable.rawValue |
        NSWindow.StyleMask.resizable.rawValue
    )
    // The size of the box that holds the "traffic light" close/minimise/maximise buttons
    static let titlebarButtonBox = NSRect(
        x: 7,
        y: 6,
        width: 54,
        height: 16
    )
    // The minimum size a window can be
    static let minimumWindowSize = CGSize(
        width: 50,
        height: 50
    )
    // Sizes used for detecting if the cursor is over a window border 
    static let windowResizeCornerHitboxSize = 15.0
    static let windowResizeCornerHitboxOffset = (windowResizeCornerHitboxSize + 1.0) / 2.0
    static let windowResizeSideHitboxSize = 7.0
    static let windowResizeSideHitboxOffset = (windowResizeSideHitboxSize + 1.0) / 2.0
    
    // Which part of the window is being resized right now, if it's being resized
    var resizeBorder: WindowBorderLocation? = nil
    // If the cursor is a non-default cursor
    var nonDefaultCursor = false
    //  If the mouse was in the window (tracks mouse entered/left events)
    var mouseWasInWindow = false
    
    init(_ size: NSRect, _ centered: Bool, _ title: String) {
        print("Making new window")
        
        super.init(
            contentRect: size,
            styleMask: Self.masks,
            backing: NSWindow.BackingStoreType.buffered,
            defer: false
        )
        
        self.title = title
        
        // Show the window, and make it the primary window
        self.makeKeyAndOrderFront(nil)
        self.makeMain()

        if centered {
            self.center()
        }

        self.disableCursorRects()
    }
    
    // Mouse press down handler for windows
    // Returns true if the event was handled, false if it wasn't
    func leftButtonDownHandler(_ event: NSEvent) -> Bool {
        if !self.isMainWindow {
            // print("Making self main window due to click")
            self.focus()
        }
        
        if let border = self.checkMouseInBorder() {
            self.resizeBorder = border
            return true
        }
        
        return false
    }
    // Mouse dragged handler for windows
    // Returns an event that should be forwarded to Lokinit
    func leftButtonDraggedHandler(_ event: NSEvent) -> LokEvent? {
        let mousePos = self.mouseLocationOutsideOfEventStream

        // Resize the window if we're currently resizing
        if let border = self.resizeBorder {
            let rect: CGRect
            
            switch border {
            case .Top:
                rect = CGRect(
                    x: self.frame.origin.x,
                    y: self.frame.origin.y,
                    width: self.frame.width,
                    height: mousePos.y
                )
            case .Bottom:
                rect = CGRect(
                    x: self.frame.origin.x,
                    y: self.frame.origin.y + mousePos.y,
                    width: self.frame.width,
                    height: self.frame.height - mousePos.y
                )
            case .Left:
                rect = CGRect(
                    x: self.frame.origin.x + mousePos.x,
                    y: self.frame.origin.y,
                    width: self.frame.width - mousePos.x,
                    height: self.frame.height
                )
            case .Right:
                rect = CGRect(
                    x: self.frame.origin.x,
                    y: self.frame.origin.y,
                    width: mousePos.x,
                    height: self.frame.height
                )
            case .TopLeft:
                rect = CGRect(
                    x: self.frame.origin.x + mousePos.x,
                    y: self.frame.origin.y,
                    width: self.frame.width - mousePos.x,
                    height: mousePos.y
                )
            case .TopRight:
                rect = CGRect(
                    x: self.frame.origin.x,
                    y: self.frame.origin.y,
                    width: mousePos.x,
                    height: mousePos.y
                )
            case .BottomLeft:
                rect = CGRect(
                    x: self.frame.origin.x + mousePos.x,
                    y: self.frame.origin.y + mousePos.y,
                    width: self.frame.width - mousePos.x,
                    height: self.frame.height - mousePos.y
                )
            case .BottomRight:
                rect = CGRect(
                    x: self.frame.origin.x,
                    y: self.frame.origin.y + mousePos.y,
                    width: mousePos.x,
                    height: self.frame.height - mousePos.y
                )
            }
            
            // Make sure the resized window has a valid size; it might break the window otherwise
            if rect.size.width > Self.minimumWindowSize.width && rect.size.height > Self.minimumWindowSize.height {
                self.setFrame(rect, display: true, animate: false)
                
                return LokEvent(.WindowResized, Int32(rect.width), Int32(rect.height), UInt(event.window!.windowNumber))
            }
        }
        
        return LokEvent(.MouseMoved, Int32(mousePos.x), Int32(mousePos.y), UInt(event.window!.windowNumber))
    }
    // Mouse release handler for windows
    // Returns true if the event was handled, false if it wasn't
    func leftButtonUpHandler(_ event: NSEvent) -> Bool {
        if self.resizeBorder != nil {
            self.resizeBorder = nil
            
            return true
        }
        
        let mousePos = self.getMouseLocation()
        for btn in self.windowButtons() {
            if btn.frame.contains(mousePos) {
                btn.isHighlighted = true
                btn.performClick(nil)

                return true
            }
        }
        
        return false
    }
    // Mouse movement handler for windows
    // Returns true if the event was handled, false if it wasn't
    func mouseMovedHandler(_ event: NSEvent) -> Bool {
        let mousePos = self.getMouseLocation()

        // Check if the mouse entered or left the window
        if self.checkMouseInWindow() && !self.mouseWasInWindow {
            self.mouseWasInWindow = true
            EventBuffer.append(LokEvent(
                .MouseEntered,
                Int32(mousePos.x),
                Int32(mousePos.y),
                UInt(self.windowNumber)
            ))
        } else if self.checkMouseOutsideWindow() && self.mouseWasInWindow {
            self.mouseWasInWindow = false
            EventBuffer.append(LokEvent(
                .MouseExited,
                Int32(mousePos.x),
                Int32(mousePos.y),
                UInt(self.windowNumber)
            ))
        }

        // Check if cursor is over one of the window borders
        let mouseInBorder = self.checkMouseInBorder()
        if let border = mouseInBorder {
            switch border {
            case .Top, .Bottom:
                self.setCursor(BSCursor.windowResizeNorthSouth)
            case .Left, .Right:
                self.setCursor(BSCursor.windowResizeEastWest)
            case .TopLeft, .BottomRight:
                self.setCursor(BSCursor.windowResizeNorthWestSouthEast)
            case .TopRight, .BottomLeft:
                self.setCursor(BSCursor.windowResizeNorthEastSouthWest)
            }
            return true
        } else if self.nonDefaultCursor {
            self.setCursor(NSCursor.arrow, false)
        }
        
        // Check if cursor is over one of the titlebar buttons
        if BSWindow.titlebarButtonBox.contains(mousePos) {
            for btn in self.windowButtons() {
                btn.isHighlighted = true
            }
            
            return true
        } else {
            for btn in self.windowButtons() {
                btn.isHighlighted = false
            }
        }
        
        return false
    }
    
    // Returns an array of all the stoplight buttons
    func windowButtons() -> [NSButton] {
        return [
            self.standardWindowButton(ButtonType.closeButton)!,
            self.standardWindowButton(ButtonType.miniaturizeButton)!,
            self.standardWindowButton(ButtonType.zoomButton)!,
        ]
    }
    
    // The mouse points apple hands us have an inverted Y-coordinate.
    // This corrects the points so they're actually useable
    func correctWindowPoint(_ point: NSPoint) -> NSPoint {
        return NSPoint(
            x: point.x,
            y: self.frame.height - point.y - 1.0
        )
    }
    
    // Gets the mouse location in the window and corrects its location
    func getMouseLocation() -> NSPoint {
        return self.correctWindowPoint(self.mouseLocationOutsideOfEventStream)
    }
  
    // Checks if the mouse is out of the window
    func checkMouseOutsideWindow() -> Bool {
        let window = CGRect(
            x: -Self.windowResizeSideHitboxOffset,
            y: -Self.windowResizeSideHitboxOffset,
            width: self.frame.width + (Self.windowResizeSideHitboxOffset * 2),
            height: self.frame.height + (Self.windowResizeSideHitboxOffset * 2)
        )

        return !window.contains(self.getMouseLocation())
    }
    // Checks if the mouse is in the window
    func checkMouseInWindow() -> Bool {
        let innerWindow = CGRect(
            x: Self.windowResizeSideHitboxOffset,
            y: Self.windowResizeSideHitboxOffset,
            width: self.frame.width - (Self.windowResizeSideHitboxOffset * 2),
            height: self.frame.height - (Self.windowResizeSideHitboxOffset * 2)
        )

        return innerWindow.contains(self.getMouseLocation())
    }
    // Checks if the mouse is in any of the window borders
    func checkMouseInBorder() -> WindowBorderLocation? {
        let mouseLocation = NSEvent.mouseLocation
        
        if self.checkMouseInWindow() {
            return nil
        }
        
        //            print("Mouse moved to \(mouseLocation) || Window frame: \(frame)")
        
        // Calculate boxes for the top, bottom, left, and right edges of the window
        let top = CGRect(
            x: self.frame.minX,
            y: self.frame.maxY - Self.windowResizeSideHitboxOffset,
            width: self.frame.width,
            height: Self.windowResizeSideHitboxSize
            )
        let bottom = CGRect(
            x: self.frame.minX,
            y: self.frame.minY - Self.windowResizeSideHitboxOffset,
            width: self.frame.width,
            height: Self.windowResizeSideHitboxSize
        )
        let left = CGRect(
            x: self.frame.minX - Self.windowResizeSideHitboxOffset,
            y: self.frame.minY,
            width: Self.windowResizeSideHitboxSize,
            height: self.frame.height
        )
        let right = CGRect(
            x: self.frame.maxX - Self.windowResizeSideHitboxOffset,
            y: self.frame.minY,
            width: Self.windowResizeSideHitboxSize,
            height: self.frame.height
        )
        
        // Calculate boxes for the corners of the window
        let topLeft = CGRect(
            x: self.frame.minX - Self.windowResizeCornerHitboxOffset,
            y: self.frame.maxY - Self.windowResizeCornerHitboxOffset,
            width: Self.windowResizeCornerHitboxSize,
            height: Self.windowResizeCornerHitboxSize
        )
        let topRight = CGRect(
            x: self.frame.maxX - Self.windowResizeCornerHitboxOffset,
            y: self.frame.maxY - Self.windowResizeCornerHitboxOffset,
            width: Self.windowResizeCornerHitboxSize,
            height: Self.windowResizeCornerHitboxSize
        )
        let bottomLeft = CGRect(
            x: self.frame.minX - Self.windowResizeCornerHitboxOffset,
            y: self.frame.minY - Self.windowResizeCornerHitboxOffset,
            width: Self.windowResizeCornerHitboxSize,
            height: Self.windowResizeCornerHitboxSize
        )
        let bottomRight = CGRect(
            x: self.frame.maxX - Self.windowResizeCornerHitboxOffset,
            y: self.frame.minY - Self.windowResizeCornerHitboxOffset,
            width: Self.windowResizeCornerHitboxSize,
            height: Self.windowResizeCornerHitboxSize
        )
        
        // Return the border the mouse is on
        if topLeft.contains(mouseLocation) {
            return .TopLeft
        } else if topRight.contains(mouseLocation) {
            return .TopRight
        } else if bottomLeft.contains(mouseLocation) {
            return .BottomLeft
        } else if bottomRight.contains(mouseLocation) {
            return .BottomRight
        } else if top.contains(mouseLocation) {
            return .Top
        } else if bottom.contains(mouseLocation) {
            return .Bottom
        } else if left.contains(mouseLocation) {
            return .Left
        } else if right.contains(mouseLocation) {
            return .Right
        } else {
            return nil
        }
    }

    // Changes the cursor icon
    func setCursor(_ cursor: NSCursor, _ nonDefault: Bool = true) {
        cursor.set()
        self.nonDefaultCursor = nonDefault
    }

    // Pass the window destroyed event to Lokinit
    public override func close() {
        EventBuffer.append(LokEvent(.WindowDestroyed, UInt(self.windowNumber)))
        super.close()
    }

    // Make the window the main window, and send focus events to Lokinit
    public func focus() {
        EventBuffer.append(LokEvent(.WindowLostFocus, UInt(NSApp.frontWindow.windowNumber)))
        EventBuffer.append(LokEvent(.WindowGainedFocus, UInt(self.windowNumber)))
        // Yes... we need all 3 of these just to make the window the main window :dawae:
        self.makeKeyAndOrderFront(nil)
        self.makeMain()
        self.becomeMain()
    }
}

#endif
