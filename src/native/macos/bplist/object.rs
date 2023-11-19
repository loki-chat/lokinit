use super::Context;
use std::collections::HashMap;
use syntax::prelude::*;

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
    pub fn num_object_bytes(ctx: &mut Context) -> usize {
        // Size is in the lower 4 bits
        let size = ctx.cursor.current_value() & 0b0000_1111;
        // We have to parse the next few bytes as an unsigned int if the size maxes out
        if size == 0b0000_1111 {
            ctx.cursor.advance_by(1).unwrap();

            Self::parse_int(ctx) as usize
        } else {
            size as usize
        }
    }

    pub fn parse_int(ctx: &mut Context) -> i64 {
        let num_bytes = 2u8.pow(Self::num_object_bytes(ctx) as u32);
        ctx.cursor.advance_by(1).unwrap();
        ctx.parse_int(num_bytes)
    }

    pub fn parse_float(ctx: &mut Context) -> f64 {
        let num_bytes = 2usize.pow(Self::num_object_bytes(ctx) as u32);
        ctx.cursor.advance_by(1).unwrap();

        match num_bytes {
            4 => f32::from_be_bytes([
                ctx.cursor.current_value().to_owned(),
                ctx.cursor.next_val().unwrap().to_owned(),
                ctx.cursor.next_val().unwrap().to_owned(),
                ctx.cursor.next_val().unwrap().to_owned(),
            ]) as f64,
            8 => f64::from_be_bytes([
                ctx.cursor.current_value().to_owned(),
                ctx.cursor.next_val().unwrap().to_owned(),
                ctx.cursor.next_val().unwrap().to_owned(),
                ctx.cursor.next_val().unwrap().to_owned(),
                ctx.cursor.next_val().unwrap().to_owned(),
                ctx.cursor.next_val().unwrap().to_owned(),
                ctx.cursor.next_val().unwrap().to_owned(),
                ctx.cursor.next_val().unwrap().to_owned(),
            ]),
            _ => panic!("Unsupported float byte size: {num_bytes} bytes"),
        }
    }

    pub fn parse_null_bool(ctx: &Context) -> Option<bool> {
        match ctx.cursor.current_value().to_owned() {
            // Null
            0b0000_0000 => None,
            // Bool
            0b0000_1000 => Some(false),
            0b0000_1001 => Some(true),
            // Invalid
            b => panic!("Invalid byte {b:#b} ({b}) while trying to parse bool"),
        }
    }

    pub fn parse_dict(ctx: &mut Context) -> HashMap<String, Self> {
        let mut map = HashMap::default();
        let map_base = ctx.cursor.pos();
        let num_entries = Self::num_object_bytes(ctx);
        let mut entries_processed = 0;

        while entries_processed < num_entries {
            entries_processed += 1;
            ctx.cursor.move_to(map_base + entries_processed).unwrap();
            ctx.follow_current_object_ref();
            let key_obj = Self::deserialize(ctx);
            let key = match key_obj {
                Self::AsciiString(key) => key,
                Self::UnicodeString(key) => key,
                other => panic!("Unsupported Dictionary key type: {other:?}"),
            };

            ctx.cursor
                .move_to(map_base + entries_processed + num_entries)
                .unwrap();
            ctx.follow_current_object_ref();
            let value = Self::deserialize(ctx);

            map.insert(key, value);
        }

        map
    }

    pub fn parse_array(ctx: &mut Context) -> Vec<Self> {
        let mut array = Vec::new();
        let base = ctx.cursor.pos();
        let num_entries = Self::num_object_bytes(ctx);
        let mut entries_processed = 0;

        while entries_processed < num_entries {
            entries_processed += 1;
            ctx.cursor.move_to(base + entries_processed).unwrap();
            ctx.follow_current_object_ref();
            array.push(Self::deserialize(ctx))
        }

        array
    }

    pub fn parse_string(ctx: &mut Context) -> String {
        let len = Self::num_object_bytes(ctx);
        let bytes = ctx.cursor.next_values(len).unwrap().to_vec();
        String::from_utf8(bytes).unwrap()
    }
}
impl Rule for Object {
    type Input = Context;

    fn matches(input: &Self::Input) -> bool {
        let signature = input.cursor.current_value() >> 4;

        ObjectSignature::try_from(signature).is_ok()
    }

    fn deserialize(input: &mut Self::Input) -> Self {
        let signature = input.cursor.current_value() >> 4;

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
