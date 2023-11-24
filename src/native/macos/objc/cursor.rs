use {
    super::{
        super::bplist::{BPlistParser, Object},
        ffi, msg_ret, str_to_nsstring, NSPoint, VTables,
    },
    std::{collections::HashMap, ffi::c_void, fs::read, path::PathBuf, str::FromStr},
};

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub enum MacOsCursor {
    Arrow,
    ResizeNorthSouth,
    ResizeEastWest,
    ResizeNorthEastSouthWest,
    ResizeNorthWestSouthEast,
}

pub fn load_cursors(vtables: &VTables) -> HashMap<MacOsCursor, *mut c_void> {
    let mut map = HashMap::default();

    map.insert(MacOsCursor::Arrow, get_standard_cursor(vtables, "arrow"));
    map.insert(
        MacOsCursor::ResizeNorthSouth,
        get_hi_cursor(vtables, "resizenorthsouth"),
    );
    map.insert(
        MacOsCursor::ResizeEastWest,
        get_hi_cursor(vtables, "resizeeastwest"),
    );
    map.insert(
        MacOsCursor::ResizeNorthEastSouthWest,
        get_hi_cursor(vtables, "resizenortheastsouthwest"),
    );
    map.insert(
        MacOsCursor::ResizeNorthWestSouthEast,
        get_hi_cursor(vtables, "resizenorthwestsoutheast"),
    );

    map
}

/// Loads pointers to default NSCursor instances
fn get_standard_cursor(vtables: &VTables, name: &str) -> *mut c_void {
    let sel =
        unsafe { ffi::sel_getUid(format!("{name}Cursor\0").as_ptr() as *const std::ffi::c_char) };
    let nscursor = vtables.nscursor.class;
    msg_ret![nscursor sel]
}

const BASE_PATH: &str = concat!(
    "/System/Library/Frameworks/ApplicationServices.framework/",
    "Versions/A/Frameworks/HIServices.framework/Versions/A/Resources/cursors/"
);
const MOUSE_ERROR: &str = concat!(
    "Malformed mouse cursor cfg. Please open an issue on Lokinit's GitHub, ",
    "a macOS update might have broken Lokinit."
);

/// Loads cursors from HIServices.framework
/// https://stackoverflow.com/a/21786835/19707043
fn get_hi_cursor(vtables: &VTables, name: &str) -> *mut c_void {
    let mut path = PathBuf::from_str(BASE_PATH).unwrap();
    path.push(name);
    let bytes = read(path.join("info.plist")).unwrap();
    let cfg = BPlistParser::new_and_parse(bytes);
    let Object::Dict(cfg) = cfg.first().expect(MOUSE_ERROR) else {
        panic!("{}", MOUSE_ERROR);
    };
    let (nsimage, nsimage_init, nscursor, nscursor_init, alloc) = (
        vtables.nsimage.class,
        vtables.nsimage.init_sel,
        vtables.nscursor.class,
        vtables.nscursor.init_sel,
        vtables.nscursor.alloc,
    );

    let cursor_img = str_to_nsstring(path.join("cursor.pdf").to_str().unwrap(), vtables);
    let img: *mut c_void = msg_ret![nsimage alloc];
    let cursor_img: *mut c_void = msg_ret![img nsimage_init initWithContentsOfFile:cursor_img];

    let Some(Object::Int(x)) = cfg.get("hotx") else {
        panic!("{}", MOUSE_ERROR)
    };
    let Some(Object::Int(y)) = cfg.get("hoty") else {
        panic!("{}", MOUSE_ERROR)
    };
    let hotspot = NSPoint {
        x: x.to_owned() as f64,
        y: y.to_owned() as f64,
    };

    let cursor = msg_ret![nscursor alloc];
    msg_ret![cursor nscursor_init initWithImage:cursor_img hotSpot:hotspot]
}
