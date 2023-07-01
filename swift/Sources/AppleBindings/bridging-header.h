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
    WindowMoved,
    WindowCloseRequested,
    WindowDestroyed,
    WindowGainedFocus,
    WindowLostFocus,

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
typedef struct LokEvent LokEvent;
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
void rust_queue_event(LokEvent event);
