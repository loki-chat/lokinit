// Tokens for glXChooseVisual and glXGetConfig

#![allow(non_snake_case, clippy::upper_case_acronyms)]

use std::ffi::{c_char, c_int, c_uint, c_ulong};

use crate::library;
use crate::x11::{Bool, Font, Pixmap, XDisplay, XVisualInfo, XWindow, XID};

pub const GLX_USE_GL: u32 = 1;
pub const GLX_BUFFER_SIZE: u32 = 2;
pub const GLX_LEVEL: u32 = 3;
pub const GLX_RGBA: u32 = 4;
pub const GLX_DOUBLEBUFFER: u32 = 5;
pub const GLX_STEREO: u32 = 6;
pub const GLX_AUX_BUFFERS: u32 = 7;
pub const GLX_RED_SIZE: u32 = 8;
pub const GLX_GREEN_SIZE: u32 = 9;
pub const GLX_BLUE_SIZE: u32 = 10;
pub const GLX_ALPHA_SIZE: u32 = 11;
pub const GLX_DEPTH_SIZE: u32 = 12;
pub const GLX_STENCIL_SIZE: u32 = 13;
pub const GLX_ACCUM_RED_SIZE: u32 = 14;
pub const GLX_ACCUM_GREEN_SIZE: u32 = 15;
pub const GLX_ACCUM_BLUE_SIZE: u32 = 16;
pub const GLX_ACCUM_ALPHA_SIZE: u32 = 17;

// Error codes returned by glXGetConfig

pub const GLX_BAD_SCREEN: u32 = 1;
pub const GLX_BAD_ATTRIBUTE: u32 = 2;
pub const GLX_NO_EXTENSION: u32 = 3;
pub const GLX_BAD_VISUAL: u32 = 4;
pub const GLX_BAD_CONTEXT: u32 = 5;
pub const GLX_BAD_VALUE: u32 = 6;
pub const GLX_BAD_ENUM: u32 = 7;

// GLX 1.1 and later

pub const GLX_VENDOR: u32 = 1;
pub const GLX_VERSION: u32 = 2;
pub const GLX_EXTENSIONS: u32 = 3;

// GLX 1.3 and later:

pub const GLX_CONFIG_CAVEAT: u32 = 0x20;
pub const GLX_DONT_CARE: u32 = 0xFFFFFFFF;
pub const GLX_X_VISUAL_TYPE: u32 = 0x22;
pub const GLX_TRANSPARENT_TYPE: u32 = 0x23;
pub const GLX_TRANSPARENT_INDEX_VALUE: u32 = 0x24;
pub const GLX_TRANSPARENT_RED_VALUE: u32 = 0x25;
pub const GLX_TRANSPARENT_GREEN_VALUE: u32 = 0x26;
pub const GLX_TRANSPARENT_BLUE_VALUE: u32 = 0x27;
pub const GLX_TRANSPARENT_ALPHA_VALUE: u32 = 0x28;
pub const GLX_WINDOW_BIT: u32 = 0x00000001;
pub const GLX_PIXMAP_BIT: u32 = 0x00000002;
pub const GLX_PBUFFER_BIT: u32 = 0x00000004;
pub const GLX_AUX_BUFFERS_BIT: u32 = 0x00000010;
pub const GLX_FRONT_LEFT_BUFFER_BIT: u32 = 0x00000001;
pub const GLX_FRONT_RIGHT_BUFFER_BIT: u32 = 0x00000002;
pub const GLX_BACK_LEFT_BUFFER_BIT: u32 = 0x00000004;
pub const GLX_BACK_RIGHT_BUFFER_BIT: u32 = 0x00000008;
pub const GLX_DEPTH_BUFFER_BIT: u32 = 0x00000020;
pub const GLX_STENCIL_BUFFER_BIT: u32 = 0x00000040;
pub const GLX_ACCUM_BUFFER_BIT: u32 = 0x00000080;
pub const GLX_NONE: u32 = 0x8000;
pub const GLX_SLOW_CONFIG: u32 = 0x8001;
pub const GLX_TRUE_COLOR: u32 = 0x8002;
pub const GLX_DIRECT_COLOR: u32 = 0x8003;
pub const GLX_PSEUDO_COLOR: u32 = 0x8004;
pub const GLX_STATIC_COLOR: u32 = 0x8005;
pub const GLX_GRAY_SCALE: u32 = 0x8006;
pub const GLX_STATIC_GRAY: u32 = 0x8007;
pub const GLX_TRANSPARENT_RGB: u32 = 0x8008;
pub const GLX_TRANSPARENT_INDEX: u32 = 0x8009;
pub const GLX_VISUAL_ID: u32 = 0x800B;
pub const GLX_SCREEN: u32 = 0x800C;
pub const GLX_NON_CONFORMANT_CONFIG: u32 = 0x800D;
pub const GLX_DRAWABLE_TYPE: u32 = 0x8010;
pub const GLX_RENDER_TYPE: u32 = 0x8011;
pub const GLX_X_RENDERABLE: u32 = 0x8012;
pub const GLX_FBCONFIG_ID: u32 = 0x8013;
pub const GLX_RGBA_TYPE: u32 = 0x8014;
pub const GLX_COLOR_INDEX_TYPE: u32 = 0x8015;
pub const GLX_MAX_PBUFFER_WIDTH: u32 = 0x8016;
pub const GLX_MAX_PBUFFER_HEIGHT: u32 = 0x8017;
pub const GLX_MAX_PBUFFER_PIXELS: u32 = 0x8018;
pub const GLX_PRESERVED_CONTENTS: u32 = 0x801B;
pub const GLX_LARGEST_PBUFFER: u32 = 0x801C;
pub const GLX_WIDTH: u32 = 0x801D;
pub const GLX_HEIGHT: u32 = 0x801E;
pub const GLX_EVENT_MASK: u32 = 0x801F;
pub const GLX_DAMAGED: u32 = 0x8020;
pub const GLX_SAVED: u32 = 0x8021;
pub const GLX_WINDOW: u32 = 0x8022;
pub const GLX_PBUFFER: u32 = 0x8023;
pub const GLX_PBUFFER_HEIGHT: u32 = 0x8040;
pub const GLX_PBUFFER_WIDTH: u32 = 0x8041;
pub const GLX_RGBA_BIT: u32 = 0x00000001;
pub const GLX_COLOR_INDEX_BIT: u32 = 0x00000002;
pub const GLX_PBUFFER_CLOBBER_MASK: u32 = 0x08000000;

// GLX 1.4 and later:

pub const GLX_SAMPLE_BUFFERS: u32 = 100000;
pub const GLX_SAMPLES: u32 = 100001;

#[repr(C)]
pub struct _GLXContext([u8; 0]);
pub type GLXContext = *mut _GLXContext;
pub type GLXPixmap = XID;
pub type GLXDrawable = XID;

// GLX 1.3 and later
#[repr(C)]
pub struct _GLXFBConfig([u8; 0]);
pub type GLXFBConfig = *mut _GLXFBConfig;
pub type GLXFBConfigID = XID;
pub type GLXContextID = XID;
pub type GLXWindow = XID;
pub type GLXPbuffer = XID;

library! {
    [LibGlx <-> "GLX"];

    pub fn glXChooseVisual(display: *mut XDisplay, screen: c_int, attrib_list: *mut c_int) -> *mut XVisualInfo;
    pub fn glXCreateContext(display: *mut XDisplay, vis: *mut XVisualInfo, share_list: GLXContext, direct: Bool) -> *mut XVisualInfo;
	pub fn glXDestroyContext(display: *mut XDisplay, ctx: GLXContext);
	pub fn glXMakeCurrent(display: *mut XDisplay, drawable: GLXDrawable) -> Bool;
	pub fn glXCopyContext(display: *mut XDisplay, src: GLXContext, dst: GLXContext, mask: c_ulong);
	pub fn glXSwapBuffers(display: *mut XDisplay, drawable: GLXDrawable);
	pub fn glXCreateGLXPixmap(display: *mut XDisplay, visual: *mut XVisualInfo, pixmap: Pixmap) -> GLXPixmap;
	pub fn glXDestroyGLXPixmap(display: *mut XDisplay, pixmap: GLXPixmap);
	pub fn glXQueryExtension(display: *mut XDisplay, errorb: *mut c_int, event: *mut c_int) -> Bool;
	pub fn glXQueryVersion(display: *mut XDisplay, maj: *mut c_int, min: *mut c_int) -> Bool;
	pub fn glXIsDirect(display: *mut XDisplay, ctx: GLXContext) -> Bool;
	pub fn glXGetConfig(display: *mut XDisplay, visual: *mut XVisualInfo, attrib: c_int, value: *mut c_int) -> c_int;
	pub fn glXGetCurrentContext() -> GLXContext;
	pub fn glXGetCurrentDrawable() -> GLXDrawable;
	pub fn glXWaitGL();
	pub fn glXWaitX();
	pub fn glXUseXFont(font: Font, first: c_int, count: c_int, list: c_int);

	// GLX 1.1 and later

	pub fn glXQueryExtensionsString(display: *mut XDisplay, screen: c_int) -> *const c_char;
	pub fn glXQueryServerString(display: *mut XDisplay, screen: c_int, name: c_int) -> *const c_char;
	pub fn glXGetClientString(display: *mut XDisplay, name: c_int) -> *const c_char;

	// GLX 1.2 and later

	pub fn glXGetCurrentDisplay() -> *mut XDisplay;

	// GLX 1.3 and later

	pub fn glXChooseFBConfig(display: *mut XDisplay, screen: c_int, attrib_list: *const c_int, nitems: *mut c_int) -> *mut GLXFBConfig;
	pub fn glXGetFBConfigAttrib(display: *mut XDisplay, config: GLXFBConfig, attribute: c_int, value: *mut c_int) -> c_int;
	pub fn glXGetFBConfigs(display: *mut XDisplay, screen: c_int, nelements: *mut c_int) -> *mut GLXFBConfig;
	pub fn glXGetVisualFromFBConfig(display: *mut XDisplay, config: GLXFBConfig) -> *mut XVisualInfo;
	pub fn glXCreateWindow(display: *mut XDisplay, config: GLXFBConfig, window: XWindow, attrib_list: *const c_int) -> GLXWindow;
	pub fn glXDestroyWindow(display: *mut XDisplay, window: GLXWindow);
	pub fn glXCreatePixmap(display: *mut XDisplay, config: GLXFBConfig, pixmap: Pixmap, attrib_list: *const c_int) -> GLXPixmap;
	pub fn glXDestroyPixmap(display: *mut XDisplay, pixmap: GLXPixmap);
	pub fn glXCreatePbuffer(display: *mut XDisplay, config: GLXFBConfig, attrib_list: *const c_int) -> GLXPbuffer;
	pub fn glXDestroyPbuffer(display: *mut XDisplay, pbuf: GLXPbuffer);
	pub fn glXQueryDrawable(display: *mut XDisplay, draw: GLXDrawable, attribute: c_int, value: *mut c_uint);
	pub fn glXCreateNewContext(display: *mut XDisplay, config: GLXFBConfig, render_type: c_int, share_list: GLXContext, direct: Bool) -> GLXContext;
	pub fn glXMakeContextCurrent(display: *mut XDisplay, draw: GLXDrawable, read: GLXDrawable, ctx: GLXContext) -> Bool;
	pub fn glXGetCurrentReadDrawable() -> GLXDrawable;
	pub fn glXQueryContext(display: *mut XDisplay, ctx: GLXContext, attribute: c_int, value: *mut c_int) -> c_int;
	pub fn glXSelectEvent(display: *mut XDisplay, drawable: GLXDrawable, mask: c_ulong);
	pub fn glXGetSelectedEvent(display: *mut XDisplay, drawable: GLXDrawable, mask: *mut c_ulong);
}
