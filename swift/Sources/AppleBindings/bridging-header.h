#include <CoreFoundation/CFAvailability.h>

// Swift repr of the events
typedef CF_ENUM(int, LokEventType) {
    MouseDownLeft,
    MouseDownMiddle,
    MouseDownRight,
    MouseDownOther,

    MouseUpLeft,
    MouseUpMiddle,
    MouseUpRight,
    MouseUpOther,

    MouseMoved,
    MouseEntered,
    MouseExited,
    MouseScrolled,

    WindowResized,

    KeyPressed,
    KeyReleased,
    KeyRepeated,

    AppQuit
};
struct LokEvent {
    LokEventType type;
    int data1;
    int data2;
    int data3;
    unsigned long window;
};
// Swift representation of the MouseButton and MouseEvent enums
typedef CF_ENUM(int, MouseButton) {
    Left = 0,
    Middle = 1,
    Right = 2
};
typedef CF_ENUM(int, MouseEvent) {
    Pressed = 0,
    Released = 1,
    Moved = 2
};

// Rust FFI callbacks
void rust_mouse_callback(int window, MouseButton btn, MouseEvent event, double x, double y);
void rust_window_resize_callback(unsigned long window, unsigned int width, unsigned int height);
