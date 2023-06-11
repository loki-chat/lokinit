#if os(iOS)
public typealias NativeApp = IosApplication
#else
#endif

// Converts data to a pointer
// Would be a macro, if Swift had macros :|
func ptr<T: AnyObject>(_ data: T) -> UnsafeMutableRawPointer {
    return Unmanaged.passRetained(data).toOpaque()
}
