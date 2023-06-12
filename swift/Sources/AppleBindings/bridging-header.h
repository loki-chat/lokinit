#include <CoreFoundation/CFAvailability.h>

typedef CF_ENUM(int, MouseButton) {
    Left,
    Middle,
    Right
};

typedef CF_ENUM(int, MouseEvent) {
    Down,
    Up,
    Moved
};

void rust_mouse_callback(MouseButton btn, MouseEvent event, double x, double y);