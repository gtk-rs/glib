// Copyright 2015-2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <https://opensource.org/licenses/MIT>

//! Runtime type information.

use glib_sys;
use gobject_sys;
use translate::{
    from_glib, from_glib_none, FromGlib, FromGlibContainerAsVec, ToGlib, ToGlibContainerFromSlice,
    ToGlibPtr, ToGlibPtrMut,
};
use value::{FromValue, FromValueOptional, SetValue, Value};

use std::fmt;
use std::mem;
use std::ptr;

/// A GLib or GLib-based library type
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Type(glib_sys::GType);

impl Type {
    /// An invalid `Type` used as error return value in some functions
    pub const INVALID: Self = Self(gobject_sys::G_TYPE_INVALID);

    /// The fundamental type corresponding to the unit type `()`
    pub const UNIT: Self = Self(gobject_sys::G_TYPE_NONE);

    /// The fundamental type corresponding to `i8`
    pub const I8: Self = Self(gobject_sys::G_TYPE_CHAR);

    /// The fundamental type corresponding to `u8`
    pub const U8: Self = Self(gobject_sys::G_TYPE_UCHAR);

    /// The fundamental type corresponding to `bool`
    pub const BOOL: Self = Self(gobject_sys::G_TYPE_BOOLEAN);

    /// The fundamental type corresponding to `i32`
    pub const I32: Self = Self(gobject_sys::G_TYPE_INT);

    /// The fundamental type corresponding to `u32`
    pub const U32: Self = Self(gobject_sys::G_TYPE_UINT);

    /// The fundamental type corresponding to C `long`
    pub const I_LONG: Self = Self(gobject_sys::G_TYPE_LONG);

    /// The fundamental type corresponding to C `unsigned long`
    pub const U_LONG: Self = Self(gobject_sys::G_TYPE_ULONG);

    /// The fundamental type corresponding to `i64`
    pub const I64: Self = Self(gobject_sys::G_TYPE_INT64);

    /// The fundamental type corresponding to `u64`
    pub const U64: Self = Self(gobject_sys::G_TYPE_UINT64);

    /// The fundamental type corresponding to `f32`
    pub const F32: Self = Self(gobject_sys::G_TYPE_FLOAT);

    /// The fundamental type corresponding to `f64`
    pub const F64: Self = Self(gobject_sys::G_TYPE_DOUBLE);

    /// The fundamental type corresponding to `String`
    pub const STRING: Self = Self(gobject_sys::G_TYPE_STRING);

    /// The fundamental type corresponding to a pointer
    pub const POINTER: Self = Self(gobject_sys::G_TYPE_POINTER);

    /// The fundamental type of GVariant
    pub const VARIANT: Self = Self(gobject_sys::G_TYPE_VARIANT);

    /// The fundamental type from which all interfaces are derived
    pub const BASE_INTERFACE: Self = Self(gobject_sys::G_TYPE_INTERFACE);

    /// The fundamental type from which all enumeration types are derived
    pub const BASE_ENUM: Self = Self(gobject_sys::G_TYPE_ENUM);

    /// The fundamental type from which all flags types are derived
    pub const BASE_FLAGS: Self = Self(gobject_sys::G_TYPE_FLAGS);

    /// The fundamental type from which all boxed types are derived
    pub const BASE_BOXED: Self = Self(gobject_sys::G_TYPE_BOXED);

    /// The fundamental type from which all `GParamSpec` types are derived
    pub const BASE_PARAM_SPEC: Self = Self(gobject_sys::G_TYPE_PARAM);

    /// The fundamental type from which all objects are derived
    pub const BASE_OBJECT: Self = Self(gobject_sys::G_TYPE_OBJECT);

    pub fn name(&self) -> String {
        match self.0 {
            gobject_sys::G_TYPE_INVALID => "<invalid>".into(),
            _ => unsafe { from_glib_none(gobject_sys::g_type_name(self.to_glib())) },
        }
    }

    pub fn qname(&self) -> ::Quark {
        match self.0 {
            gobject_sys::G_TYPE_INVALID => ::Quark::from_string("<invalid>"),
            _ => unsafe { from_glib(gobject_sys::g_type_qname(self.to_glib())) },
        }
    }

    pub fn is_a(&self, other: &Self) -> bool {
        unsafe { from_glib(gobject_sys::g_type_is_a(self.to_glib(), other.to_glib())) }
    }

    pub fn parent(&self) -> Option<Self> {
        unsafe {
            let parent = gobject_sys::g_type_parent(self.to_glib());
            if parent == gobject_sys::G_TYPE_INVALID {
                None
            } else {
                Some(from_glib(parent))
            }
        }
    }

    pub fn children(&self) -> Vec<Self> {
        unsafe {
            let mut n_children = 0u32;
            let children = gobject_sys::g_type_children(self.to_glib(), &mut n_children);

            FromGlibContainerAsVec::from_glib_full_num_as_vec(children, n_children as usize)
        }
    }

    pub fn interfaces(&self) -> Vec<Self> {
        unsafe {
            let mut n_interfaces = 0u32;
            let interfaces = gobject_sys::g_type_interfaces(self.to_glib(), &mut n_interfaces);

            FromGlibContainerAsVec::from_glib_full_num_as_vec(interfaces, n_interfaces as usize)
        }
    }
    pub fn interface_prerequisites(&self) -> Vec<Self> {
        match self {
            t if !t.is_a(&Self::BASE_INTERFACE) => vec![],
            _ => unsafe {
                let mut n_prereqs = 0u32;
                let prereqs =
                    gobject_sys::g_type_interface_prerequisites(self.to_glib(), &mut n_prereqs);

                FromGlibContainerAsVec::from_glib_full_num_as_vec(prereqs, n_prereqs as usize)
            },
        }
    }

    pub fn from_name<'a, P: Into<&'a str>>(name: P) -> Option<Self> {
        unsafe {
            let type_ = gobject_sys::g_type_from_name(name.into().to_glib_none().0);
            if type_ == gobject_sys::G_TYPE_INVALID {
                None
            } else {
                Some(from_glib(type_))
            }
        }
    }

    pub fn is_valid(&self) -> bool {
        *self != Self::INVALID
    }
}

impl fmt::Debug for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.name())
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.name())
    }
}

/// Types that are supported by GLib dynamic typing.
pub trait StaticType {
    /// Returns the type identifier of `Self`.
    fn static_type() -> Type;
}

impl StaticType for Type {
    fn static_type() -> Type {
        unsafe { from_glib(gobject_sys::g_gtype_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for Type {
    unsafe fn from_value_optional(value: &'a Value) -> Option<Self> {
        Some(from_glib(gobject_sys::g_value_get_gtype(
            value.to_glib_none().0,
        )))
    }
}

impl<'a> FromValue<'a> for Type {
    unsafe fn from_value(value: &'a Value) -> Self {
        from_glib(gobject_sys::g_value_get_gtype(value.to_glib_none().0))
    }
}

impl SetValue for Type {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_gtype(value.to_glib_none_mut().0, this.to_glib())
    }
}

impl<'a, T: ?Sized + StaticType> StaticType for &'a T {
    fn static_type() -> Type {
        T::static_type()
    }
}

impl<'a, T: ?Sized + StaticType> StaticType for &'a mut T {
    fn static_type() -> Type {
        T::static_type()
    }
}

macro_rules! builtin {
    ($name:ident, $val:ident) => {
        impl StaticType for $name {
            fn static_type() -> Type {
                Type::$val
            }
        }
    };
}

builtin!(bool, BOOL);
builtin!(i8, I8);
builtin!(u8, U8);
builtin!(i32, I32);
builtin!(u32, U32);
builtin!(i64, I64);
builtin!(u64, U64);
builtin!(f32, F32);
builtin!(f64, F64);
builtin!(str, STRING);
builtin!(String, STRING);

impl<'a> StaticType for [&'a str] {
    fn static_type() -> Type {
        unsafe { from_glib(glib_sys::g_strv_get_type()) }
    }
}

impl StaticType for Vec<String> {
    fn static_type() -> Type {
        unsafe { from_glib(glib_sys::g_strv_get_type()) }
    }
}

#[inline]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn instance_of<C: StaticType>(ptr: glib_sys::gconstpointer) -> bool {
    from_glib(gobject_sys::g_type_check_instance_is_a(
        ptr as *mut _,
        <C as StaticType>::static_type().to_glib(),
    ))
}

impl FromGlib<glib_sys::GType> for Type {
    #[inline]
    fn from_glib(val: glib_sys::GType) -> Self {
        Self(val)
    }
}

impl ToGlib for Type {
    type GlibType = glib_sys::GType;

    fn to_glib(&self) -> Self::GlibType {
        self.0
    }
}

impl<'a> ToGlibContainerFromSlice<'a, *mut glib_sys::GType> for Type {
    type Storage = Option<Vec<glib_sys::GType>>;

    fn to_glib_none_from_slice(t: &'a [Type]) -> (*mut glib_sys::GType, Self::Storage) {
        let mut vec = t.iter().map(ToGlib::to_glib).collect::<Vec<_>>();

        (vec.as_mut_ptr(), Some(vec))
    }

    fn to_glib_container_from_slice(t: &'a [Type]) -> (*mut glib_sys::GType, Self::Storage) {
        (Self::to_glib_full_from_slice(t), None)
    }

    fn to_glib_full_from_slice(t: &[Type]) -> *mut glib_sys::GType {
        if t.is_empty() {
            return ptr::null_mut();
        }

        unsafe {
            let res = glib_sys::g_malloc0(mem::size_of::<glib_sys::GType>() * (t.len() + 1))
                as *mut glib_sys::GType;
            for (i, v) in t.iter().enumerate() {
                *res.add(i) = v.to_glib();
            }
            res
        }
    }
}

impl FromGlibContainerAsVec<Type, *const glib_sys::GType> for Type {
    unsafe fn from_glib_none_num_as_vec(ptr: *const glib_sys::GType, num: usize) -> Vec<Self> {
        if num == 0 || ptr.is_null() {
            return Vec::new();
        }

        let mut res = Vec::with_capacity(num);
        for i in 0..num {
            res.push(from_glib(*ptr.add(i)));
        }
        res
    }

    unsafe fn from_glib_container_num_as_vec(_: *const glib_sys::GType, _: usize) -> Vec<Self> {
        // Can't really free a *const
        unimplemented!();
    }

    unsafe fn from_glib_full_num_as_vec(_: *const glib_sys::GType, _: usize) -> Vec<Self> {
        // Can't really free a *const
        unimplemented!();
    }
}

impl FromGlibContainerAsVec<Type, *mut glib_sys::GType> for Type {
    unsafe fn from_glib_none_num_as_vec(ptr: *mut glib_sys::GType, num: usize) -> Vec<Self> {
        FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr as *const _, num)
    }

    unsafe fn from_glib_container_num_as_vec(ptr: *mut glib_sys::GType, num: usize) -> Vec<Self> {
        let res = FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr, num);
        glib_sys::g_free(ptr as *mut _);
        res
    }

    unsafe fn from_glib_full_num_as_vec(ptr: *mut glib_sys::GType, num: usize) -> Vec<Self> {
        FromGlibContainerAsVec::from_glib_container_num_as_vec(ptr, num)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{BTreeSet, HashSet};
    use InitiallyUnowned;

    #[test]
    fn invalid() {
        let invalid = Type::INVALID;

        assert_eq!(invalid.name(), "<invalid>");
        assert_eq!(invalid.qname(), ::Quark::from_string("<invalid>"));
        assert!(invalid.is_a(&Type::INVALID));
        assert!(!invalid.is_a(&Type::STRING));
        assert_eq!(invalid.parent(), None);
        assert_eq!(invalid.children(), vec![]);
        assert_eq!(invalid.interfaces(), vec![]);
        assert_eq!(invalid.interface_prerequisites(), vec![]);
        assert!(!invalid.is_valid());
        dbg!(&invalid);
    }

    #[test]
    fn hash() {
        // Get this first so the type is registered
        let iu_type = InitiallyUnowned::static_type();

        let set = Type::BASE_OBJECT
            .children()
            .into_iter()
            .collect::<HashSet<_>>();

        assert!(set.contains(&iu_type));
    }

    #[test]
    fn ord() {
        // Get this first so the type is registered
        let iu_type = InitiallyUnowned::static_type();
        assert!(Type::BASE_OBJECT < iu_type);

        let set = Type::BASE_OBJECT
            .children()
            .into_iter()
            .collect::<BTreeSet<_>>();

        assert!(set.contains(&iu_type));
    }
}
