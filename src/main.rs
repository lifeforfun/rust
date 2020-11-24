
use libc::{c_char, c_int, c_uint};
use std::env::args;
use std::ffi::CString;
#[link(name = "gstreamer-1.0")]
extern "C" {
    fn gst_init(argv: *const c_int, argc: *const *const *const c_char);
    fn gst_version(
        major: *const c_uint,
        minor: *const c_uint,
        micro: *const c_uint,
        nano: *const c_uint,
    );
}

fn main() {
    let args = args()
        .map(|arg| CString::new(arg).unwrap())
        .collect::<Vec<CString>>();

    let c_args = args
        .iter()
        .map(|arg| arg.as_ptr())
        .collect::<Vec<*const c_char>>();
    let (major, minor, micro, nano): (c_uint, c_uint, c_uint, c_uint) = (0, 0, 0, 0);
    unsafe {
        gst_init(&(c_args.len() as c_int) as *const c_int, &c_args.as_ptr());
        gst_version(&major, &minor, &micro, &nano);
    }
    println!("{}, {}, {}, {}", major, minor, micro, nano);
}
