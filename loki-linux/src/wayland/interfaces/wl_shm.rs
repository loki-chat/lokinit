use std::ffi::{c_int, c_void};

use crate::wayland::{LibWaylandClient, NULLPTR};

use super::wl_shm_pool::WlShmPool;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WlShmError {
    /// buffer format is not known
    InvalidFormat = 0,
    /// invalid size or stride during pool or buffer creation
    InvalidStride = 1,
    /// mmapping the file descriptor failed
    InvalidFd = 2,
}

/// Pixel formats
///
/// This describes the memory layout of an individual pixel.
///
/// All renderers should support argb8888 and xrgb8888 but any other
/// formats are optional and may not be supported by the particular
/// renderer in use.
///
/// The drm format codes match the macros defined in drm_fourcc.h, except
/// argb8888 and xrgb8888. The formats actually supported by the compositor
/// will be reported by the format event.
///
/// For all wl_shm formats and unless specified in another protocol
/// extension, pre-multiplied alpha is used for pixel values.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WlShmFormat {
    /// 32-bit ARGB format, [31:0] A:R:G:B 8:8:8:8 little endian
    Argb8888 = 0,
    /// 32-bit RGB format, [31:0] x:R:G:B 8:8:8:8 little endian
    Xrgb8888 = 1,
    /// 8-bit color index format, [7:0] C
    C8 = 0x20203843,
    /// 8-bit RGB format, [7:0] R:G:B 3:3:2
    Rgb332 = 0x38424752,
    /// 8-bit BGR format, [7:0] B:G:R 2:3:3
    Bgr233 = 0x38524742,
    /// 16-bit xRGB format, [15:0] x:R:G:B 4:4:4:4 little endian
    Xrgb4444 = 0x32315258,
    /// 16-bit xBGR format, [15:0] x:B:G:R 4:4:4:4 little endian
    Xbgr4444 = 0x32314258,
    /// 16-bit RGBx format, [15:0] R:G:B:x 4:4:4:4 little endian
    Rgbx4444 = 0x32315852,
    /// 16-bit BGRx format, [15:0] B:G:R:x 4:4:4:4 little endian
    Bgrx4444 = 0x32315842,
    /// 16-bit ARGB format, [15:0] A:R:G:B 4:4:4:4 little endian
    Argb4444 = 0x32315241,
    /// 16-bit ABGR format, [15:0] A:B:G:R 4:4:4:4 little endian
    Abgr4444 = 0x32314241,
    /// 16-bit RBGA format, [15:0] R:G:B:A 4:4:4:4 little endian
    Rgba4444 = 0x32314152,
    /// 16-bit BGRA format, [15:0] B:G:R:A 4:4:4:4 little endian
    Bgra4444 = 0x32314142,
    /// 16-bit xRGB format, [15:0] x:R:G:B 1:5:5:5 little endian
    Xrgb1555 = 0x35315258,
    /// 16-bit xBGR 1555 format, [15:0] x:B:G:R 1:5:5:5 little endian
    Xbgr1555 = 0x35314258,
    /// 16-bit RGBx 5551 format, [15:0] R:G:B:x 5:5:5:1 little endian
    Rgbx5551 = 0x35315852,
    /// 16-bit BGRx 5551 format, [15:0] B:G:R:x 5:5:5:1 little endian
    Bgrx5551 = 0x35315842,
    /// 16-bit ARGB 1555 format, [15:0] A:R:G:B 1:5:5:5 little endian
    Argb1555 = 0x35315241,
    /// 16-bit ABGR 1555 format, [15:0] A:B:G:R 1:5:5:5 little endian
    Abgr1555 = 0x35314241,
    /// 16-bit RGBA 5551 format, [15:0] R:G:B:A 5:5:5:1 little endian
    Rgba5551 = 0x35314152,
    /// 16-bit BGRA 5551 format, [15:0] B:G:R:A 5:5:5:1 little endian
    Bgra5551 = 0x35314142,
    /// 16-bit RGB 565 format, [15:0] R:G:B 5:6:5 little endian
    Rgb565 = 0x36314752,
    /// 16-bit BGR 565 format, [15:0] B:G:R 5:6:5 little endian
    Bgr565 = 0x36314742,
    /// 24-bit RGB format, [23:0] R:G:B little endian
    Rgb888 = 0x34324752,
    /// 24-bit BGR format, [23:0] B:G:R little endian
    Bgr888 = 0x34324742,
    /// 32-bit xBGR format, [31:0] x:B:G:R 8:8:8:8 little endian
    Xbgr8888 = 0x34324258,
    /// 32-bit RGBx format, [31:0] R:G:B:x 8:8:8:8 little endian
    Rgbx8888 = 0x34325852,
    /// 32-bit BGRx format, [31:0] B:G:R:x 8:8:8:8 little endian
    Bgrx8888 = 0x34325842,
    /// 32-bit ABGR format, [31:0] A:B:G:R 8:8:8:8 little endian
    Abgr8888 = 0x34324241,
    /// 32-bit RGBA format, [31:0] R:G:B:A 8:8:8:8 little endian
    Rgba8888 = 0x34324152,
    /// 32-bit BGRA format, [31:0] B:G:R:A 8:8:8:8 little endian
    Bgra8888 = 0x34324142,
    /// 32-bit xRGB format, [31:0] x:R:G:B 2:10:10:10 little endian
    Xrgb2101010 = 0x30335258,
    /// 32-bit xBGR format, [31:0] x:B:G:R 2:10:10:10 little endian
    Xbgr2101010 = 0x30334258,
    /// 32-bit RGBx format, [31:0] R:G:B:x 10:10:10:2 little endian
    Rgbx1010102 = 0x30335852,
    /// 32-bit BGRx format, [31:0] B:G:R:x 10:10:10:2 little endian
    Bgrx1010102 = 0x30335842,
    /// 32-bit ARGB format, [31:0] A:R:G:B 2:10:10:10 little endian
    Argb2101010 = 0x30335241,
    /// 32-bit ABGR format, [31:0] A:B:G:R 2:10:10:10 little endian
    Abgr2101010 = 0x30334241,
    /// 32-bit RGBA format, [31:0] R:G:B:A 10:10:10:2 little endian
    Rgba1010102 = 0x30334152,
    /// 32-bit BGRA format, [31:0] B:G:R:A 10:10:10:2 little endian
    Bgra1010102 = 0x30334142,
    /// packed YCbCr format, [31:0] Cr0:Y1:Cb0:Y0 8:8:8:8 little endian
    Yuyv = 0x56595559,
    /// packed YCbCr format, [31:0] Cb0:Y1:Cr0:Y0 8:8:8:8 little endian
    Yvyu = 0x55595659,
    /// packed YCbCr format, [31:0] Y1:Cr0:Y0:Cb0 8:8:8:8 little endian
    Uyvy = 0x59565955,
    /// packed YCbCr format, [31:0] Y1:Cb0:Y0:Cr0 8:8:8:8 little endian
    Vyuy = 0x59555956,
    /// packed AYCbCr format, [31:0] A:Y:Cb:Cr 8:8:8:8 little endian
    Ayuv = 0x56555941,
    /// 2 plane YCbCr Cr:Cb format, 2x2 subsampled Cr:Cb plane
    Nv12 = 0x3231564e,
    /// 2 plane YCbCr Cb:Cr format, 2x2 subsampled Cb:Cr plane
    Nv21 = 0x3132564e,
    /// 2 plane YCbCr Cr:Cb format, 2x1 subsampled Cr:Cb plane
    Nv16 = 0x3631564e,
    /// 2 plane YCbCr Cb:Cr format, 2x1 subsampled Cb:Cr plane
    Nv61 = 0x3136564e,
    /// 3 plane YCbCr format, 4x4 subsampled Cb (1) and Cr (2) planes
    Yuv410 = 0x39565559,
    /// 3 plane YCbCr format, 4x4 subsampled Cr (1) and Cb (2) planes
    Yvu410 = 0x39555659,
    /// 3 plane YCbCr format, 4x1 subsampled Cb (1) and Cr (2) planes
    Yuv411 = 0x31315559,
    /// 3 plane YCbCr format, 4x1 subsampled Cr (1) and Cb (2) planes
    Yvu411 = 0x31315659,
    /// 3 plane YCbCr format, 2x2 subsampled Cb (1) and Cr (2) planes
    Yuv420 = 0x32315559,
    /// 3 plane YCbCr format, 2x2 subsampled Cr (1) and Cb (2) planes
    Yvu420 = 0x32315659,
    /// 3 plane YCbCr format, 2x1 subsampled Cb (1) and Cr (2) planes
    Yuv422 = 0x36315559,
    /// 3 plane YCbCr format, 2x1 subsampled Cr (1) and Cb (2) planes
    Yvu422 = 0x36315659,
    /// 3 plane YCbCr format, non-subsampled Cb (1) and Cr (2) planes
    Yuv444 = 0x34325559,
    /// 3 plane YCbCr format, non-subsampled Cr (1) and Cb (2) planes
    Yvu444 = 0x34325659,
    /// [7:0] R
    R8 = 0x20203852,
    /// [15:0] R little endian
    R16 = 0x20363152,
    /// [15:0] R:G 8:8 little endian
    Rg88 = 0x38384752,
    /// [15:0] G:R 8:8 little endian
    Gr88 = 0x38385247,
    /// [31:0] R:G 16:16 little endian
    Rg1616 = 0x32334752,
    /// [31:0] G:R 16:16 little endian
    Gr1616 = 0x32335247,
    /// [63:0] x:R:G:B 16:16:16:16 little endian
    Xrgb16161616F = 0x48345258,
    /// [63:0] x:B:G:R 16:16:16:16 little endian
    Xbgr16161616F = 0x48344258,
    /// [63:0] A:R:G:B 16:16:16:16 little endian
    Argb16161616F = 0x48345241,
    /// [63:0] A:B:G:R 16:16:16:16 little endian
    Abgr16161616F = 0x48344241,
    /// [31:0] X:Y:Cb:Cr 8:8:8:8 little endian
    Xyuv8888 = 0x56555958,
    /// [23:0] Cr:Cb:Y 8:8:8 little endian
    Vuy888 = 0x34325556,
    /// Y followed by U then V, 10:10:10. Non-linear modifier only
    Vuy101010 = 0x30335556,
    /// [63:0] Cr0:0:Y1:0:Cb0:0:Y0:0 10:6:10:6:10:6:10:6 little endian per 2 Y pixels
    Y210 = 0x30313259,
    /// [63:0] Cr0:0:Y1:0:Cb0:0:Y0:0 12:4:12:4:12:4:12:4 little endian per 2 Y pixels
    Y212 = 0x32313259,
    /// [63:0] Cr0:Y1:Cb0:Y0 16:16:16:16 little endian per 2 Y pixels
    Y216 = 0x36313259,
    /// [31:0] A:Cr:Y:Cb 2:10:10:10 little endian
    Y410 = 0x30313459,
    /// [63:0] A:0:Cr:0:Y:0:Cb:0 12:4:12:4:12:4:12:4 little endian
    Y412 = 0x32313459,
    /// [63:0] A:Cr:Y:Cb 16:16:16:16 little endian
    Y416 = 0x36313459,
    /// [31:0] X:Cr:Y:Cb 2:10:10:10 little endian
    Xvyu2101010 = 0x30335658,
    /// [63:0] X:0:Cr:0:Y:0:Cb:0 12:4:12:4:12:4:12:4 little endian
    Xvyu12_16161616 = 0x36335658,
    /// [63:0] X:Cr:Y:Cb 16:16:16:16 little endian
    Xvyu16161616 = 0x38345658,
    /// [63:0]   A3:A2:Y3:0:Cr0:0:Y2:0:A1:A0:Y1:0:Cb0:0:Y0:0  1:1:8:2:8:2:8:2:1:1:8:2:8:2:8:2 little endian
    Y0L0 = 0x304c3059,
    /// [63:0]   X3:X2:Y3:0:Cr0:0:Y2:0:X1:X0:Y1:0:Cb0:0:Y0:0  1:1:8:2:8:2:8:2:1:1:8:2:8:2:8:2 little endian
    X0L0 = 0x304c3058,
    /// [63:0]   A3:A2:Y3:Cr0:Y2:A1:A0:Y1:Cb0:Y0  1:1:10:10:10:1:1:10:10:10 little endian
    Y0L2 = 0x324c3059,
    /// [63:0]   X3:X2:Y3:Cr0:Y2:X1:X0:Y1:Cb0:Y0  1:1:10:10:10:1:1:10:10:10 little endian
    X0L2 = 0x324c3058,
    Yuv420_8Bit = 0x38305559,
    Yuv420_10Bit = 0x30315559,
    Xrgb8888A8 = 0x38415258,
    Xbgr8888A8 = 0x38414258,
    Rgbx8888A8 = 0x38415852,
    Bgrx8888A8 = 0x38415842,
    Rgb888A8 = 0x38413852,
    Bgr888A8 = 0x38413842,
    Rgb565A8 = 0x38413552,
    Bgr565A8 = 0x38413542,
    /// non-subsampled Cr:Cb plane
    Nv24 = 0x3432564e,
    /// non-subsampled Cb:Cr plane
    Nv42 = 0x3234564e,
    /// 2x1 subsampled Cr:Cb plane, 10 bit per channel
    P210 = 0x30313250,
    /// 2x2 subsampled Cr:Cb plane 10 bits per channel
    P010 = 0x30313050,
    /// 2x2 subsampled Cr:Cb plane 12 bits per channel
    P012 = 0x32313050,
    /// 2x2 subsampled Cr:Cb plane 16 bits per channel
    P016 = 0x36313050,
    /// [63:0] A:x:B:x:G:x:R:x 10:6:10:6:10:6:10:6 little endian
    Axbxgxrx106106106106 = 0x30314241,
    /// 2x2 subsampled Cr:Cb plane
    Nv15 = 0x3531564e,
    Q410 = 0x30313451,
    Q401 = 0x31303451,
    /// [63:0] x:R:G:B 16:16:16:16 little endian
    Xrgb16161616 = 0x38345258,
    /// [63:0] x:B:G:R 16:16:16:16 little endian
    Xbgr16161616 = 0x38344258,
    /// [63:0] A:R:G:B 16:16:16:16 little endian
    Argb16161616 = 0x38345241,
    /// [63:0] A:B:G:R 16:16:16:16 little endian
    Abgr16161616 = 0x38344241,
}

pub const WL_SHM_CREATE_POOL: u32 = 0;
pub const WL_SHM_FORMAT_SINCE_VERSION: u32 = 1;
pub const WL_SHM_CREATE_POOL_SINCE_VERSION: u32 = 1;

#[repr(C)]
pub struct WlShm([u8; 0]);

#[repr(C)]
pub struct WlShmListener {
    pub done: Option<
        unsafe extern "C" fn(data: *mut c_void, wl_shm: *mut WlShm, format: u32),
    >,
}

#[allow(clippy::missing_safety_doc)]
impl LibWaylandClient {
    pub unsafe fn wl_shm_add_listener(
        &self,
        wl_shm: *mut WlShm,
        listener: *const WlShmListener,
        data: *mut c_void,
    ) -> c_int {
        (self.wl_proxy_add_listener)(wl_shm as _, listener as _, data)
    }

    pub unsafe fn wl_shm_set_user_data(
        &self,
        wl_shm: *mut WlShm,
        user_data: *mut c_void,
    ) {
        (self.wl_proxy_set_user_data)(wl_shm as _, user_data)
    }

    pub unsafe fn wl_shm_get_user_data(&self, wl_shm: *mut WlShm) -> *mut c_void {
        (self.wl_proxy_get_user_data)(wl_shm as _)
    }

    pub unsafe fn wl_shm_get_version(&self, wl_shm: *mut WlShm) -> u32 {
        (self.wl_proxy_get_version)(wl_shm as _)
    }

    pub unsafe fn wl_shm_destroy(&self, wl_shm: *mut WlShm) {
        (self.wl_proxy_destroy)(wl_shm as _)
    }

	pub unsafe fn wl_shm_create_pool(&self, wl_shm: *mut WlShm, fd: i32, size: i32) -> *mut WlShmPool {
		(self.wl_proxy_marshal_flags)(wl_shm as _, WL_SHM_CREATE_POOL, self.wl_shm_pool_interface, (self.wl_proxy_get_version)(wl_shm as _), 0, NULLPTR, fd, size) as _
	}
}
