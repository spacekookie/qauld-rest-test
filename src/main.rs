extern crate libqaul;

use libqaul::*;
use std::ffi::CString;


fn main() {

    unsafe {
        let mut qaul: *mut qaul::qaul = std::ptr::null_mut();
        let status = ql_initialise(&mut qaul, qaul_os_LINUX, CString::new("path").unwrap().as_ptr(), CString::new("path").unwrap().as_ptr());

        match status {
            ql_error_t_SUCCESS => println!("Initialisation was a success!"),
            _ => {}
        }

        let status2 = ql_shutdown(qaul);
        match status2 {
            ql_error_t_NOT_INITIALISED => println!("Shutdown didn't do anything!!"),
            _ => {}
        }
    }

    println!("Hello, world!");
}
