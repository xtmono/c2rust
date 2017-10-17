// JodyHash: fast hash function for 64-bit blocks
// from https://github.com/jbruchon/jodyhash

use std::hash::Hasher;
use std::ops::BitXor;
use super::XCheckHasher;

#[derive(Debug, Default)]
pub struct JodyHasher(u64);

impl JodyHasher {
    #[inline]
    pub fn new() -> JodyHasher {
        Default::default()
    }
}

const JODY_HASH_CONSTANT: u64 = 0x1f3d5b79u64;
const JODY_HASH_SHIFT: u32 = 14;

impl Hasher for JodyHasher {
    #[inline]
    fn finish(&self) -> u64 {
        self.0
    }

    fn write(&mut self, _bytes: &[u8]) {
        // FIXME: we only support 64-bit inputs for now
        unimplemented!()
    }

    #[inline]
    fn write_u64(&mut self, i: u64) {
        self.0 = self.0
            .wrapping_add(i)
            .wrapping_add(JODY_HASH_CONSTANT)
            .rotate_left(JODY_HASH_SHIFT)
            .bitxor(i)
            .rotate_left(JODY_HASH_SHIFT)
            .bitxor(JODY_HASH_CONSTANT)
            .wrapping_add(i);
    }
}

impl XCheckHasher for JodyHasher {}

#[cfg(test)]
mod tests {
    use super::{Hasher, JodyHasher};

    fn jodyhash_string(s: &str) -> u64 {
        assert!(s.len() % 8 == 0);
        let mut h = JodyHasher::default();
        let sptr = s.as_ptr() as *const u64;
        (0..s.len() / 8).for_each(|i| {
            h.write_u64(unsafe { *sptr.offset(i as isize) })
        });
        h.finish()
    }

    #[test]
    fn test_jodyhash() {
        // Single word strings
        assert_eq!(jodyhash_string("aaaaaaaa"), 0xa1b314f742d47698u64);
        assert_eq!(jodyhash_string("abcdefgh"), 0xea0ab92ac586d967u64);
        assert_eq!(jodyhash_string("hgfedcba"), 0x31f588dbf657cc7fu64);
        assert_eq!(jodyhash_string("jodyhash"), 0xba43645fabc566ddu64);

        // 2-word strings
        assert_eq!(jodyhash_string("aaaaaaaaaaaaaaaa"), 0x35d0ccd0c8b5a5a5u64);
        assert_eq!(jodyhash_string("abcdefghijklmnop"), 0xcef9115bbc6c0f84u64);
        assert_eq!(jodyhash_string("zyxwvutsrqponmlk"), 0xe408a517f5c12d07u64);
    }
}