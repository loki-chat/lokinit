//! An implementation of Wayland's wire protocol.
//! Implemented from https://wayland.freedesktop.org/docs/html/ch04.html#sect-Protocol-Wire-Format.

use std::{ffi::CStr, io::Write, mem, os::unix::net::UnixStream};

use super::Object;

pub trait WireType {
    fn to_wire(&self, vec: &mut Vec<u8>);
    fn from_wire(data: &[u32]) -> (Self, usize)
    where
        Self: Sized;
    fn size(&self) -> u16;
}
/// The `uint` type.
impl WireType for u32 {
    fn to_wire(&self, vec: &mut Vec<u8>) {
        vec.extend(self.to_ne_bytes());
    }
    fn from_wire(data: &[u32]) -> (Self, usize) {
        (data[0], 1)
    }
    fn size(&self) -> u16 {
        4
    }
}
/// The `int` type.
impl WireType for i32 {
    fn to_wire(&self, vec: &mut Vec<u8>) {
        vec.extend(self.to_ne_bytes());
    }
    fn from_wire(data: &[u32]) -> (Self, usize) {
        (unsafe { mem::transmute(data[0]) }, 1)
    }
    fn size(&self) -> u16 {
        4
    }
}
/// The `fixed` type.
impl WireType for f32 {
    fn to_wire(&self, vec: &mut Vec<u8>) {
        vec.extend(self.to_ne_bytes());
    }
    fn from_wire(data: &[u32]) -> (Self, usize) {
        (f32::from_bits(data[0]), 1)
    }
    fn size(&self) -> u16 {
        4
    }
}
/// The `string` type.
impl WireType for &str {
    fn to_wire(&self, vec: &mut Vec<u8>) {
        let len = (self.len() + 1) as u32;
        len.to_wire(vec);
        vec.extend_from_slice(self.as_bytes());
        vec.push(0);

        // Must align to 32-bits
        if len % 4 != 0 {
            let alignment_padding_size = 4 - (len % 4);
            vec.resize(vec.len() + alignment_padding_size as usize, 0);
        }
    }
    fn from_wire(data: &[u32]) -> (Self, usize) {
        let len = data[0].div_ceil(4) + 1;
        let c_str = unsafe { CStr::from_ptr(&data[1..] as *const [u32] as *const _) };
        (
            c_str.to_str().expect("Error: Wayland string was not UTF-8"),
            len as usize,
        )
    }
    fn size(&self) -> u16 {
        4 + self.len() as u16 + 1
    }
}
/// The nullable `string` type.
impl WireType for Option<&str> {
    fn to_wire(&self, vec: &mut Vec<u8>) {
        match self {
            Some(string) => string.to_wire(vec),
            None => vec.push(0),
        }
    }
    fn from_wire(data: &[u32]) -> (Self, usize) {
        let len = data[0];

        if len == 0 {
            (None, 1)
        } else {
            let data = <&str>::from_wire(data);
            (Some(data.0), data.1)
        }
    }
    fn size(&self) -> u16 {
        match self {
            Some(string) => string.size(),
            None => 4,
        }
    }
}
/// The `object` type.
impl<O: Object> WireType for O {
    fn to_wire(&self, vec: &mut Vec<u8>) {
        self.id().to_wire(vec);
    }
    fn from_wire(data: &[u32]) -> (Self, usize) {
        unreachable!("Decoding objects from wire")
    }
    fn size(&self) -> u16 {
        4
    }
}
/// The nullable `object` type.
impl<I: Object> WireType for Option<I> {
    fn to_wire(&self, vec: &mut Vec<u8>) {
        match self {
            Some(object) => object.to_wire(vec),
            None => vec.push(0),
        }
    }
    fn from_wire(data: &[u32]) -> (Self, usize) {
        unreachable!("Decoding objects from wire")
    }
    fn size(&self) -> u16 {
        4
    }
}
/// The `array` type.
// impl<T: WireType> WireType for &[T] {
//     fn write_wire(&self, socket: &mut UnixStream) {
//         let size = self.iter().map(|val| val.size()).sum::<u16>() as u32;
//         socket.write_all(size.to_ne_bytes().as_ref()).unwrap();
//         self.iter().for_each(|val| val.write_wire(socket));
//         // Must align to 32-bits
//         for _ in 0..size % 4 {
//             socket.write_all(&[0]).unwrap();
//         }
//     }
//     fn from_wire(data: &[u32]) -> (Self, usize) {
//         todo!()
//     }
//     fn size(&self) -> u16 {
//         self.iter().map(|val| val.size()).sum::<u16>() + 32
//     }
// }

/// Wire's `new_id` type.
pub struct NewId<'a, O: Object>(pub &'a O);
impl<'a, O: Object> WireType for NewId<'a, O> {
    fn to_wire(&self, vec: &mut Vec<u8>) {
        self.0.interface().to_wire(vec);
        self.0.version().to_wire(vec);
        self.0.to_wire(vec);
    }
    fn from_wire(data: &[u32]) -> (Self, usize)
    where
        Self: Sized,
    {
        todo!()
    }
    fn size(&self) -> u16 {
        self.0.interface().size() + self.0.version().size() + self.0.id().size()
    }
}

/// The header for the wire format.
#[derive(Debug)]
pub struct MessageHeader {
    pub object: u32,
    pub opcode: u16,
    pub message_size: u16,
}
impl MessageHeader {
    pub fn to_bytes(self) -> [u8; 8] {
        self.into()
    }
}
impl From<[u8; 8]> for MessageHeader {
    fn from(value: [u8; 8]) -> Self {
        Self {
            object: u32::from_ne_bytes([value[0], value[1], value[2], value[3]]),
            opcode: u16::from_ne_bytes([value[4], value[5]]),
            message_size: u16::from_ne_bytes([value[6], value[7]]),
        }
    }
}
impl From<MessageHeader> for [u8; 8] {
    fn from(value: MessageHeader) -> Self {
        let object = value.object.to_ne_bytes();
        let opcode = value.opcode.to_ne_bytes();
        let message_size = value.message_size.to_ne_bytes();

        [
            object[0],
            object[1],
            object[2],
            object[3],
            opcode[0],
            opcode[1],
            message_size[0],
            message_size[1],
        ]
    }
}

#[cfg(test)]
mod tests {
    use crate::wayland::{wire::NewId, Object};

    use super::WireType;

    fn to_wire<W: WireType>(w: W) -> Vec<u8> {
        let mut vec = Vec::new();
        w.to_wire(&mut vec);
        vec
    }

    #[test]
    fn nums() {
        assert_eq!(to_wire(0_u32), 0_u32.to_ne_bytes());
        assert_eq!(to_wire(15_u32), 15_u32.to_ne_bytes());
    }

    #[test]
    fn string() {
        let val = "stringy string";

        let len = val.len() + 1;
        let mut expected = (len as u32).to_ne_bytes().to_vec();
        expected.extend(b"stringy string\0");
        expected.push(0);

        assert!(expected.len() % 4 == 0);
        assert_eq!(to_wire(val), expected);
    }

    #[test]
    fn new_id() {
        let object = &VeryRealObject {};

        let mut expected = to_wire("wl_object");
        expected.extend(1_u32.to_ne_bytes());
        expected.extend(1_u32.to_ne_bytes());

        println!("{expected:?}");
        assert_eq!(to_wire(NewId(object)), expected);
    }

    struct VeryRealObject {}
    impl Object for VeryRealObject {
        fn id(&self) -> u32 {
            1
        }

        fn interface(&self) -> &'static str {
            "wl_object"
        }
        fn version(&self) -> u32 {
            1
        }
    }
}
