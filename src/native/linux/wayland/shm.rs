use {
    loki_linux::wayland::{
        create_tmpfile, interfaces::all::*, methods::*, wire::Fd, WaylandClient,
    },
    std::{
        cmp::Ordering,
        collections::{BTreeSet, LinkedList},
        env,
        fs::{File, OpenOptions},
        ops::Index,
        os::fd::AsRawFd,
    },
};

#[derive(PartialEq, Eq)]
pub struct Range {
    pub start: u64,
    pub end: u64,
}
impl Range {
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

pub struct Buffer {
    wl_buffer: WlBuffer,
    range: Range,
}
impl Buffer {
    pub fn wl_buffer(&self) -> WlBuffer {
        self.wl_buffer
    }
}

pub struct ShmAllocator {
    pub file: File,
    pub pool: WlShmPool,
    size: u64,
    free_list: Vec<Range>,
}
impl ShmAllocator {
    pub fn new(size: u64, client: &mut WaylandClient) -> Option<Self> {
        let shm_file_dir = env::var("XDG_RUNTIME_DIR")
            .ok()
            .unwrap_or("/tmp".to_string());
        let file = create_tmpfile(OpenOptions::new().write(true).read(true))
            .open(shm_file_dir)
            .ok()?;

        let shm: WlShm = client.get_global();
        file.set_len(size).unwrap();
        let pool = shm.create_pool(
            client,
            Fd {
                raw: file.as_raw_fd(),
            },
            size as _,
        );

        let free_list = vec![Range {
            start: 0,
            end: size,
        }];

        Some(Self {
            file,
            pool,
            size,
            free_list,
        })
    }

    pub fn allocate(&mut self, client: &WaylandClient, size: u64) -> Buffer {
        todo!()
    }

    pub fn free(&mut self, client: &WaylandClient, buffer: Buffer) {
        let Buffer { wl_buffer, range } = buffer;
        client.call_method(&wl_buffer, WlBufferMethod::Destroy);

        let mut idx = self.free_list.len().div_ceil(2);
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
