#if os(macOS)

import Foundation
import AppKit

// Loads external cursors that aren't in NSCursor
public struct BSCursor {
    static let baseCursorPath = URL(string: "file:///System/Library/Frameworks/ApplicationServices.framework/Versions/A/Frameworks/HIServices.framework/Versions/A/Resources/cursors/")!
    
    public static let windowResizeNorthSouth: NSCursor = fetchHICursor("resizenorthsouth")
    public static let windowResizeEastWest: NSCursor = fetchHICursor("resizeeastwest")
    public static let windowResizeNorthEastSouthWest: NSCursor = fetchHICursor("resizenortheastsouthwest")
    public static let windowResizeNorthWestSouthEast: NSCursor = fetchHICursor("resizenorthwestsoutheast")
    public static let empty: NSCursor = NSCursor(image: NSImage(size: NSSize.zero), hotSpot: NSPoint.zero)
    
    // Loads cursors from HIServices.framework
    // https://stackoverflow.com/a/21786835/19707043
    static func fetchHICursor(_ name: String) -> NSCursor {
        let path = baseCursorPath.append(name + "/")
        let image = NSImage(byReferencing: path.append("cursor.pdf"))
        let info = try! NSDictionary(contentsOf: path.append("info.plist"), error: ())
        let hotspot = NSPoint(x: info.value(forKey: "hotx")! as! Double, y: info.value(forKey: "hoty")! as! Double)
        return NSCursor(image: image, hotSpot: hotspot)
    }
}

#endif
