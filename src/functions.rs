#[cfg(any(feature = "v2_34", feature = "dox"))]
use Error;
#[cfg(any(feature = "v2_34", feature = "dox"))]
use ffi;
#[cfg(any(feature = "v2_34", feature = "dox"))]
use std::ptr;
#[cfg(any(feature = "v2_34", feature = "dox"))]
use translate::*;

#[cfg(any(feature = "v2_34", feature = "dox"))]
pub fn spawn_check_exit_status(exit_status: i32) -> Result<bool, Error> {
    unsafe {
        let mut error = ptr::null_mut();
        let ret = ffi::g_spawn_check_exit_status(exit_status, &mut error);
        if error.is_null() { Ok(from_glib(ret)) } else { Err(from_glib_full(error)) }
    }
}
