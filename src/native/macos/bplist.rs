//! A parser for Apple's Binary Plist (called BPlist here) format. Cursors store their metadata in a bplist file,
//! so this parser is used to load that. Why Apple refuses to just provide their default cursors in the `Cursor`
//! class is a mystery.
//!
//! Technically, Lokinit couldn't used FFI calls to create an `NSDictionary` and load it from the BPlist file
//! instead of writing the parser ourselves. However, the FFI calls to do this were annoying, and writing this
//! parser creates more code other people can look at in the future to understand the BPlist format.
//!
//! # How BPlist Works
//!
//! There's 4 concepts to understand in this format: The Signature, Objects, the Offset Table, and the Trailer.
//!
//! ## The Signature
//! This is pretty straighforward: Each file starts with `bplist00` to represent the file's format and version.
//! `00` represents the BPlist version the file uses - there are other versions, for example `bplist01`, but
//! we only need `00` to load cursors. I'm not sure what the differences are in other BPlist versions, but I think
//! those versions just add more object types (more info on objects below).
//!
//! ## Objects
//! An object is just a piece of data in the BPlist file. There are lots of types of objects, but they all follow
//! this general format in the actual binary:
//!
//! ```not_rust
//! 0101    0100    1001100    1101111    1101011    1101001
//! marker  size*   other data (these bytes are specific to each object)
//! ```
//!
//! Generally speaking, the first 4 bits denote an object's type, the next 4 bits denote its size in the file,
//! and any remaining bytes (up to the object's size) store any other data about it. In the object above, `0101`
//! denotes an ASCII string, and `0100` indicates it contains 4 characters (which decode to "Loki" :).
//!
//! Of course, Apple is about as consistant as a non-cryptographic RNG: While patterns do technically exist, they're rare.
//! For the null, boolean, and fill types, the first four bits are all 0s, and the remaining 4 bits of the first byte
//! *actually* denote the type (hence the * beside size above). Null is a null byte; booleans are `0000 1000` for false,
//! and `0000 1001` for true; the fill byte is `0000 1111`. Why the fill byte exists is unknown, as it seems to serve
//! no purpose, and the name also indicates it has no purpose.
//!
//! There are two other concepts you need to understand to grasp BPlist objects: Integers, and objects that store objects.
//! The first is important because it's also how other types store their size; the second introduces a new concept called
//! object references. I'll explain integers here and nested objects in the offset table section below.
//!
//! Integers have the signature `0001`, then a 4 bit number we'll call `x`. The integer is `2^x` bytes long: So, for example,
//! `00010001` is an integer where `x` is 1. `2^1` is 2, so this integer is 2 bytes long, or an `i16`. Integers are always
//! signed and always stored big-endian.
//!
//! Variable-length objects (strings, arrays, dicts, etc) will encode their size in the lower 4-bits of the first byte, as
//! explained above. This has an obvious problem: they can't be longer than `1111` (15) bytes in length. To solve this,
//! objects can simply put `1111` as their length; when they do this, it means the next byte will be an integer object that
//! stores the object's *actual* length.
//!
//! I won't take the time to explain the other object types. There are more, but between this and the object reference
//! explanation below, they should be easy to figure out. I've included 3 links I found useful while figuring this format
//! out at the bottom of this documentation: you can read those for more information.
//!
//! ## The Offset Table
//! A giant list of offsets to various objects in the BPlist file. Objects that store other objects (dictionaries and arrays,
//! for example) will store an index into this table instead of storing the actual objects they contain. This acts like a
//! double pointer: If you follow that index into this table, you'll get an offset from the start of the file to an object.
//! If you follow that offset, you'll get the actual object that's being stored. This is called an object reference.
//!
//! ## The Trailer
//! This is the most important part of the file: a 32-byte structure that stores information needed to parse the
//! BPlist file. Logically, since it's so important, Apple put it at the end of the file; you have to skip to the
//! *last* 32 bytes to find this structure.
//!
//! The Trailer contains 5 fields, all of which are unsigned numbers:
//! 1. Offset size: a `u8` storing the size (in bytes) of one offset in the offset table. Usually 1.
//! 2. Object ref size: a `u8` storing the size (in bytes) of object references. Usually 1.
//! 3. Object count: a `u64` storing the total number of objects in this BPlist file.
//! 4. First object offset: a `u64` storing an index into the offset table. The offset at that index is an offset
//! to the first object in this BPlist file. You can think of it like a double-pointer to the first object in the file.
//! 5. Offset table's start: a `u64` storing an offset, relative to the start of the file, to the start of the offset
//! table.
//!
//! All of this information is necessary to (correctly) parse a BPlist file, so it's the first thing that needs to be
//! parsed, and Apple definitely should've put it at the start instead of the end. But whatever.
//!
//! # Sources/further reading:
//!
//! - https://opensource.apple.com/source/CF/CF-1153.18/CFBinaryPList.c.auto.html
//! - https://medium.com/@karaiskc/understanding-apples-binary-property-list-format-281e6da00dbd
//! - https://en.wikipedia.org/wiki/Property_list#Format

use std::collections::HashMap;

/// Similar to an Iterator in rust, except it can go both forwards and backwards and jump
/// to arbitrary locations.
pub struct Cursor {
    buffer: Vec<u8>,
    idx: usize,
}
impl Cursor {
    #[inline]
    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    pub fn advance_cursor(&mut self, offset: usize) -> Result<(), ()> {
        if let Some(idx) = self.idx.checked_add(offset) {
            if idx < self.len() {
                self.idx += offset;
                return Ok(());
            }
        }

        Err(())
    }
    pub fn retreat_cursor(&mut self, offset: usize) -> Result<(), ()> {
        if let Some(idx) = self.idx.checked_sub(offset) {
            self.idx -= offset;
            return Ok(());
        }

        Err(())
    }

    pub fn next(&mut self) -> Option<u8> {
        self.advance_cursor(1).ok()?;
        Some(self.current())
    }
    pub fn current(&self) -> u8 {
        self.buffer[self.idx]
    }
    pub fn prev(&mut self) -> Option<u8> {
        self.retreat_cursor(1).ok()?;
        Some(self.current())
    }

    pub fn next_n_slice(&mut self, amount: usize) -> Option<&[u8]> {
        let start = self.idx + 1;
        if self.advance_cursor(amount).is_ok() {
            let buf = &self.buffer[start..=self.idx];
            Some(buf)
        } else {
            None
        }
    }
    #[inline]
    pub fn next_n<const N: usize>(&mut self) -> Option<[u8; N]> {
        Some(self.next_n_slice(N)?.try_into().unwrap())
    }
    pub fn current_n<const N: usize>(&mut self) -> Option<[u8; N]> {
        let start = self.idx;
        if self.advance_cursor(N).is_ok() {
            let buf = &self.buffer[start..self.idx];
            Some(buf.try_into().unwrap())
        } else {
            None
        }
    }

    pub fn jump_to(&mut self, idx: usize) -> Result<(), ()> {
        if idx < self.buffer.len() {
            self.idx = idx;
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn remaining_values(&self) -> usize {
        self.len() - self.idx - 1
    }

    pub fn new(buffer: Vec<u8>) -> Self {
        Self { buffer, idx: 0 }
    }

    pub fn pos(&self) -> usize {
        self.idx
    }
}

pub struct BPlistParser {
    pub cursor: Cursor,

    // TRAILER DATA
    /// The size of each offset in the offset table
    pub offset_size: usize,
    /// Byte size of object references in arrays and dictionaries
    pub object_ref_size: usize,
    /// The number of encoded objects
    pub num_objects: usize,
    /// The byte in the offset table that gives the offset of the first object
    pub first_object_offset: usize,
    /// The location of the first byte of the offset table, relative to the start of the file
    pub offset_table_start: usize,
}
impl BPlistParser {
    /// Parses the file's trailer and creates a new parser, then calls `self.parse()`.
    pub fn new_and_parse(bytes: Vec<u8>) -> Vec<Object> {
        let mut cursor = Cursor::new(bytes);

        // The trailer is 32 bytes long. The first 4 bytes are unused, and I dunno how to use 5,
        // giving us 27 bytes. Then we offset by 1 more so we can call `current` instead of `next`.
        cursor.jump_to(cursor.len() - 26).unwrap();
        let offset_size = cursor.current() as usize;
        let object_ref_size = cursor.next().unwrap() as usize;
        let num_objects = u64::from_be_bytes(cursor.next_n().unwrap()) as usize;
        let first_object_offset = u64::from_be_bytes(cursor.next_n().unwrap()) as usize;
        let offset_table_start = u64::from_be_bytes(cursor.next_n().unwrap()) as usize;

        Self {
            cursor,
            offset_size,
            object_ref_size,
            num_objects,
            first_object_offset,
            offset_table_start,
        }
        .parse()
    }

    /// Parses the BPlist file.
    pub fn parse(mut self) -> Vec<Object> {
        let mut objects = Vec::new();

        // Go to the first object
        self.cursor
            .jump_to(self.offset_table_start + self.first_object_offset)
            .unwrap();
        self.follow_current_offset();

        // The object table's end, relative to the end of the file
        let offset_table_size = self.offset_size * self.num_objects;
        let object_table_end = offset_table_size + 32;

        while self.cursor.remaining_values() > object_table_end {
            objects.push(Object::deserialize(&mut self));
            self.cursor.advance_cursor(1).unwrap();
        }
        objects
    }

    /// Parses an integer that is `num_bytes` bytes in size. Will always convert the number to an
    /// i64, since 8-byte numbers can be negative.
    #[inline]
    pub fn parse_int(&mut self, num_bytes: usize) -> i64 {
        match num_bytes {
            1 => u8::from_be_bytes([self.cursor.current()]).into(),
            2 => u16::from_be_bytes(self.cursor.current_n().unwrap()).into(),
            4 => u32::from_be_bytes(self.cursor.current_n().unwrap()).into(),
            8 => i64::from_be_bytes(self.cursor.current_n().unwrap()),
            _ => panic!("Unsupported int byte size: {num_bytes} bytes"),
        }
    }

    /// Assumes the cursor is currently pointing to an offset in the offset table.
    /// Reads that offset and moves the cursor to the object that offset points to.
    /// The only reason this has its own function is the offsets can span one or multiple bytes.
    #[inline]
    pub fn follow_current_offset(&mut self) {
        let pos = self.parse_int(self.offset_size) as usize;
        self.cursor
            .jump_to(pos)
            .map_err(|_| format!("Failed to follow current offset, pos was {pos} ({pos:#x})"))
            .unwrap();
    }

    /// Assumes the cursor is currently pointing to an object ref in the object table.
    /// Reads that object ref, follows it to the offset table, then follows that to the actual object.
    /// The only reason this has its own function is the object refs can span one or multiple bytes.
    #[inline]
    pub fn follow_current_object_ref(&mut self) {
        let offset_index = self.parse_int(self.object_ref_size) as usize;
        self.cursor
            .jump_to(self.offset_table_start + offset_index)
            .unwrap();

        self.follow_current_offset();
    }
}

#[derive(Debug)]
pub enum ObjectSignature {
    NullBoolFill, // These three all have the signature `0000`
    Int,
    Real,
    Date,
    Data,
    AsciiString,
    UnicodeString,
    UID,
    Array,
    Set,
    Dict,
}
impl TryFrom<u8> for ObjectSignature {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            0b0000_0000 => Self::NullBoolFill,
            0b0000_0001 => Self::Int,
            0b0000_0010 => Self::Real,
            0b0000_0011 => Self::Date,
            0b0000_0100 => Self::Data,
            0b0000_0101 => Self::AsciiString,
            0b0000_0110 => Self::UnicodeString,
            0b0000_1000 => Self::UID,
            0b0000_1010 => Self::Array,
            0b0000_1100 => Self::Set,
            0b0000_1101 => Self::Dict,
            _ => return Err(()),
        })
    }
}

#[derive(Debug)]
pub enum Object {
    Null,
    Bool(bool),
    // Note: ints can technically be variable-length, and could be larger than 8 bytes.
    // This, however, is a pain in the ass to deal with, so we don't.
    Int(i64),
    // Same as above.
    Real(f64),
    AsciiString(String),
    UnicodeString(String),
    Array(Vec<Self>),
    Dict(HashMap<String, Self>),
}
impl Object {
    /// Parses the lower 4 bits of the marker byte to find the number of bytes the current object
    /// takes up.
    pub fn num_object_bytes(ctx: &mut BPlistParser) -> usize {
        // Size is in the lower 4 bits
        let size = ctx.cursor.current() & 0b0000_1111;
        // We have to parse the next few bytes as an unsigned int if the size maxes out
        if size == 0b0000_1111 {
            ctx.cursor.advance_cursor(1).unwrap();

            Self::parse_int(ctx) as usize
        } else {
            size as usize
        }
    }

    pub fn parse_int(ctx: &mut BPlistParser) -> i64 {
        let num_bytes = 2usize.pow(Self::num_object_bytes(ctx) as u32);
        ctx.cursor.advance_cursor(1).unwrap();
        ctx.parse_int(num_bytes)
    }

    pub fn parse_float(ctx: &mut BPlistParser) -> f64 {
        let num_bytes = 2usize.pow(Self::num_object_bytes(ctx) as u32);
        ctx.cursor.advance_cursor(1).unwrap();

        match num_bytes {
            4 => f32::from_be_bytes(ctx.cursor.current_n().unwrap()) as f64,
            8 => f64::from_be_bytes(ctx.cursor.current_n().unwrap()),
            _ => panic!("Unsupported float byte size: {num_bytes} bytes"),
        }
    }

    pub fn parse_null_bool(ctx: &BPlistParser) -> Option<bool> {
        match ctx.cursor.current() {
            // Null
            0b0000_0000 => None,
            // Bool
            0b0000_1000 => Some(false),
            0b0000_1001 => Some(true),
            // Invalid
            b => panic!("Invalid byte {b:#b} ({b}) while trying to parse bool"),
        }
    }

    // I believe that, technically, any object can be used as a key for a dictionary.
    // However, it's annoying to have to hash those, and they're almost always string
    // values anyways (they are always string values in the case of the cursor files).
    // So, technically, this should return a `HashMap<Self, Self>`, but Lokinit doesn't handle
    // non-String cases.
    pub fn parse_dict(ctx: &mut BPlistParser) -> HashMap<String, Self> {
        let mut map = HashMap::default();
        let map_base = ctx.cursor.pos();
        let num_entries = Self::num_object_bytes(ctx);
        let mut entries_processed = 0;

        while entries_processed < num_entries {
            entries_processed += 1;
            ctx.cursor.jump_to(map_base + entries_processed).unwrap();
            ctx.follow_current_object_ref();
            let key_obj = Self::deserialize(ctx);
            let key = match key_obj {
                Self::AsciiString(key) => key,
                Self::UnicodeString(key) => key,
                other => panic!("Unsupported Dictionary key type: {other:?}"),
            };

            ctx.cursor
                .jump_to(map_base + entries_processed + num_entries)
                .unwrap();
            ctx.follow_current_object_ref();
            let value = Self::deserialize(ctx);

            map.insert(key, value);
        }

        map
    }

    pub fn parse_array(ctx: &mut BPlistParser) -> Vec<Self> {
        let mut array = Vec::new();
        let base = ctx.cursor.pos();
        let num_entries = Self::num_object_bytes(ctx);
        let mut entries_processed = 0;

        while entries_processed < num_entries {
            entries_processed += 1;
            ctx.cursor.jump_to(base + entries_processed).unwrap();
            ctx.follow_current_object_ref();
            array.push(Self::deserialize(ctx))
        }

        array
    }

    pub fn parse_string(ctx: &mut BPlistParser) -> String {
        let len = Self::num_object_bytes(ctx);
        let bytes = ctx.cursor.next_n_slice(len).unwrap().to_vec();
        String::from_utf8(bytes).unwrap()
    }

    pub fn deserialize(input: &mut BPlistParser) -> Self {
        let signature = input.cursor.current() >> 4;

        match ObjectSignature::try_from(signature).unwrap() {
            ObjectSignature::Int => Self::Int(Self::parse_int(input)),
            ObjectSignature::Real => Self::Real(Self::parse_float(input)),
            ObjectSignature::Dict => Self::Dict(Self::parse_dict(input)),
            ObjectSignature::AsciiString => Self::AsciiString(Self::parse_string(input)),
            ObjectSignature::UnicodeString => Self::UnicodeString(Self::parse_string(input)),
            ObjectSignature::Array => Self::Array(Self::parse_array(input)),
            ObjectSignature::NullBoolFill => match Self::parse_null_bool(input) {
                None => Self::Null,
                Some(val) => Self::Bool(val),
            },
            e => panic!("Unsupported object type {e:?}"),
        }
    }
}

// fn main() {
//     let file = include_bytes!(concat!(
//     "/System/Library/Frameworks/ApplicationServices.framework/Versions/A",
//     "/Frameworks/HIServices.framework/Versions/A/Resources/cursors/closedhand/info.plist"
//     ));
//     let bplist = Parser::<BPlist, _>::parse(file.to_vec()).unwrap();
//     println!("{:?}", bplist.objects);
// }
