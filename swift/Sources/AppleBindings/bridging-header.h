#include <CoreFoundation/CFAvailability.h>

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

void rust_mouse_callback(int window, MouseButton btn, MouseEvent event, double x, double y);