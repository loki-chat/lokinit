use {
    crate::window::WindowBorder,
    core::ops::{Deref, DerefMut},
    loki_mac::{NSPoint, NSRect, NSSize, NSWindow},
};

/// A wrapper around [`NSWindow`] with resize features.
pub struct Window {
    pub nswindow: NSWindow,
    pub borders: [NSRect; 8],
}
impl Window {
    pub fn new(nswindow: NSWindow) -> Self {
        let mut this = Self {
            nswindow,
            borders: [
                NSRect::default(),
                NSRect::default(),
                NSRect::default(),
                NSRect::default(),
                NSRect::default(),
                NSRect::default(),
                NSRect::default(),
                NSRect::default(),
            ],
        };
        this.recalculate_borders();

        this
    }

    pub fn point_to_window_coordinates(&self, point: Point) -> (i32, i32) {
        match point {
            Point::Window(x, y) => (x, y),
            Point::Screen(x, y) => {
                let origin = self.frame().origin;
                (x - origin.x as i32, y - origin.y as i32)
            }
        }
    }
    pub fn point_to_screen_coordinates(&self, point: Point) -> (i32, i32) {
        match point {
            Point::Window(x, y) => {
                let origin = self.frame().origin;
                (x + origin.x as i32, y + origin.y as i32)
            }
            Point::Screen(x, y) => (x, y),
        }
    }

    /// Resize a border of this window to a point on the screen.
    pub fn resize_border(&mut self, border: WindowBorder, point: Point) {
        let (mouse_x, mouse_y) = self.point_to_window_coordinates(point);
        let mouse_x = mouse_x as f64;
        let mouse_y = mouse_y as f64;
        let frame = self.frame();

        let mut new_frame = frame.clone();
        match border {
            WindowBorder::Top => {
                new_frame.size.height = mouse_y;
            }
            WindowBorder::Bottom => {
                new_frame.size.height -= mouse_y;
                new_frame.origin.y += mouse_y;
            }
            WindowBorder::Left => {
                new_frame.origin.x += mouse_x;
                new_frame.size.width -= mouse_x;
            }
            WindowBorder::Right => {
                new_frame.size.width = mouse_x;
            }

            WindowBorder::TopLeft => {
                new_frame.origin.x += mouse_x;
                new_frame.size.width -= mouse_x;
                new_frame.size.height = mouse_y;
            }
            WindowBorder::TopRight => {
                new_frame.size.width = mouse_x;
                new_frame.size.height = mouse_y;
            }
            WindowBorder::BottomLeft => {
                new_frame.origin.x += mouse_x;
                new_frame.origin.y += mouse_y;
                new_frame.size.width -= mouse_x;
                new_frame.size.height -= mouse_y;
            }
            WindowBorder::BottomRight => {
                new_frame.origin.y += mouse_y;
                new_frame.size.width = mouse_x;
                new_frame.size.height -= mouse_y;
            }
        }

        self.set_frame(new_frame, false.into(), false.into());
        self.recalculate_borders();
    }

    /// Returns which window border a point is inside of, if it's in a border.
    pub fn point_in_border(&self, point: Point) -> Option<WindowBorder> {
        let (x, y) = self.point_to_screen_coordinates(point);
        for (idx, border) in self.borders.iter().enumerate() {
            if border.contains(x, y) {
                return Some(idx.into());
            }
        }
        None
    }

    /// Should be called every time this window moves or is resized. Recalculates
    /// all of the borders that are checked in [`Self::point_in_border`].
    pub fn recalculate_borders(&mut self) {
        let frame = self.frame();

        /// The size of the hitboxes for the window corners.
        const CORNER_HITBOX_SIZE: f64 = 15.0;
        /// How much of a corner hitbox is outside the window.
        const CORNER_OUTSIDE_HITBOX_SIZE: f64 = 3.0;
        /// How much of a corner hitbox is inside the window.
        const CORNER_INSIDE_HITBOX_SIZE: f64 = 12.0;

        /// The size of the hitboxes for the window sides.
        const SIDE_HITBOX_SIZE: f64 = 7.0;
        /// How much of a side hitbox is outside the window.
        const SIDE_OUTSIDE_HITBOX_SIZE: f64 = 3.0;
        /// How much of a side hitbox is inside the window.
        const SIDE_INSIDE_HITBOX_SIZE: f64 = 4.0;

        // Maybe useful note: Apple's origins are from the *bottom left*,
        // not the top left.

        // Sides
        self.borders[WindowBorder::Top as usize] = NSRect {
            origin: NSPoint {
                x: frame.origin.x + CORNER_INSIDE_HITBOX_SIZE,
                y: frame.origin.y + frame.size.height - SIDE_INSIDE_HITBOX_SIZE,
            },
            size: NSSize {
                width: frame.size.width - (2.0 * CORNER_INSIDE_HITBOX_SIZE),
                height: SIDE_HITBOX_SIZE,
            },
        };
        self.borders[WindowBorder::Bottom as usize] = NSRect {
            origin: NSPoint {
                x: frame.origin.x + CORNER_INSIDE_HITBOX_SIZE,
                y: frame.origin.y - SIDE_OUTSIDE_HITBOX_SIZE,
            },
            size: NSSize {
                width: frame.size.width - (2.0 * CORNER_INSIDE_HITBOX_SIZE),
                height: SIDE_HITBOX_SIZE,
            },
        };
        self.borders[WindowBorder::Left as usize] = NSRect {
            origin: NSPoint {
                x: frame.origin.x - SIDE_OUTSIDE_HITBOX_SIZE,
                y: frame.origin.y + CORNER_INSIDE_HITBOX_SIZE,
            },
            size: NSSize {
                width: SIDE_HITBOX_SIZE,
                height: frame.size.height - (2.0 * CORNER_INSIDE_HITBOX_SIZE),
            },
        };
        self.borders[WindowBorder::Right as usize] = NSRect {
            origin: NSPoint {
                x: frame.origin.x + frame.size.width - SIDE_INSIDE_HITBOX_SIZE,
                y: frame.origin.y + CORNER_INSIDE_HITBOX_SIZE,
            },
            size: NSSize {
                width: SIDE_HITBOX_SIZE,
                height: frame.size.height - (2.0 * CORNER_INSIDE_HITBOX_SIZE),
            },
        };

        // Corners
        self.borders[WindowBorder::TopLeft as usize] = NSRect {
            origin: NSPoint {
                x: frame.origin.x - CORNER_OUTSIDE_HITBOX_SIZE,
                y: frame.origin.y + frame.size.height - CORNER_INSIDE_HITBOX_SIZE,
            },
            size: NSSize {
                width: CORNER_HITBOX_SIZE,
                height: CORNER_HITBOX_SIZE,
            },
        };
        self.borders[WindowBorder::TopRight as usize] = NSRect {
            origin: NSPoint {
                x: frame.origin.x + frame.size.width - CORNER_INSIDE_HITBOX_SIZE,
                y: frame.origin.y + frame.size.height - CORNER_INSIDE_HITBOX_SIZE,
            },
            size: NSSize {
                width: CORNER_HITBOX_SIZE,
                height: CORNER_HITBOX_SIZE,
            },
        };
        self.borders[WindowBorder::BottomRight as usize] = NSRect {
            origin: NSPoint {
                x: frame.origin.x + frame.size.width - CORNER_INSIDE_HITBOX_SIZE,
                y: frame.origin.y - CORNER_OUTSIDE_HITBOX_SIZE,
            },
            size: NSSize {
                width: CORNER_HITBOX_SIZE,
                height: CORNER_HITBOX_SIZE,
            },
        };
        self.borders[WindowBorder::BottomLeft as usize] = NSRect {
            origin: NSPoint {
                x: frame.origin.x - CORNER_OUTSIDE_HITBOX_SIZE,
                y: frame.origin.y - CORNER_OUTSIDE_HITBOX_SIZE,
            },
            size: NSSize {
                width: CORNER_HITBOX_SIZE,
                height: CORNER_HITBOX_SIZE,
            },
        };
    }
}
impl Deref for Window {
    type Target = NSWindow;

    fn deref(&self) -> &Self::Target {
        &self.nswindow
    }
}
impl DerefMut for Window {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.nswindow
    }
}

impl From<usize> for WindowBorder {
    fn from(value: usize) -> Self {
        match value {
            0 => Self::Top,
            1 => Self::Bottom,
            2 => Self::Left,
            3 => Self::Right,
            4 => Self::TopLeft,
            5 => Self::TopRight,
            6 => Self::BottomLeft,
            7 => Self::BottomRight,

            _ => unreachable!(),
        }
    }
}

pub enum Point {
    Window(i32, i32),
    Screen(i32, i32),
}
