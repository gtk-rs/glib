// Copyright 2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! `IMPL` Low level signal support.

use libc::{c_void, c_uint};

use gobject_ffi::{self, GCallback};
use glib_ffi::{GQuark, GType};
use source::CallbackGuard;
use translate::ToGlibPtr;

pub unsafe fn connect(receiver: *mut gobject_ffi::GObject, signal_name: &str, trampoline: GCallback,
                      closure: *mut Box<Fn() + 'static>) -> u64 {
    let handle = gobject_ffi::g_signal_connect_data(receiver, signal_name.to_glib_none().0,
        trampoline, closure as *mut _, Some(destroy_closure),
        gobject_ffi::GConnectFlags::empty()) as u64;
    assert!(handle > 0);
    handle
}

pub unsafe fn stop_emission(instance: *mut gobject_ffi::GObject, signal_id: u32, detail: GQuark) {
    gobject_ffi::g_signal_stop_emission(instance, signal_id as c_uint, detail);
}

unsafe extern "C" fn destroy_closure(ptr: *mut c_void, _: *mut gobject_ffi::GClosure) {
    let _guard = CallbackGuard::new();
    // destroy
    Box::<Box<Fn()>>::from_raw(ptr as *mut _);
}
