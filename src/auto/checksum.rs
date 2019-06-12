// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib_sys;
use translate::*;
use ChecksumType;

glib_wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Checksum(Boxed<glib_sys::GChecksum>);

    match fn {
        copy => |ptr| glib_sys::g_checksum_copy(mut_override(ptr)),
        free => |ptr| glib_sys::g_checksum_free(ptr),
        get_type => || glib_sys::g_checksum_get_type(),
    }
}

impl Checksum {
    pub fn new(checksum_type: ChecksumType) -> Checksum {
        unsafe { from_glib_full(glib_sys::g_checksum_new(checksum_type.to_glib())) }
    }

    pub fn reset(&mut self) {
        unsafe {
            glib_sys::g_checksum_reset(self.to_glib_none_mut().0);
        }
    }

    pub fn update(&mut self, data: &[u8]) {
        let length = data.len() as isize;
        unsafe {
            glib_sys::g_checksum_update(self.to_glib_none_mut().0, data.to_glib_none().0, length);
        }
    }

    pub fn type_get_length(checksum_type: ChecksumType) -> isize {
        unsafe { glib_sys::g_checksum_type_get_length(checksum_type.to_glib()) }
    }
}

unsafe impl Send for Checksum {}
unsafe impl Sync for Checksum {}
