use loki_linux::wayland::{interfaces::display::DisplayListener, Object};

pub struct DisplayEventListener {}
impl DisplayListener for DisplayEventListener {
    fn error(&self, object: &dyn Object, code: u32, message: &str) {
        panic!(
            "
            Lokinit: Critical wayland error!\n\
            Object:\n\t\
                Interface: {}\n\t\
                Interface Version: {}\n\t\
                ID: {}\n\t\
            Wayland Error Code: {code}
            Wayland Error Message: `{message}`
            ",
            object.interface(),
            object.version(),
            object.id()
        )
    }

    fn delete_object(&self, _object: &dyn Object) {
        todo!("Delete objects")
    }
}
