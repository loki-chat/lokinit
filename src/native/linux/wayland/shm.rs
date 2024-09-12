use {
    loki_linux::wayland::{interfaces::all::*, methods::*, WaylandClient},
    std::{cmp::Ordering, sync::atomic::AtomicU16},
};

// TODO: Move to loki-linux (and maybe use `libc` for a lot of these)
mod libc {
    use std::{
        ffi::{c_char, c_int, c_uint},
        ops::{Deref, DerefMut},
        os::fd::RawFd,
    };

    const fn hugetlb(flag: c_uint) -> c_uint {
        flag << 26
    }

    pub const HUGETLB_FLAG_ENCODE_8MB: c_uint = hugetlb(23);
    pub const MFD_HUGETLB: c_uint = 0x0004;
    pub const MFD_HUGEPAGE: c_uint = 0x0008;
    pub const PROT_READ: c_int = 0x1;
    pub const PROT_WRITE: c_int = 0x2;
    pub const PROT_EXEC: c_int = 0x4;
    pub const PROT_NONE: c_int = 0;
    pub const MAP_SHARED: c_int = 0x01;
    pub const MAP_PRIVATE: c_int = 0x02;

    pub struct MemMap {
        fd: RawFd,
        ptr: *mut u8,
        len: usize,
    }
    impl MemMap {
        pub unsafe fn create(len: usize) -> std::io::Result<Self> {
            let mut fd = memfd_create(
                c"buffer".as_ptr(),
                MFD_HUGEPAGE | MFD_HUGETLB | HUGETLB_FLAG_ENCODE_8MB,
            );
            if fd == -1 {
                eprintln!("Failed to initialize memfd with huge pages");
                fd = memfd_create(c"buffer".as_ptr(), 0);
            }
            if fd == -1 {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::OutOfMemory,
                    "memfd_create failed",
                ));
            }
            if ftruncate(fd, len) == -1 {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::OutOfMemory,
                    "ftruncate failed",
                ));
            }
            Self::new(fd, len)
        }

        pub unsafe fn new(fd: RawFd, len: usize) -> std::io::Result<Self> {
            let ptr = mmap(
                std::ptr::null_mut(),
                len,
                PROT_WRITE | PROT_READ,
                MAP_SHARED,
                fd,
                0,
            );
            if ptr.is_null() {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "mmap failed",
                ));
            }
            Ok(MemMap { fd, ptr, len })
        }

        pub fn fd(&self) -> RawFd {
            self.fd
        }
    }
    impl Deref for MemMap {
        type Target = [u8];

        fn deref(&self) -> &Self::Target {
            unsafe { std::slice::from_raw_parts(self.ptr, self.len) }
        }
    }
    impl DerefMut for MemMap {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { std::slice::from_raw_parts_mut(self.ptr, self.len) }
        }
    }
    impl Drop for MemMap {
        fn drop(&mut self) {
            unsafe {
                munmap(self.ptr, self.len);
                close(self.fd);
            }
        }
    }

    extern "C" {
        fn memfd_create(name: *const c_char, flags: c_uint) -> RawFd;
        fn ftruncate(fd: RawFd, size: usize) -> c_int;
        fn mmap(
            ptr: *mut u8,
            size: usize,
            prot: c_int,
            flags: c_int,
            fd: RawFd,
            offset: u32,
        ) -> *mut u8;
        fn munmap(ptr: *mut u8, size: usize);
        fn close(fd: RawFd);
    }
}

#[derive(PartialEq, Eq)]
pub struct Range {
    pub start: u64,
    pub end: u64,
}
impl Range {
    pub fn is_empty(&self) -> bool {
        self.end == self.start
    }

    pub fn len(&self) -> u64 {
        self.end - self.start
    }
}
impl PartialOrd for Range {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Range {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.end < other.start {
            Ordering::Less
        } else if self.start > other.end {
            Ordering::Greater
        } else {
            unreachable!()
        }
    }
}

// TODO: Move in `loki-linux`
#[derive(Clone, Copy, Debug)]
pub enum Format {
    /// 32-bit ARGB format, [31:0] A:R:G:B 8:8:8:8 little endian
    Argb8888,
    /// 32-bit RGB format, [31:0] x:R:G:B 8:8:8:8 little endian
    Xrgb8888,
}
impl Format {
    pub fn stride(&self) -> i32 {
        4
    }
}
impl From<Format> for u32 {
    fn from(value: Format) -> Self {
        match value {
            Format::Argb8888 => 0,
            Format::Xrgb8888 => 1,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ImageInfo {
    width: i32,
    height: i32,
    offset: i32,
    stride: i32,
    format: Format,
}
impl ImageInfo {
    pub fn new(width: u32, height: u32, format: Format) -> Option<Self> {
        let width: i32 = width.try_into().ok()?;
        let height: i32 = height.try_into().ok()?;
        let stride = width.checked_mul(format.stride())?;

        Some(Self {
            width,
            height,
            offset: 0,
            stride,
            format,
        })
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn len(&self) -> usize {
        TryInto::<usize>::try_into(self.height).expect("height was negative")
            * TryInto::<usize>::try_into(self.stride).expect("stride was negative")
            + TryInto::<usize>::try_into(self.offset).expect("offset was negative")
    }
}

pub struct Buffer {
    wl_buffer: WlBuffer,
    pool_id: u16,
    ptr: usize,
}
impl Buffer {
    pub fn wl_buffer(&self) -> WlBuffer {
        self.wl_buffer
    }
}

struct Point {
    ptr: usize,
    len: usize,
}

static FREE_POOL_ID: AtomicU16 = AtomicU16::new(0);

pub struct ShmAllocator {
    //pub file: File,
    //pub pool: WlShmPool,
    //size: usize,
    //free_list: Vec<Range>,
    memmap: libc::MemMap,
    allocated: Vec<Point>,
    pool: WlShmPool,
    pool_id: u16,
}
impl ShmAllocator {
    pub fn new(size: usize, client: &mut WaylandClient) -> std::io::Result<Self> {
        //let shm_file_dir = env::var("XDG_RUNTIME_DIR")
        //    .ok()
        //    .unwrap_or("/tmp".to_string());
        //let file = create_tmpfile(OpenOptions::new().write(true).read(true))
        //    .open(shm_file_dir)
        //    .ok()?;

        //let shm: WlShm = client.get_global();
        //file.set_len(size).unwrap();
        //let pool = shm.create_pool(
        //    client,
        //    Fd {
        //        raw: file.as_raw_fd(),
        //    },
        //    size as _,
        //);

        //let free_list = vec![Range {
        //    start: 0,
        //    end: size,
        //}];

        //Some(Self {
        //    file,
        //    pool,
        //    size,
        //    free_list,
        //})

        let memmap = unsafe { libc::MemMap::create(size) }?;
        let shm: WlShm = client.get_global();
        let pool = shm.create_pool(
            client,
            memmap.fd().into(),
            size.try_into().expect("size conversion failed"),
        );

        Ok(Self {
            memmap,
            pool,
            allocated: Vec::with_capacity(64),
            pool_id: FREE_POOL_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
        })
    }

    pub fn allocate(
        &mut self,
        client: &mut WaylandClient,
        image_info: ImageInfo,
    ) -> std::io::Result<Buffer> {
        let len = image_info.len();

        if len == 0 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Cannot allocate ZSTs",
            ));
        }

        let mut ptr = 0usize;
        let mut i = 0usize;

        // 'allocated' is always sorted by ptr from lowest to highest.
        for point in self.allocated.iter() {
            if point.ptr - ptr > len {
                break;
            }
            ptr = point.ptr + point.len;
            i += 1;
        }

        if self.memmap.len() - ptr < len {
            return Err(std::io::Error::new(
                std::io::ErrorKind::OutOfMemory,
                "No more shm memory available",
            ));
        }

        let wl_buffer = self.pool.create_buffer(
            client,
            image_info.offset,
            image_info.width,
            image_info.height,
            image_info.stride,
            image_info.format.into(),
        );

        self.allocated.insert(i, Point { ptr, len });

        Ok(Buffer {
            wl_buffer,
            ptr,
            pool_id: self.pool_id,
        })
    }

    pub fn free(
        &mut self,
        client: &mut WaylandClient,
        buffer: Buffer,
    ) -> Result<(), (std::io::Error, Buffer)> {
        let Buffer {
            wl_buffer,
            ptr,
            pool_id,
        } = buffer;

        if pool_id != self.pool_id {
            return Err((
                std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "Trying to deallocate a foreign buffer",
                ),
                buffer,
            ));
        }

        let prior_len = self.allocated.len();
        self.allocated.retain(|x| x.ptr != ptr);
        match prior_len - self.allocated.len() {
            0 => Err((
                std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "Trying to deallocate a foreign buffer",
                ),
                buffer,
            )),
            1 => {
                client.call_method(&wl_buffer, WlBufferMethod::Destroy);
                Ok(())
            }
            x => panic!("Shm allocator error! Deallocated {x} objects!"),
        }
        // loop {
        //     let current = self.free_list.get_mut(idx).unwrap();
        //     match range.cmp(current) {
        //         Ordering::Less => {
        //             if let Some(next) = self.free_list.get(idx + 1) {
        //                 if next > &range {
        //                     if current.end + 1 == range.start && range.end + 1 == next.start {
        //                         next.start = current.start;
        //                         self.free_list.remove(idx);
        //                     } else if current.end + 1 == range.start {
        //                         current.end = range.end;
        //                     } else if range.end + 1 == next.start {
        //                         next.start = range.start;
        //                     } else {
        //                         self.free_list.insert(idx + 1, range);
        //                     }
        //                 }
        //             } else if current.end == range.start - 1 {
        //                 current.end = range.end;
        //             } else {
        //                 self.free_list.push(range);
        //             }
        //         }
        //         Ordering::Greater => {
        //             if let Some(prev) = self.free_list.get(idx.saturating_sub(1)) {
        //                 if prev < &range {
        //                     if prev.end + 1 == range.start && range.end + 1 == current.start {
        //                         prev.end = current.end;
        //                         self.free_list.remove(idx);
        //                     } else if prev.end + 1 == range.start {
        //                         prev.end = range.end;
        //                     } else if range.end + 1 == current.start {
        //                         current.start = range.start;
        //                     } else {
        //                         self.free_list.insert(idx, range)
        //                     }
        //                 }
        //             } else if range.end + 1 == current.start {
        //                 current.start = range.start;
        //             } else {
        //                 self.free_list.push(range);
        //             }
        //         }
        //         Ordering::Equal => unreachable!(),
        //     }
        // }
    }
}
