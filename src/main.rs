use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_void};

extern "C" {
    fn greet_me(name: *const c_char) -> i32;
    fn be_rude(name: *const c_char);
    fn segfault();
}

fn main() {
    unsafe {
        greet_me(CString::new("Kate").unwrap().as_ptr());
        be_rude(CString::new("Kate").unwrap().as_ptr());
    }

    println!("Be careful though...");
    unsafe { segfault() };
}
