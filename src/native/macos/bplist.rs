use syntax::prelude::*;

// Source for how all this shit works:
// https://opensource.apple.com/source/CF/CF-1153.18/CFBinaryPList.c.auto.html

mod object;
pub use object::*;

#[derive(Debug)]
pub struct Trailer {
    /// The size of each offset in the offset table
    pub offset_size: u8,
    /// Byte size of object references in arrays and dictionaries
    pub object_ref_size: u8,
    /// The number of encoded objects
    pub num_objects: u64,
    /// The byte in the offset table that gives the offset of the first object
    pub first_object_offset: u64,
    /// The location of the first byte of the offset table, relative to the start of the file
    pub offset_table_start: u64,
}
impl Trailer {
    const SIZE: usize = 32;
}
impl Rule for Trailer {
    type Input = Cursor<u8>;

    fn matches(input: &Self::Input) -> bool {
        input.remaining_values() == Self::SIZE
    }

    fn deserialize(input: &mut Self::Input) -> Self {
        // First 4 bytes are unused, 5th seems to be irrelevant
        input.advance_by(5).unwrap();
        let offset_size = input.next_val().unwrap().to_owned();
        let object_ref_size = input.next_val().unwrap().to_owned();
        let num_objects = u64::from_be_bytes(input.next_n_values::<8>().unwrap().to_owned());
        let first_object_offset =
            u64::from_be_bytes(input.next_n_values::<8>().unwrap().to_owned());
        let offset_table_start = u64::from_be_bytes(input.next_n_values::<8>().unwrap().to_owned());

        Self {
            offset_size,
            object_ref_size,
            num_objects,
            first_object_offset,
            offset_table_start,
        }
    }
}

/// Parsing objects requires information from the trailer and the reader. Object serializers take
/// this Context struct so they have both.
pub struct Context {
    pub trailer: Trailer,
    pub cursor: Cursor<u8>,
}
impl Context {
    /// Parses an integer that is `num_bytes` bytes in size. Will always convert the number to an
    /// i64, since 8-byte numbers can be negative.
    pub fn parse_int(&mut self, num_bytes: u8) -> i64 {
        match num_bytes {
            1 => u8::from_be_bytes([self.cursor.current_value().to_owned()]).into(),
            2 => u16::from_be_bytes([
                self.cursor.current_value().to_owned(),
                self.cursor.next_val().unwrap().to_owned(),
            ])
            .into(),
            4 => u32::from_be_bytes([
                self.cursor.current_value().to_owned(),
                self.cursor.next_val().unwrap().to_owned(),
                self.cursor.next_val().unwrap().to_owned(),
                self.cursor.next_val().unwrap().to_owned(),
            ])
            .into(),
            8 => i64::from_be_bytes([
                self.cursor.current_value().to_owned(),
                self.cursor.next_val().unwrap().to_owned(),
                self.cursor.next_val().unwrap().to_owned(),
                self.cursor.next_val().unwrap().to_owned(),
                self.cursor.next_val().unwrap().to_owned(),
                self.cursor.next_val().unwrap().to_owned(),
                self.cursor.next_val().unwrap().to_owned(),
                self.cursor.next_val().unwrap().to_owned(),
            ]),
            _ => panic!("Unsupported int byte size: {num_bytes} bytes"),
        }
    }

    /// Assumes the InputReader's cursor is currently pointing to an offset in the offset table.
    /// Reads that offset and moves the cursor to the object that offset points to.
    /// The only reason this has its own function is the offsets can span one or multiple bytes.
    pub fn follow_current_offset(&mut self) {
        let pos = self.parse_int(self.trailer.offset_size) as usize;
        self.cursor
            .move_to(pos)
            .map_err(|_| format!("Failed to follow current offset, pos was {pos} ({pos:#x})"))
            .unwrap();
    }

    /// Assumes the InputReader's cursor is currently pointing to an object ref in the object table.
    /// Reads that object ref, follows it to the offset table, then follows that to the actual object.
    /// The only reason this has its own function is the object refs can span one or multiple bytes.
    pub fn follow_current_object_ref(&mut self) {
        let offset_index = self.parse_int(self.trailer.object_ref_size) as usize;
        self.cursor
            .move_to(self.trailer.offset_table_start as usize + offset_index)
            .unwrap();

        self.follow_current_offset();
    }
}

#[derive(Debug)]
pub struct BPlist {
    pub objects: Vec<Object>,
}
impl Rule for BPlist {
    type Input = Context;

    fn matches(_input: &Self::Input) -> bool {
        // Not really a way to check this one
        true
    }

    fn deserialize(ctx: &mut Self::Input) -> Self {
        let mut objects = Vec::new();

        // Point the reader at the offset of the first object in the object table, then follow
        // that offset to the actual object.
        ctx.cursor
            .move_to((ctx.trailer.offset_table_start + ctx.trailer.first_object_offset) as usize)
            .unwrap();
        ctx.follow_current_offset();

        // Calculate the end of the object table, relative to the end of the file
        let offset_table_size = ctx.trailer.offset_size as u64 * ctx.trailer.num_objects;
        let object_table_end = offset_table_size as usize + Trailer::SIZE;

        // Parse the whole object table, and store all of the objects in the objects vector.
        // This assumes that each Object will leave the cursor at the last byte of the object it
        // just parsed.
        // remaining_values() includes the current value, so we subtract one from it.
        while (ctx.cursor.remaining_values() - 1) > object_table_end {
            objects.push(Object::deserialize(ctx));

            // Move to the start of the next object in the object table
            ctx.cursor.advance_by(1).unwrap();
        }

        Self { objects }
    }
}
impl MainRule for BPlist {
    type Input = Cursor<u8>;

    fn matches(input: &Self::Input) -> bool {
        let Some(header) = input.peek_range(0, 8) else {
            return false;
        };
        header == b"bplist00"
    }

    fn deserialize(mut cursor: Self::Input) -> Self {
        cursor
            .move_to(cursor.remaining_values() - Trailer::SIZE)
            .unwrap();
        assert!(Trailer::matches(&cursor));
        let trailer = Trailer::deserialize(&mut cursor);

        let mut ctx = Context { trailer, cursor };
        <Self as Rule>::deserialize(&mut ctx)
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
