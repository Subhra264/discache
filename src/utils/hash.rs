use super::bindings::{XXH3_64bits, XXH3_64bits_withSeed};
use std::os::raw::c_void;

pub fn xxhash_64(data: &str) -> u64 {
    unsafe { XXH3_64bits(data.as_ptr() as *const c_void, data.len()) }
}

pub fn xxhash_64_with_seed(data: &str, seed: u64) -> u64 {
    unsafe { XXH3_64bits_withSeed(data.as_ptr() as *const c_void, data.len(), seed) }
}
