// Copyright 2019, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib_sys;
use std::mem;
use translate::{from_glib, Stash, ToGlibPtr};

#[derive(Debug)]
pub enum RecMutex<'a> {
    Owned(glib_sys::GRecMutex),
    Borrowed(&'a glib_sys::GRecMutex),
}

impl<'a> RecMutex<'a> {
    pub fn new() -> Self {
        let rec_mutex = unsafe {
            let mut mutex = mem::zeroed();
            glib_sys::g_rec_mutex_init(&mut mutex);
            mutex
        };
        RecMutex::Owned(rec_mutex)
    }

    #[doc(hidden)]
    pub unsafe fn borrow(rec_mutex: &'a glib_sys::GRecMutex) -> Self {
        RecMutex::Borrowed(rec_mutex)
    }

    pub fn lock(&self) -> RecMutexGuard {
        unsafe {
            glib_sys::g_rec_mutex_lock(self.as_ptr());
        }
        RecMutexGuard { rec_mutex: self }
    }

    pub fn try_lock(&self) -> Option<RecMutexGuard> {
        let locked = unsafe { from_glib(glib_sys::g_rec_mutex_trylock(self.as_ptr())) };

        if locked {
            Some(RecMutexGuard { rec_mutex: self })
        } else {
            None
        }
    }

    #[doc(hidden)]
    fn as_ptr(&self) -> *mut glib_sys::GRecMutex {
        self.to_glib_none().0 as *const _ as usize as *mut _
    }
}

impl<'a> Drop for RecMutex<'a> {
    fn drop(&mut self) {
        match self {
            RecMutex::Owned(_) => unsafe {
                glib_sys::g_rec_mutex_clear(self.as_ptr());
            },
            _ => {}
        }
    }
}

#[derive(Debug)]
pub struct RecMutexGuard<'a> {
    rec_mutex: &'a RecMutex<'a>,
}

impl<'a> Drop for RecMutexGuard<'a> {
    fn drop(&mut self) {
        unsafe {
            glib_sys::g_rec_mutex_unlock(self.rec_mutex.as_ptr());
        }
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *const glib_sys::GRecMutex> for RecMutex<'a> {
    type Storage = &'a Self;

    fn to_glib_none(&'a self) -> Stash<'a, *const glib_sys::GRecMutex, Self> {
        match self {
            RecMutex::Owned(ref rec_mutex) => Stash(rec_mutex, self),
            RecMutex::Borrowed(rec_mutex) => Stash(*rec_mutex, self),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_mutex(mutex: RecMutex) {
        assert!(mutex.try_lock().is_some());
        {
            let _first = mutex.lock();
            let _snd = mutex.lock();
        }
        mutex.lock();
    }

    #[test]
    fn test_owned() {
        let mutex = RecMutex::new();
        test_mutex(mutex);
    }

    #[test]
    fn test_borrowed() {
        unsafe {
            let mut mutex = mem::zeroed();
            glib_sys::g_rec_mutex_init(&mut mutex);

            let mutex = RecMutex::borrow(&mutex);
            test_mutex(mutex)
        }
    }
}
