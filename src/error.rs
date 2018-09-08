// Copyright 2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! `Error` binding and helper trait.

use std::ffi::CStr;
use Quark;
use std::error;
use std::fmt;
use std::str;
use std::ptr;
use std::mem;
use translate::*;
use ffi as glib_ffi;
use gobject_ffi;

glib_wrapper! {
    /// A generic error capable of representing various error domains (types).
    #[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Error(Boxed<glib_ffi::GError>);

    match fn {
        copy => |ptr| glib_ffi::g_error_copy(ptr),
        free => |ptr| glib_ffi::g_error_free(ptr),
        get_type => || glib_ffi::g_error_get_type(),
    }
}

unsafe impl Send for Error {}
unsafe impl Sync for Error {}

impl Error {
    /// Creates an error with supplied error enum variant and message.
    pub fn new<T: ErrorDomain>(error: T, message: &str) -> Error {
        unsafe {
            from_glib_full(
                glib_ffi::g_error_new_literal(T::domain().to_glib(), error.code(), message.to_glib_none().0))
        }
    }

    /// Checks if the error domain matches `T`.
    pub fn is<T: ErrorDomain>(&self) -> bool {
        self.0.domain == T::domain().to_glib()
    }

    /// Tries to convert to a specific error enum.
    ///
    /// Returns `Some` if the error belongs to the enum's error domain and
    /// `None` otherwise.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// if let Some(file_error) = error.kind::<FileError>() {
    ///     match file_error {
    ///         FileError::Exist => ...
    ///         FileError::Isdir => ...
    ///         ...
    ///     }
    /// }
    /// ```
    ///
    /// ```ignore
    /// match error {
    ///     Some(FileError::Exist) => ...
    ///     Some(FileError::Isdir) => ...
    ///     ...
    /// }
    /// ```
    pub fn kind<T: ErrorDomain>(&self) -> Option<T> {
        if self.0.domain == T::domain().to_glib() {
            T::from(self.0.code)
        }
        else {
            None
        }
    }

    fn message(&self) -> &str {
        unsafe {
            let bytes = CStr::from_ptr(self.0.message).to_bytes();
            str::from_utf8(bytes).unwrap_or_else(|err| {
                str::from_utf8(&bytes[..err.valid_up_to()]).unwrap()
            })
        }
    }

    // backcompat shim
    #[cfg_attr(feature = "cargo-clippy", allow(not_unsafe_ptr_arg_deref))]
    pub fn wrap(ptr: *mut glib_ffi::GError) -> Error {
        unsafe { from_glib_full(ptr) }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.message())
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        self.message()
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Error")
            .field("domain", &::Quark::from_glib(self.0.domain))
            .field("code", &self.0.code)
            .field("message", &self.message())
            .finish()
    }
}

/// `GLib` error domain.
///
/// This trait is implemented by error enums that represent error domains (types).
pub trait ErrorDomain: Copy {
    /// Returns the quark identifying the error domain.
    ///
    /// As returned from `g_some_error_quark`.
    fn domain() -> Quark;

    /// Gets the integer representation of the variant.
    fn code(self) -> i32;

    /// Tries to convert an integer code to an enum variant.
    ///
    /// By convention, the `Failed` variant, if present, is a catch-all,
    /// i.e. any unrecognized codes map to it.
    fn from(code: i32) -> Option<Self> where Self: Sized;
}

/// Generic error used for functions that fail without any further information
#[derive(Debug)]
pub struct BoolError(pub &'static str);

impl BoolError {
    pub fn from_glib(b: glib_ffi::gboolean, s: &'static str) -> Result<(), Self> {
        match b {
            glib_ffi::GFALSE => Err(BoolError(s)),
            _ => Ok(()),
        }
    }
}

impl fmt::Display for BoolError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl error::Error for BoolError {
    fn description(&self) -> &str {
        self.0
    }
}
