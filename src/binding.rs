// Copyright 2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use std::mem;
use std::ptr;

use {BindingFlags, Closure};
use ffi as glib_ffi;
use gobject_ffi;
use object::{IsA, Object};
use translate::{FromGlib, ToGlib, ToGlibPtr, ToGlibPtrMut, FromGlibPtrFull, Stash, from_glib_none};

glib_wrapper! {
    pub struct Binding(Object<gobject_ffi::GBinding>);

    match fn {
        get_type => || gobject_ffi::g_binding_get_type(),
    }
}

pub trait BindingExt {
    fn bind_property<S: IsA<Object>, T: IsA<Object>>(source: &S, source_property: &str, target: &T, target_property: &str, flags: BindingFlags) -> Self;
    fn bind_property_with_closures<S: IsA<Object>, T: IsA<Object>>(source: &S, source_property: &str, target: &T, target_property: &str, flags: BindingFlags, transform_to: Closure, transform_from: Closure) -> Self;
    fn get_flags(&self) -> BindingFlags;
    fn get_source(&self) -> Object;
    fn get_source_property(&self) -> String;
    fn get_target(&self) -> Object;
    fn get_target_property(&self) -> String;
    #[cfg(any(feature = "v2_38"))]
    fn unbind(&self);
}

impl BindingExt for Binding {
    fn bind_property<S: IsA<Object>, T: IsA<Object>>(source: &S, source_property: &str, target: &T, target_property: &str, flags: BindingFlags) -> Self {
        unsafe {
            let gbinding = gobject_ffi::g_object_bind_property(source.to_glib_none().0,
                source_property.to_glib_none().0, target.to_glib_none().0, target_property.to_glib_none().0,
                flags.to_glib());
            from_glib_none(gbinding)
        }
    }

    fn bind_property_with_closures<S: IsA<Object>, T: IsA<Object>>(source: &S, source_property: &str, target: &T, target_property: &str, flags: BindingFlags, transform_to: Closure, transform_from: Closure) -> Self {
        unsafe {
            let gbinding = gobject_ffi::g_object_bind_property_with_closures(source.to_glib_none().0,
                source_property.to_glib_none().0, target.to_glib_none().0, target_property.to_glib_none().0,
                flags.to_glib(), transform_to.to_glib_none().0, transform_from.to_glib_none().0);
            from_glib_none(gbinding)
        }
    }

    fn get_flags(&self) -> BindingFlags {
        unsafe {
            FromGlib::from_glib(gobject_ffi::g_binding_get_flags(self.to_glib_none().0))
        }
    }

    fn get_source(&self) -> Object {
        unsafe {
            from_glib_none(gobject_ffi::g_binding_get_target(self.to_glib_none().0))
        }
    }

    fn get_source_property(&self) -> String {
        unsafe {
            from_glib_none(gobject_ffi::g_binding_get_source_property(self.to_glib_none().0))
        }
    }

    fn get_target(&self) -> Object {
        unsafe {
            from_glib_none(gobject_ffi::g_binding_get_target(self.to_glib_none().0))
        }
    }

    fn get_target_property(&self) -> String {
        unsafe {
            from_glib_none(gobject_ffi::g_binding_get_target_property(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_38"))]
    fn unbind(&self) {
        unsafe {
            gobject_ffi::g_binding_unbind(self.to_glib_none().0);
        }
    }
}
