use super::bindings::XXH3_64bits;
use std::os::raw::c_void;

pub fn xxhash_64(data: &str) -> u64 {
    unsafe { XXH3_64bits(data.as_ptr() as *const c_void, data.len()) }
}
