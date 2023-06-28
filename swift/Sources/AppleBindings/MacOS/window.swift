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
    
    // Which part of the window is being resized right now, if it's being resized
    var resizeBorder: WindowBorderLocation? = nil
    // If the cursor is a non-default cursor
    var nonDefaultCursor = false
    
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
            // Yes... we need all 3 of these just to make the window the main window :dawae:
            self.makeKeyAndOrderFront(nil)
            self.makeMain()
            self.becomeMain()
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
            if rect.width > Self.minimumWindowSize.width && rect.height > Self.minimumWindowSize.height {
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
        let mousePos = self.getMouseLocation()
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
    
    // Checks if the mouse is in any of the window borders
    func checkMouseInBorder() -> WindowBorderLocation? {
        let cornerBoxSize = 15.0
        let cornerBoxOffset = (cornerBoxSize + 1.0) / 2.0
        let sideBoxSize = 7.0
        let sideBoxOffset = (sideBoxSize + 1.0) / 2.0
        let mouseLocationWindow = self.getMouseLocation()
        let mouseLocationScreen = NSEvent.mouseLocation
        
        let innerWindow = CGRect(
            x: sideBoxOffset,
            y: sideBoxOffset,
            width: self.frame.width - (sideBoxOffset * 2),
            height: self.frame.height - (sideBoxOffset * 2)
        )
        if innerWindow.contains(mouseLocationWindow) {
            return nil
        }
        
        //            print("Mouse moved to \(mouseLocation) || Window frame: \(frame)")
        
        // Calculate boxes for the top, bottom, left, and right edges of the window
        let top = CGRect(
            x: self.frame.minX,
            y: self.frame.maxY - sideBoxOffset,
            width: self.frame.width,
            height: sideBoxSize
        )
        let bottom = CGRect(
            x: self.frame.minX,
            y: self.frame.minY - sideBoxOffset,
            width: self.frame.width,
            height: sideBoxSize
        )
        let left = CGRect(
            x: self.frame.minX - sideBoxOffset,
            y: self.frame.minY,
            width: sideBoxSize,
            height: self.frame.height
        )
        let right = CGRect(
            x: self.frame.maxX - sideBoxOffset,
            y: self.frame.minY,
            width: sideBoxSize,
            height: self.frame.height
        )
        
        // Calculate boxes for the corners of the window
        let topLeft = CGRect(
            x: self.frame.minX - cornerBoxOffset,
            y: self.frame.maxY - cornerBoxOffset,
            width: cornerBoxSize,
            height: cornerBoxSize
        )
        let topRight = CGRect(
            x: self.frame.maxX - cornerBoxOffset,
            y: self.frame.maxY - cornerBoxOffset,
            width: cornerBoxSize,
            height: cornerBoxSize
        )
        let bottomLeft = CGRect(
            x: self.frame.minX - cornerBoxOffset,
            y: self.frame.minY - cornerBoxOffset,
            width: cornerBoxSize,
            height: cornerBoxSize
        )
        let bottomRight = CGRect(
            x: self.frame.maxX - cornerBoxOffset,
            y: self.frame.minY - cornerBoxOffset,
            width: cornerBoxSize,
            height: cornerBoxSize
        )
        
        // Return the border the mouse is on
        if topLeft.contains(mouseLocationScreen) {
            return .TopLeft
        } else if topRight.contains(mouseLocationScreen) {
            return .TopRight
        } else if bottomLeft.contains(mouseLocationScreen) {
            return .BottomLeft
        } else if bottomRight.contains(mouseLocationScreen) {
            return .BottomRight
        } else if top.contains(mouseLocationScreen) {
            return .Top
        } else if bottom.contains(mouseLocationScreen) {
            return .Bottom
        } else if left.contains(mouseLocationScreen) {
            return .Left
        } else if right.contains(mouseLocationScreen) {
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
}

#endif
