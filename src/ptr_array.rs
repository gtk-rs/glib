// Copyright 2019, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib_sys;
use std::fmt;
use translate::*;

glib_wrapper! {
    pub struct PtrArray(Shared<glib_sys::GPtrArray>);

    match fn {
        ref => |ptr| glib_sys::g_ptr_array_ref(ptr),
        unref => |ptr| glib_sys::g_ptr_array_unref(ptr),
        get_type => || glib_sys::g_ptr_array_get_type(),
    }
}

impl<'a, T> ToGlibContainerFromSlice<'a, *mut glib_sys::GPtrArray> for T
where T: GlibPtrDefault + ToGlibPtr<'a, <T as GlibPtrDefault>::GlibType> {
    type Storage = (Option<PtrArray>, Vec<Stash<'a, <T as GlibPtrDefault>::GlibType, T>>);

    #[inline]
    fn to_glib_none_from_slice(t: &'a [T]) -> (*mut glib_sys::GPtrArray, Self::Storage) {
        let (ptr, storage) = Self::to_glib_container_from_slice(t);

        unsafe { (ptr, (Some(from_glib_full(ptr)), storage.1)) }
    }

    #[inline]
    fn to_glib_container_from_slice(t: &'a [T]) -> (*mut glib_sys::GPtrArray, Self::Storage) {
        let stash_vec: Vec<_> =
            t.iter().map(ToGlibPtr::to_glib_none).collect();
        unsafe {
            let arr = glib_sys::g_ptr_array_new_full(t.len() as _, None);

            for stash in &stash_vec {
                let ptr: *mut <T as GlibPtrDefault>::GlibType = Ptr::to(stash.0);
                glib_sys::g_ptr_array_add(arr, ptr as *mut _);
            }

            (arr, (None, stash_vec))
        }
    }

    #[inline]
    fn to_glib_full_from_slice(_t: &[T]) -> *mut glib_sys::GPtrArray {
        unimplemented!() // with or without destroy callback?
    }
}

impl<'a, T> ToGlibContainerFromSlice<'a, *const glib_sys::GPtrArray> for T
where T: GlibPtrDefault + ToGlibPtr<'a, <T as GlibPtrDefault>::GlibType> {
    type Storage = (Option<PtrArray>, Vec<Stash<'a, <T as GlibPtrDefault>::GlibType, T>>);

    #[inline]
    fn to_glib_none_from_slice(t: &'a [T]) -> (*const glib_sys::GPtrArray, Self::Storage) {
        let (arr, stash) = ToGlibContainerFromSlice::<*mut glib_sys::GPtrArray>::to_glib_none_from_slice(t);
        (arr as *const glib_sys::GPtrArray, stash)
    }

    #[inline]
    fn to_glib_container_from_slice(_t: &'a [T]) -> (*const glib_sys::GPtrArray, Self::Storage) {
        unimplemented!()
    }

    #[inline]
    fn to_glib_full_from_slice(_t: &[T]) -> *const glib_sys::GPtrArray {
        unimplemented!()
    }
}

impl<T> FromGlibContainerAsVec<<T as GlibPtrDefault>::GlibType, *mut glib_sys::GPtrArray> for T
where T: GlibPtrDefault + FromGlibPtrNone<<T as GlibPtrDefault>::GlibType> + FromGlibPtrFull<<T as GlibPtrDefault>::GlibType> {
    unsafe fn from_glib_none_num_as_vec(ptr: *mut glib_sys::GPtrArray, num: usize) -> Vec<T> {
        if num == 0 || ptr.is_null() {
            return Vec::new()
        }
        let pdata = (*ptr).pdata;
        let mut res = Vec::with_capacity(num);
        for i in 0..num {
            let item_ptr = ::std::ptr::read(pdata.add(i));
            let item_ptr: <T as GlibPtrDefault>::GlibType = Ptr::from(item_ptr);
            res.push(from_glib_none(item_ptr));
        }
        res
    }

    unsafe fn from_glib_container_num_as_vec(ptr: *mut glib_sys::GPtrArray, num: usize) -> Vec<T> {
        let res = FromGlibContainer::from_glib_none_num(ptr, num);
        if !ptr.is_null() {
            glib_sys::g_ptr_array_free(ptr, true.to_glib());
        }
        res
    }

    unsafe fn from_glib_full_num_as_vec(ptr: *mut glib_sys::GPtrArray, num: usize) -> Vec<T> {
        if num == 0 || ptr.is_null() {
            return Vec::new()
        }
        let pdata = (*ptr).pdata;
        let mut res = Vec::with_capacity(num);
        for i in 0..num {
            let item_ptr = ::std::ptr::read(pdata.add(i));
            let item_ptr: <T as GlibPtrDefault>::GlibType = Ptr::from(item_ptr);
            res.push(from_glib_full(item_ptr));
        }
        glib_sys::g_ptr_array_free(ptr, true.to_glib());
        res
    }
}

impl<T> FromGlibPtrArrayContainerAsVec<<T as GlibPtrDefault>::GlibType, *mut glib_sys::GPtrArray> for T
where T: GlibPtrDefault + FromGlibPtrNone<<T as GlibPtrDefault>::GlibType> + FromGlibPtrFull<<T as GlibPtrDefault>::GlibType> {
    unsafe fn from_glib_none_as_vec(ptr: *mut glib_sys::GPtrArray) -> Vec<T> {
        let num = (*ptr).len as usize;
        FromGlibContainer::from_glib_none_num(ptr, num)
    }

    unsafe fn from_glib_container_as_vec(ptr: *mut glib_sys::GPtrArray) -> Vec<T> {
        let num = (*ptr).len as usize;
        FromGlibContainer::from_glib_container_num(ptr, num)
    }

    unsafe fn from_glib_full_as_vec(ptr: *mut glib_sys::GPtrArray) -> Vec<T> {
        let num = (*ptr).len as usize;
        FromGlibContainer::from_glib_full_num(ptr, num)
    }
}

impl<T> FromGlibContainerAsVec<<T as GlibPtrDefault>::GlibType, *const glib_sys::GPtrArray> for T
where T: GlibPtrDefault + FromGlibPtrNone<<T as GlibPtrDefault>::GlibType> + FromGlibPtrFull<<T as GlibPtrDefault>::GlibType> {
    unsafe fn from_glib_none_num_as_vec(ptr: *const glib_sys::GPtrArray, num: usize) -> Vec<T> {
        FromGlibContainer::from_glib_none_num(mut_override(ptr), num)
    }

    unsafe fn from_glib_container_num_as_vec(_: *const glib_sys::GPtrArray, _: usize) -> Vec<T> {
        // Can't really free a *const
        unimplemented!()
    }

    unsafe fn from_glib_full_num_as_vec(_: *const glib_sys::GPtrArray, _: usize) -> Vec<T> {
        // Can't really free a *const
        unimplemented!()
    }
}


impl<T> FromGlibPtrArrayContainerAsVec<<T as GlibPtrDefault>::GlibType, *const glib_sys::GPtrArray> for T
where T: GlibPtrDefault + FromGlibPtrNone<<T as GlibPtrDefault>::GlibType> + FromGlibPtrFull<<T as GlibPtrDefault>::GlibType> {
    unsafe fn from_glib_none_as_vec(ptr: *const glib_sys::GPtrArray) -> Vec<T> {
        FromGlibPtrContainer::from_glib_none(mut_override(ptr))
    }

    unsafe fn from_glib_container_as_vec(_: *const glib_sys::GPtrArray) -> Vec<T> {
        // Can't really free a *const
        unimplemented!()
    }

    unsafe fn from_glib_full_as_vec(_: *const glib_sys::GPtrArray) -> Vec<T> {
        // Can't really free a *const
        unimplemented!()
    }
}

impl fmt::Debug for PtrArray {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe { f.debug_struct("PtrArray")
                 .field("ptr", &self.to_glib_none().0)
                 .field("len", &(*self.to_glib_none().0).len)
                 .finish()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gstring::GString;

    #[test]
    fn ptr_array() {
        let arr = &["foo", "bar", "baz"];
        let (ptr, pa) = ToGlibContainerFromSlice::<*mut glib_sys::GPtrArray>::to_glib_none_from_slice(arr);
        let vec: Vec<GString> = unsafe { FromGlibPtrArrayContainerAsVec::from_glib_none_as_vec(ptr) };
        assert_eq!(&vec[1], "bar");
    }
}
