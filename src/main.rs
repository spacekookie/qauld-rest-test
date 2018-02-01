extern crate libqaul;

use libqaul::*;
use std::ffi::CString;
use std::mem;

fn main() {

    unsafe {
        let q: *mut *mut qaul::qaul = mem::uninitialized();
        ql_initialise(q, qaul_os_LINUX, CString::new("path").unwrap().as_ptr(), CString::new("path").unwrap().as_ptr());
    }

    println!("Hello, world!");
}
