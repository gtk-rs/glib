// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib_sys;
use std::mem;
use translate::*;

pub use glib_sys::GTimeVal as TimeVal;

pub fn get_current_time() -> TimeVal {
    unsafe {
        let mut ret = mem::uninitialized();
        glib_sys::g_get_current_time(&mut ret);
        ret
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *const glib_sys::GTimeVal> for TimeVal {
    type Storage = &'a Self;

    fn to_glib_none(&'a self) -> Stash<'a, *const glib_sys::GTimeVal, Self> {
        Stash(self as _, self)
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtrMut<'a, *mut glib_sys::GTimeVal> for TimeVal {
    type Storage = &'a mut Self;

    fn to_glib_none_mut(&'a mut self) -> StashMut<'a, *mut glib_sys::GTimeVal, Self> {
        StashMut(self as _, self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use DateTime;

    #[test]
    fn various() {
        let tv = get_current_time();
        let dt = DateTime::new_from_timeval_local(&tv);
        let _ = dt.format("It is currently %x %X %z");
    }
}
