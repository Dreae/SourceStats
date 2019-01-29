extern crate mio;
extern crate bytes;
extern crate byteorder;
extern crate sourcestats_protocol;

use std::ffi::CStr;
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn init_sourcestats(daemon_url: *const c_char) -> i32 {
    let url = unsafe { CStr::from_ptr(daemon_url) };
    unimplemented!()
}