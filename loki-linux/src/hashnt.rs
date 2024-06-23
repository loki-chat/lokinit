use std::hash::{BuildHasher, Hasher};

/// Hashn'ts values. It implements hash but doesn't actually do any hashing.
/// This can be useful for hashmaps where the keys are numbers, especially if
/// you know those numbers will be unique.
///
/// Note: Rust has hashes and hash builders. This is technically the hash builder.
/// It's the type you want to use with hashmaps and related data structures; but
/// for the actual hash implementation, see [`HashntHash`].
#[derive(Default)]
pub struct Hashnt;
impl BuildHasher for Hashnt {
    type Hasher = HashntHash;

    fn build_hasher(&self) -> Self::Hasher {
        HashntHash::default()
    }
}

/// The [`Hasher`] implementation for [`Hashnt`]. It simply stores values written
/// to it while hashing and then returns the final value in `finish`, thus implementing
/// [`Hasher`] without doing any actual hashing.
#[derive(Default)]
pub struct HashntHash {
    pub result: u64,
}
impl Hasher for HashntHash {
    fn write(&mut self, i: &[u8]) {
        match i.len() {
            1 => self.write_u8(u8::from_ne_bytes(i.try_into().unwrap())),
            2 => self.write_u16(u16::from_ne_bytes(i.try_into().unwrap())),
            4 => self.write_u32(u32::from_ne_bytes(i.try_into().unwrap())),
            8 => self.write_u64(u64::from_ne_bytes(i.try_into().unwrap())),
            16 => self.write_u128(u128::from_ne_bytes(i.try_into().unwrap())),
            _ => unimplemented!(),
        }
    }

    fn write_u8(&mut self, i: u8) {
        self.result = i as u64;
    }
    fn write_i8(&mut self, i: i8) {
        self.result = i as u64;
    }
    fn write_u16(&mut self, i: u16) {
        self.result = i as u64;
    }
    fn write_i16(&mut self, i: i16) {
        self.result = i as u64;
    }
    fn write_u32(&mut self, i: u32) {
        self.result = i as u64;
    }
    fn write_i32(&mut self, i: i32) {
        self.result = i as u64;
    }
    fn write_u64(&mut self, i: u64) {
        self.result = i;
    }
    fn write_i64(&mut self, i: i64) {
        self.result = i as u64;
    }
    fn write_u128(&mut self, i: u128) {
        self.result = i as u64;
    }
    fn write_i128(&mut self, i: i128) {
        self.result = i as u64;
    }
    fn write_usize(&mut self, i: usize) {
        self.result = i as u64;
    }
    fn write_isize(&mut self, i: isize) {
        self.result = i as u64;
    }

    fn finish(&self) -> u64 {
        self.result
    }
}
