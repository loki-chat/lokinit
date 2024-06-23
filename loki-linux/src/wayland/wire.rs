//! An implementation of Wayland's wire protocol.
//! Implemented from https://wayland.freedesktop.org/docs/html/ch04.html#sect-Protocol-Wire-Format.

use std::{cell::Cell, ffi::CStr, fmt::Display, os::fd::RawFd};

use super::{interfaces::Interface, Object};

/// Types that can be serialized as wire.
pub trait WriteWire {
    fn write_wire(&self, encoder: &mut WireEncoder);
}
/// Types that can deserialize from wire.
pub trait ReadWire<'a>: Sized {
    fn read_wire(data: &'a [u8]) -> (Self, usize);
}

/// Decodes a Wire message.
pub struct WireDecoder<'a> {
    bytes: &'a [u8],
    idx: Cell<usize>,
}
impl<'a> WireDecoder<'a> {
    pub fn new(vec: &'a [u8]) -> Self {
        Self {
            bytes: vec,
            idx: Cell::new(8),
        }
    }

    pub fn object_id(&self) -> Id {
        Id {
            raw: u32::from_ne_bytes([self.bytes[0], self.bytes[1], self.bytes[2], self.bytes[3]]),
        }
    }
    pub fn opcode(&self) -> u16 {
        u16::from_ne_bytes([self.bytes[4], self.bytes[5]])
    }
    pub fn len(&self) -> u16 {
        u16::from_ne_bytes([self.bytes[6], self.bytes[7]])
    }
    pub fn is_empty(&self) -> bool {
        // Message header is 8 bytes, so it's the min message size
        self.len() == 8
    }
    pub fn decode<T: ReadWire<'a>>(&self) -> T {
        let (result, used_u8s) = T::read_wire(&self.bytes[self.idx.get()..]);
        self.idx.set(self.idx.get() + used_u8s);

        result
    }
}

/// Encodes a wire message.
pub struct WireEncoder {
    pub vec: Vec<u8>,
    pub fd: Option<RawFd>,
}
impl WireEncoder {
    pub fn new(obj: Id, opcode: u16) -> Self {
        let mut vec = Vec::with_capacity(8);

        // First 4 bytes: Object ID
        vec.extend(obj.raw.to_ne_bytes());
        // Next 2 bytes: Opcode
        vec.extend(opcode.to_ne_bytes());
        // Next 2 bytes: Message len (set in .finish())
        vec.push(0);
        vec.push(0);

        Self { vec, fd: None }
    }
    pub fn finish(mut self) -> (Vec<u8>, Option<RawFd>) {
        let len = (self.vec.len() as u16).to_ne_bytes();
        println!("Final wire msg len: {}", self.vec.len());
        self.vec[6] = len[0];
        self.vec[7] = len[1];

        (self.vec, self.fd)
    }
}

// int

impl<'a> ReadWire<'a> for i32 {
    fn read_wire(data: &'a [u8]) -> (Self, usize) {
        (Self::from_ne_bytes([data[0], data[1], data[2], data[3]]), 4)
    }
}
impl WriteWire for i32 {
    fn write_wire(&self, encoder: &mut WireEncoder) {
        encoder.vec.extend(self.to_ne_bytes());
    }
}

// uint

impl ReadWire<'_> for u32 {
    fn read_wire(data: &'_ [u8]) -> (Self, usize) {
        (Self::from_ne_bytes([data[0], data[1], data[2], data[3]]), 4)
    }
}
impl WriteWire for u32 {
    fn write_wire(&self, encoder: &mut WireEncoder) {
        encoder.vec.extend(self.to_ne_bytes())
    }
}

// fixed

// TODO: implement fixed decimal point functionalities
#[derive(Clone, Copy)]
#[allow(dead_code)]
pub struct Fixed(i32);

impl<'a> ReadWire<'a> for Fixed {
    fn read_wire(_data: &'a [u8]) -> (Self, usize) {
        todo!()
    }
}
impl WriteWire for Fixed {
    fn write_wire(&self, _encoder: &mut WireEncoder) {
        todo!()
    }
}

// string

impl<'a> ReadWire<'a> for &'a str {
    fn read_wire(data: &'a [u8]) -> (Self, usize) {
        let len = u32::from_ne_bytes([data[0], data[1], data[2], data[3]]);

        if len == 0 {
            return ("", 4);
        }

        let c_str = unsafe { CStr::from_ptr(&data[4..] as *const [u8] as *const _) };
        (
            c_str
                .to_str()
                .expect("Lokinit error: Wayland string was not UTF-8"),
            (len + 4) as usize,
        )
    }
}
impl<'a> WriteWire for &'a str {
    fn write_wire(&self, encoder: &mut WireEncoder) {
        let len = (self.len() + 1) as u32;
        len.write_wire(encoder);

        let mut bytes = self.as_bytes().iter().cloned().peekable();
        while bytes.peek().is_some() {
            encoder.vec.extend([
                bytes.next().unwrap_or(0),
                bytes.next().unwrap_or(0),
                bytes.next().unwrap_or(0),
                bytes.next().unwrap_or(0),
            ])
        }
        if self.len() % 4 == 0 {
            encoder.vec.extend(0u32.to_ne_bytes());
        }
    }
}
impl ReadWire<'_> for String {
    fn read_wire(data: &'_ [u8]) -> (Self, usize) {
        let (str, used_bytes) = <&str>::read_wire(data);

        (str.to_string(), used_bytes)
    }
}
impl WriteWire for String {
    fn write_wire(&self, encoder: &mut WireEncoder) {
        self.as_str().write_wire(encoder)
    }
}

// object

impl<O: Object> WriteWire for O {
    fn write_wire(&self, encoder: &mut WireEncoder) {
        self.id().write_wire(encoder);
    }
}
impl<O: Object> ReadWire<'_> for O {
    fn read_wire(data: &[u8]) -> (Self, usize) {
        let (id, bytes) = u32::read_wire(data);

        (Self::new_with_id(Id { raw: id }), bytes)
    }
}
impl<O: Object> WriteWire for Option<O> {
    fn write_wire(&self, encoder: &mut WireEncoder) {
        match self {
            Some(obj) => obj.write_wire(encoder),
            None => 0_u32.write_wire(encoder),
        }
    }
}
impl WriteWire for Option<Id> {
    fn write_wire(&self, encoder: &mut WireEncoder) {
        self.unwrap_or(Id { raw: 0 }).write_wire(encoder);
    }
}

/// A file descriptor.
#[derive(PartialEq, Eq, Clone, Copy)]
#[repr(transparent)]
pub struct Fd {
    pub raw: RawFd,
}
impl From<RawFd> for Fd {
    fn from(value: RawFd) -> Self {
        Self { raw: value }
    }
}
impl From<Fd> for RawFd {
    fn from(value: Fd) -> Self {
        value.raw
    }
}
impl WriteWire for Fd {
    fn write_wire(&self, encoder: &mut WireEncoder) {
        encoder.fd = Some(self.raw);
    }
}

/// A 32-bit unique identifier for a global singleton.
#[derive(Hash, PartialEq, Eq, Clone, Copy)]
#[repr(transparent)]
pub struct Name {
    pub raw: u32,
}
impl From<u32> for Name {
    fn from(value: u32) -> Self {
        Self { raw: value }
    }
}
impl From<Name> for u32 {
    fn from(value: Name) -> Self {
        value.raw
    }
}
impl WriteWire for Name {
    fn write_wire(&self, encoder: &mut WireEncoder) {
        self.raw.write_wire(encoder);
    }
}
impl ReadWire<'_> for Name {
    fn read_wire(data: &[u8]) -> (Self, usize) {
        let (raw, len) = u32::read_wire(data);
        (Self { raw }, len)
    }
}
impl Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.raw)
    }
}

/// A 32-bit unique identifier for an object.
#[derive(Hash, PartialEq, Eq, Clone, Copy)]
#[repr(transparent)]
pub struct Id {
    pub raw: u32,
}
impl From<u32> for Id {
    fn from(value: u32) -> Self {
        Self { raw: value }
    }
}
impl From<Id> for u32 {
    fn from(value: Id) -> Self {
        value.raw
    }
}
impl WriteWire for Id {
    fn write_wire(&self, encoder: &mut WireEncoder) {
        self.raw.write_wire(encoder);
    }
}
impl ReadWire<'_> for Id {
    fn read_wire(data: &[u8]) -> (Self, usize) {
        let (name, len) = u32::read_wire(data);
        (Self { raw: name }, len)
    }
}
impl Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.raw)
    }
}

pub struct NewId {
    pub interface: Interface,
    pub version: u32,
    pub id: Id,
}
impl WriteWire for NewId {
    fn write_wire(&self, encoder: &mut WireEncoder) {
        self.interface.to_string().write_wire(encoder);
        self.version.write_wire(encoder);
        self.id.write_wire(encoder);
    }
}
