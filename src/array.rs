// Copyright 2019, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib_sys;
use std::fmt;
use std::ops::Deref;
use std::slice;
use translate::*;
use std::ptr::NonNull;
use std::marker::PhantomData;

//TODO - macro doesn't like generics yet
//glib_wrapper! {
//    pub struct Array<T>(Shared<glib_sys::GArray>, PhantomData<T>);
//
//    match fn {
//        ref => |ptr| glib_sys::g_array_ref(ptr),
//        unref => |ptr| glib_sys::g_array_unref(ptr),
//        get_type => || glib_sys::g_array_get_type(),
//    }
//}

pub struct Array<T>(*mut glib_sys::GArray, PhantomData<T>);

impl<T> Array<T> {
    pub fn new(zero_terminated: bool, clear: bool) -> Self {
        unsafe {
            Array(glib_sys::g_array_new(zero_terminated.to_glib(),
                                        clear.to_glib(),
                                        ::std::mem::size_of::<T>() as _),
                  PhantomData)
        }
    }

    pub fn with_capacity(zero_terminated: bool, clear: bool, capacity: usize) -> Self {
        unsafe {
            Array(glib_sys::g_array_sized_new(zero_terminated.to_glib(),
                                              clear.to_glib(),
                                              ::std::mem::size_of::<T>() as _,
                                              capacity as _),
                  PhantomData)
        }
    }

    pub fn len(&self) -> usize {
        unsafe { (*self.to_glib_none().0).len as usize }
    }
}

impl<T: Copy> Array<T> {
    pub fn append(&self, elem: T) -> &Self {
        // copying elem in memory, that feels quite unsafe, might be ok on Copy types
        // another variant would do ToGlibPtr conversions?
        unsafe {
            let elem: *const T = &elem;
            glib_sys::g_array_append_vals(self.to_glib_none().0, elem as *const _, 1);
        }
        self
    }
}

impl<T> Deref for Array<T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        unsafe {
            let mut ptr = (*self.to_glib_none().0).data as *const T;
            let len = self.len();
            debug_assert!(!ptr.is_null() || len == 0);
            if ptr.is_null() {
                ptr = NonNull::dangling().as_ptr();
            }
            slice::from_raw_parts(ptr as *const _, len)
        }
    }
}

impl<T> AsRef<[T]> for Array<T> {
    fn as_ref(&self) -> &[T] {
        &*self
    }
}

impl<T> fmt::Debug for Array<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Array")
            .field("ptr", &self.to_glib_none().0)
            .field("len", &self.len())
            .finish()
    }
}

impl<T> Drop for Array<T> {
    fn drop(&mut self) {
        unsafe { glib_sys::g_array_free(self.0, true.to_glib()); }
    }
}

#[doc(hidden)]
impl<'a, T: 'a> ToGlibPtr<'a, *mut glib_sys::GArray> for Array<T> {
    type Storage = &'a Self;

    fn to_glib_none(&'a self) -> Stash<'a, *mut glib_sys::GArray, Self> {
        let ptr = self.0 as *const glib_sys::GArray;
        Stash(ptr as _, self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn array() {
        let arr = Array::with_capacity(true, true, 3);
        arr.append(42).append(43);
        assert_eq!(arr.as_ref(), [42, 43]);
    }
}
