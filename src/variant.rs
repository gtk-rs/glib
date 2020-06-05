// Copyright 2013-2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! `Variant` binding and helper traits.
//!
//! [`Variant`](struct.Variant.html) is an immutable dynamically-typed generic
//! container. Its type and value are defined at construction and never change.
//!
//! `Variant` types are described by [`VariantType`](../struct.VariantType.html)
//! "type strings".
//!
//! Although `GVariant` supports arbitrarily complex types, this binding is
//! currently limited to the basic ones: `bool`, `u8`, `i16`, `u16`, `i32`,
//! `u32`, `i64`, `u64`, `f64`, `&str`/`String`, and [`VariantDict`](../struct.VariantDict.html).
//!
//! # Examples
//!
//! ```
//! use glib::prelude::*; // or `use gtk::prelude::*;`
//! use glib::Variant;
//!
//! // Using the `ToVariant` trait.
//! let num = 10.to_variant();
//!
//! // `is` tests the type of the value.
//! assert!(num.is::<i32>());
//!
//! // `get` tries to extract the value.
//! assert_eq!(num.get::<i32>(), Some(10));
//! assert_eq!(num.get::<u32>(), None);
//!
//! // `Variant` implements `From`
//! let hello = Variant::from("Hello!");
//!
//! // `get_str` tries to borrow a string slice.
//! assert_eq!(hello.get_str(), Some("Hello!"));
//! assert_eq!(num.get_str(), None);
//! ```

use bytes::Bytes;
use glib_sys;
use gobject_sys;
use gstring::GString;
use std::borrow::Cow;
use std::cmp::{Eq, Ordering, PartialEq, PartialOrd};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::slice;
use std::str;
use translate::*;
use value;
use StaticType;
use Type;
use Value;
use VariantTy;
use VariantType;

glib_wrapper! {
    /// A generic immutable value capable of carrying various types.
    ///
    /// See the [module documentation](index.html) for more details.
    pub struct Variant(Shared<glib_sys::GVariant>);

    match fn {
        ref => |ptr| glib_sys::g_variant_ref_sink(ptr),
        unref => |ptr| glib_sys::g_variant_unref(ptr),
    }
}

impl StaticType for Variant {
    fn static_type() -> Type {
        Type::Variant
    }
}

#[doc(hidden)]
impl<'a> value::FromValueOptional<'a> for Variant {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        from_glib_full(gobject_sys::g_value_dup_variant(
            ToGlibPtr::to_glib_none(value).0,
        ))
    }
}

#[doc(hidden)]
impl value::SetValue for Variant {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_variant(
            ToGlibPtrMut::to_glib_none_mut(value).0,
            ToGlibPtr::<*mut glib_sys::GVariant>::to_glib_none(this).0,
        )
    }
}

#[doc(hidden)]
impl value::SetValueOptional for Variant {
    unsafe fn set_value_optional(value: &mut Value, this: Option<&Self>) {
        gobject_sys::g_value_set_variant(
            ToGlibPtrMut::to_glib_none_mut(value).0,
            ToGlibPtr::<*mut glib_sys::GVariant>::to_glib_none(&this).0,
        )
    }
}

impl Variant {
    /// Returns the type of the value.
    pub fn type_(&self) -> &VariantTy {
        unsafe { VariantTy::from_ptr(glib_sys::g_variant_get_type(self.to_glib_none().0)) }
    }

    /// Returns `true` if the type of the value corresponds to `T`.
    #[inline]
    pub fn is<T: StaticVariantType>(&self) -> bool {
        self.type_() == T::static_variant_type()
    }

    /// Tries to extract a value of type `T`.
    ///
    /// Returns `Some` if `T` matches the variant's type.
    #[inline]
    pub fn get<T: FromVariant>(&self) -> Option<T> {
        T::from_variant(self)
    }

    /// Tries to extract a `&str`.
    ///
    /// Returns `Some` if the variant has a string type (`s`, `o` or `g` type
    /// strings).
    pub fn get_str(&self) -> Option<&str> {
        unsafe {
            match self.type_().to_str() {
                "s" | "o" | "g" => {
                    let mut len = 0;
                    let ptr = glib_sys::g_variant_get_string(self.to_glib_none().0, &mut len);
                    let ret = str::from_utf8_unchecked(slice::from_raw_parts(
                        ptr as *const u8,
                        len as usize,
                    ));
                    Some(ret)
                }
                _ => None,
            }
        }
    }

    /// Constructs a new serialised-mode GVariant instance.
    pub fn new_from_bytes<T: StaticVariantType>(bytes: &Bytes) -> Self {
        unsafe {
            from_glib_none(glib_sys::g_variant_new_from_bytes(
                T::static_variant_type().as_ptr() as *const _,
                bytes.to_glib_none().0,
                false.to_glib(),
            ))
        }
    }

    /// Constructs a new serialised-mode GVariant instance.
    ///
    /// This is the same as `new_from_bytes`, except that checks on the passed
    /// data are skipped.
    ///
    /// You should not use this function on data from external sources.
    ///
    /// # Safety
    ///
    /// Since the data is not validated, this is potentially dangerous if called
    /// on bytes which are not guaranteed to have come from serialising another
    /// Variant.  The caller is responsible for ensuring bad data is not passed in.
    pub unsafe fn new_from_bytes_trusted<T: StaticVariantType>(bytes: &Bytes) -> Self {
        from_glib_none(glib_sys::g_variant_new_from_bytes(
            T::static_variant_type().as_ptr() as *const _,
            bytes.to_glib_none().0,
            true.to_glib(),
        ))
    }

    /// Returns the serialised form of a GVariant instance.
    pub fn get_data_as_bytes(&self) -> Bytes {
        unsafe { from_glib_full(glib_sys::g_variant_get_data_as_bytes(self.to_glib_none().0)) }
    }
}

unsafe impl Send for Variant {}
unsafe impl Sync for Variant {}

impl fmt::Debug for Variant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Variant")
            .field("ptr", &self.to_glib_none().0)
            .field("type", &self.type_())
            .field("value", &self.to_string())
            .finish()
    }
}

impl fmt::Display for Variant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let serialized: GString = unsafe {
            from_glib_full(glib_sys::g_variant_print(
                self.to_glib_none().0,
                false.to_glib(),
            ))
        };
        f.write_str(&serialized)
    }
}

impl PartialEq for Variant {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            from_glib(glib_sys::g_variant_equal(
                self.to_glib_none().0 as *const _,
                other.to_glib_none().0 as *const _,
            ))
        }
    }
}

impl Eq for Variant {}

impl PartialOrd for Variant {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        unsafe {
            if glib_sys::g_variant_classify(self.to_glib_none().0)
                != glib_sys::g_variant_classify(other.to_glib_none().0)
            {
                return None;
            }

            if glib_sys::g_variant_is_container(self.to_glib_none().0) != glib_sys::GFALSE {
                return None;
            }

            let res = glib_sys::g_variant_compare(
                self.to_glib_none().0 as *const _,
                other.to_glib_none().0 as *const _,
            );

            Some(res.cmp(&0))
        }
    }
}

impl Hash for Variant {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe { state.write_u32(glib_sys::g_variant_hash(self.to_glib_none().0 as *const _)) }
    }
}

/// Converts to `Variant`.
pub trait ToVariant {
    /// Returns a `Variant` clone of `self`.
    fn to_variant(&self) -> Variant;
}

/// Extracts a value.
pub trait FromVariant: Sized + StaticVariantType {
    /// Tries to extract a value.
    ///
    /// Returns `Some` if the variant's type matches `Self`.
    fn from_variant(variant: &Variant) -> Option<Self>;
}

/// Returns `VariantType` of `Self`.
pub trait StaticVariantType {
    /// Returns the `VariantType` corresponding to `Self`.
    fn static_variant_type() -> Cow<'static, VariantTy>;
}

impl<'a, T: ?Sized + ToVariant> ToVariant for &'a T {
    fn to_variant(&self) -> Variant {
        <T as ToVariant>::to_variant(self)
    }
}

impl<'a, T: ?Sized + StaticVariantType> StaticVariantType for &'a T {
    fn static_variant_type() -> Cow<'static, VariantTy> {
        <T as StaticVariantType>::static_variant_type()
    }
}

macro_rules! impl_numeric {
    ($name:ty, $type_str:expr, $new_fn:ident, $get_fn:ident) => {
        impl StaticVariantType for $name {
            fn static_variant_type() -> Cow<'static, VariantTy> {
                unsafe { VariantTy::from_str_unchecked($type_str).into() }
            }
        }

        impl ToVariant for $name {
            fn to_variant(&self) -> Variant {
                unsafe { from_glib_none(glib_sys::$new_fn(*self)) }
            }
        }

        impl FromVariant for $name {
            fn from_variant(variant: &Variant) -> Option<Self> {
                unsafe {
                    if variant.is::<Self>() {
                        Some(glib_sys::$get_fn(variant.to_glib_none().0))
                    } else {
                        None
                    }
                }
            }
        }
    };
}

impl_numeric!(u8, "y", g_variant_new_byte, g_variant_get_byte);
impl_numeric!(i16, "n", g_variant_new_int16, g_variant_get_int16);
impl_numeric!(u16, "q", g_variant_new_uint16, g_variant_get_uint16);
impl_numeric!(i32, "i", g_variant_new_int32, g_variant_get_int32);
impl_numeric!(u32, "u", g_variant_new_uint32, g_variant_get_uint32);
impl_numeric!(i64, "x", g_variant_new_int64, g_variant_get_int64);
impl_numeric!(u64, "t", g_variant_new_uint64, g_variant_get_uint64);
impl_numeric!(f64, "d", g_variant_new_double, g_variant_get_double);

impl StaticVariantType for bool {
    fn static_variant_type() -> Cow<'static, VariantTy> {
        unsafe { VariantTy::from_str_unchecked("b").into() }
    }
}

impl ToVariant for bool {
    fn to_variant(&self) -> Variant {
        unsafe { from_glib_none(glib_sys::g_variant_new_boolean(self.to_glib())) }
    }
}

impl FromVariant for bool {
    fn from_variant(variant: &Variant) -> Option<Self> {
        unsafe {
            if variant.is::<Self>() {
                Some(from_glib(glib_sys::g_variant_get_boolean(
                    variant.to_glib_none().0,
                )))
            } else {
                None
            }
        }
    }
}

impl StaticVariantType for String {
    fn static_variant_type() -> Cow<'static, VariantTy> {
        unsafe { VariantTy::from_str_unchecked("s").into() }
    }
}

impl ToVariant for String {
    fn to_variant(&self) -> Variant {
        self[..].to_variant()
    }
}

impl FromVariant for String {
    fn from_variant(variant: &Variant) -> Option<Self> {
        variant.get_str().map(String::from)
    }
}

impl StaticVariantType for str {
    fn static_variant_type() -> Cow<'static, VariantTy> {
        unsafe { VariantTy::from_str_unchecked("s").into() }
    }
}

impl ToVariant for str {
    fn to_variant(&self) -> Variant {
        unsafe { from_glib_none(glib_sys::g_variant_new_take_string(self.to_glib_full())) }
    }
}

impl<T: ToVariant> From<T> for Variant {
    fn from(value: T) -> Variant {
        value.to_variant()
    }
}

/// Returns `VariantType` of `Self`.
pub trait DynamicVariantType {
    /// Returns the `VariantType` corresponding to `self`.
    fn variant_type() -> VariantType;
}

impl<T: ?Sized + StaticVariantType> DynamicVariantType for T {
    fn variant_type() -> VariantType {
        T::static_variant_type().into_owned().to_owned()
    }
}

impl<T: DynamicVariantType> DynamicVariantType for [T] {
    fn variant_type() -> VariantType {
        let child_type = T::variant_type();
        let signature = format!("a{}", child_type.to_str());

        VariantType::new(&signature).expect("incorrect signature")
    }
}

impl<T: DynamicVariantType> DynamicVariantType for Vec<T> {
    fn variant_type() -> VariantType {
        <[T]>::variant_type()
    }
}

macro_rules! map_impls {
    ($name:ident) => {
        impl<K: StaticVariantType, V: DynamicVariantType> DynamicVariantType for std::collections::$name<K, V> {
            fn variant_type() -> VariantType {
                let key_type = K::static_variant_type();
                let value_type = V::variant_type();
                let signature = format!("a{{{}{}}}", key_type.to_str(), value_type.to_str());

                VariantType::new(&signature).expect("incorrect signature")
            }
        }
    }
}
map_impls!(HashMap);
map_impls!(HashSet);

macro_rules! tuple_impls {
    ($($len:expr => ($($n:tt $name:ident)+))+) => {
        $(
            impl<$($name),+> DynamicVariantType for ($($name,)+)
            where
                $($name: DynamicVariantType,)+
            {
                fn variant_type() -> VariantType {
                    let mut signature = String::with_capacity(255);
                    signature.push('(');
                    $(
                        signature.push_str($name::variant_type().to_str());
                    )+
                    signature.push(')');

                    VariantType::new(&signature).expect("incorrect signature")
                }
            }
        )+
    }
}

tuple_impls! {
    1 => (0 T0)
    2 => (0 T0 1 T1)
    3 => (0 T0 1 T1 2 T2)
    4 => (0 T0 1 T1 2 T2 3 T3)
    5 => (0 T0 1 T1 2 T2 3 T3 4 T4)
    6 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5)
    7 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6)
    8 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7)
    9 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8)
    10 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9)
    11 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10)
    12 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11)
    13 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12)
    14 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13)
    15 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14)
    16 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    macro_rules! unsigned {
        ($name:ident, $ty:ident) => {
            #[test]
            fn $name() {
                let mut n = $ty::max_value();
                while n > 0 {
                    let v = Variant::from(n);
                    assert_eq!(v.get(), Some(n));
                    n /= 2;
                }
            }
        };
    }

    macro_rules! signed {
        ($name:ident, $ty:ident) => {
            #[test]
            fn $name() {
                let mut n = $ty::max_value();
                while n > 0 {
                    let v = Variant::from(n);
                    assert_eq!(v.get(), Some(n));
                    let v = Variant::from(-n);
                    assert_eq!(v.get(), Some(-n));
                    n /= 2;
                }
            }
        };
    }

    unsigned!(test_u8, u8);
    unsigned!(test_u16, u16);
    unsigned!(test_u32, u32);
    unsigned!(test_u64, u64);
    signed!(test_i16, i16);
    signed!(test_i32, i32);
    signed!(test_i64, i64);

    #[test]
    fn test_str() {
        let s = "this is a test";
        let v = Variant::from(s);
        assert_eq!(v.get_str(), Some(s));
    }

    #[test]
    fn test_string() {
        let s = String::from("this is a test");
        let v = Variant::from(s.clone());
        assert_eq!(v.get(), Some(s));
    }

    #[test]
    fn test_eq() {
        let v1 = Variant::from("this is a test");
        let v2 = Variant::from("this is a test");
        let v3 = Variant::from("test");
        assert_eq!(v1, v2);
        assert!(v1 != v3);
    }

    #[test]
    fn test_hash() {
        let v1 = Variant::from("this is a test");
        let v2 = Variant::from("this is a test");
        let v3 = Variant::from("test");
        let mut set = HashSet::new();
        set.insert(v1);
        assert!(set.contains(&v2));
        assert!(!set.contains(&v3));

        assert_eq!(<HashSet<&str,(&str,u8,u32)>>::variant_type().to_str(), "a{s(syu)}");
    }

    #[test]
    fn test_array() {
        // Test just the signature for now.
        assert_eq!(<Vec<&str>>::variant_type().to_str(), "as");
        assert_eq!(<Vec<(&str,u8,u32)>>::variant_type().to_str(), "a(syu)");
    }
}
