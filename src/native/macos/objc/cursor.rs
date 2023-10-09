use {
    super::{
        super::bplist::{BPlist, Object},
        ffi, msg, msg_ret, str_to_nsstring, NSPoint, VTables,
    },
    std::{
        cell::OnceCell, collections::HashMap, ffi::c_void, fs::read, path::PathBuf, str::FromStr,
    },
    syntax::Parser,
};

const BASE_PATH: &str = concat!(
    "/System/Library/Frameworks/ApplicationServices.framework/",
    "Versions/A/Frameworks/HIServices.framework/Versions/A/Resources/cursors/"
);
const MOUSE_ERROR: &str = concat!(
    "Malformed mouse cursor cfg. Please open an issue on Lokinit's GitHub, ",
    "a macOS update might have broken Lokinit."
);

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum MacOsCursor {
    Arrow,
    ResizeNorthSouth,
    ResizeEastWest,
    ResizeNorthEastSouthWest,
    ResizeNorthWestSouthEast,
}

thread_local! {
    pub static CURSORS: OnceCell<Cursors> = OnceCell::new();
}

#[derive(Debug)]
pub struct Cursors {
    map: HashMap<MacOsCursor, *mut c_void>,
}
impl Cursors {
    pub fn with<T>(func: impl Fn(&Self) -> T) -> T {
        CURSORS.with(|cursors| func(cursors.get().unwrap()))
    }

    pub fn get(&self, cursor: MacOsCursor) -> NSCursor {
        NSCursor {
            ptr: self.map.get(&cursor).unwrap().to_owned(),
        }
    }
}
impl Default for Cursors {
    fn default() -> Self {
        let mut map = HashMap::default();

        map.insert(MacOsCursor::Arrow, get_standard_cursor("arrow"));
        map.insert(
            MacOsCursor::ResizeNorthSouth,
            get_hi_cursor("resizenorthsouth"),
        );
        map.insert(MacOsCursor::ResizeEastWest, get_hi_cursor("resizeeastwest"));
        map.insert(
            MacOsCursor::ResizeNorthEastSouthWest,
            get_hi_cursor("resizenortheastsouthwest"),
        );
        map.insert(
            MacOsCursor::ResizeNorthWestSouthEast,
            get_hi_cursor("resizenorthwestsoutheast"),
        );

        Self { map }
    }
}

pub struct NSCursor {
    pub ptr: *mut c_void,
}
impl NSCursor {
    /// https://developer.apple.com/documentation/appkit/nscursor/1526148-set?language=objc
    pub fn set(&self) {
        let set = VTables::with(|vtables| vtables.nscursor.set_sel);
        let ptr = self.ptr;
        msg![ptr set]
    }
}

/// Loads pointers to default NSCursor instances
fn get_standard_cursor(name: &str) -> *mut c_void {
    let sel = unsafe {
        ffi::sel_getUid((name.to_string() + "Cursor\0").as_ptr() as *const std::ffi::c_char)
    };
    let nscursor = VTables::with(|vtables| vtables.nscursor.class);
    msg_ret![nscursor sel]
}

/// Loads cursors from HIServices.framework
/// https://stackoverflow.com/a/21786835/19707043
fn get_hi_cursor(name: &str) -> *mut c_void {
    let mut path = PathBuf::from_str(BASE_PATH).unwrap();
    path.push(name);
    let bytes = read(path.join("info.plist")).unwrap();
    let cfg: BPlist = Parser::parse(bytes).unwrap();
    let Object::Dict(cfg) = cfg.objects.into_iter().next().expect(MOUSE_ERROR) else {
        panic!("{}", MOUSE_ERROR);
    };
    let (nsimage, nsimage_init, nscursor, nscursor_init, alloc) = VTables::with(|vtables| {
        (
            vtables.nsimage.class,
            vtables.nsimage.init_sel,
            vtables.nscursor.class,
            vtables.nscursor.init_sel,
            vtables.nscursor.alloc,
        )
    });

    let cursor_img = str_to_nsstring(path.join("cursor.pdf").to_str().unwrap());
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

pub fn load_cursors() {
    CURSORS
        .with(|cursors| cursors.set(Cursors::default()))
        .unwrap();
}
