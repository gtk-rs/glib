// Copyright 2013-2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! # **glib**, **gobject** and **gio** bindings for Rust
//!
//! This library contains
//!
//! - bindings to some essential GLib, GObject, GIO types and APIs,
//!
//! - common building blocks used in both handmade and machine generated
//! bindings to GTK+ and other GLib-based libraries.
//!
//! It is the foundation for higher level libraries with uniform Rusty (safe and
//! strongly typed) APIs. It avoids exposing GLib-specific data types where
//! possible and is not meant to provide comprehensive GLib bindings, which
//! would often amount to duplicating the Rust Standard Library or other utility
//! crates.
//!
//! The library is a work in progress: expect missing functionality and breaking
//! changes.
//!
//! # Dynamic typing
//!
//! Most types in the GLib family have type identifiers
//! ([`Type`](types/enum.Type.html)). Their corresponding Rust types implement
//! the [`StaticType`](types/trait.StaticType.html) trait.
//!
//! Dynamically typed [`Value`](value/index.html) can carry values of any `T:
//! StaticType`.
//!
//! [`Variant`](variant/index.html) can carry values of `T: StaticVariantType`.
//!
//! # Errors
//!
//! Errors are represented by [`Error`](error/struct.Error.html), which can
//! carry values from various [error
//! domains](error/trait.ErrorDomain.html#implementors) (such as
//! [`FileError`](enum.FileError.html)).
//!
//! # Objects
//!
//! Each class and interface has a corresponding smart pointer struct
//! representing an instance of that type (e.g. `Object` for `GObject`,
//! `gtk::Widget` for `GtkWidget`). They are reference counted and feature
//! interior mutability similarly to Rust's `Rc<RefCell<T>>` idiom.
//! Consequently, cloning objects is cheap and their methods never require
//! mutable borrows. Two smart pointers are equal iff they point to the same
//! object.
//!
//! The root of the object hierarchy is [`Object`](object/struct.Object.html).
//! Inheritance and subtyping is denoted with the [`IsA`](object/trait.IsA.html)
//! marker trait. The [`Cast`](object/trait.Cast.html) trait enables upcasting
//! and downcasting.
//!
//! Interfaces and non-leaf classes also have corresponding traits (e.g.
//! `ObjectExt` and `gtk::WidgetExt`), which are blanketly implemented for all
//! their subtypes.
//!
//! For creating new subclasses of `Object` or other object types this crate has to be compiled
//! with the `subclassing` feature to enable the [`subclass`](subclass/index.html) module. Check
//! the module's documentation for further details and a code example.
//!
//! # Under the hood
//!
//! GLib-based libraries largely operate on pointers to various boxed or
//! reference counted structures so the bindings have to implement corresponding
//! smart pointers (wrappers), which encapsulate resource management and safety
//! checks. Such wrappers are defined via the
//! [`glib_wrapper!`](macro.glib_wrapper!.html) macro, which uses abstractions
//! defined in the [`wrapper`](wrapper/index.html), [`boxed`](boxed/index.html),
//! [`shared`](shared/index.html) and [`object`](object/index.html) modules.
//!
//! The [`translate`](translate/index.html) module defines and partly implements
//! conversions between high level Rust types (including the aforementioned
//! wrappers) and their FFI counterparts.

#![allow(clippy::doc_markdown)]
#![allow(clippy::unreadable_literal)]

#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate lazy_static;
extern crate libc;

#[doc(hidden)]
pub extern crate glib_sys;
#[doc(hidden)]
pub extern crate gobject_sys;

#[cfg(feature = "futures")]
pub extern crate futures;

use std::ffi::CStr;
pub use bytes::Bytes;
pub use string::String;
pub use closure::Closure;
pub use error::{Error, BoolError};
pub use file_error::FileError;
pub use object::{
    Cast,
    IsA,
    IsClassFor,
    Object,
    ObjectExt,
    ObjectClass,
    ObjectType,
    InitiallyUnowned,
    InitiallyUnownedClass,
    WeakRef,
    SendWeakRef,
};
pub use signal::{
    SignalHandlerId,
    signal_handler_block,
    signal_handler_disconnect,
    signal_handler_unblock,
    signal_stop_emission_by_name
};

pub use types::{
    StaticType,
    Type,
};
pub use value::{
    ToValue,
    ToSendValue,
    TypedValue,
    SendValue,
    Value,
};
pub use variant::{
    StaticVariantType,
    ToVariant,
    Variant,
};
pub use variant_type::{
    VariantTy,
    VariantType,
};
pub use time_val::{
    TimeVal,
    get_current_time,
};
pub use enums::{
    UserDirectory,
    EnumClass,
    EnumValue,
    FlagsClass,
    FlagsValue,
    FlagsBuilder,
};

#[macro_use]
pub mod wrapper;
#[macro_use]
pub mod boxed;
#[macro_use]
pub mod shared;
#[macro_use]
pub mod error;
#[macro_use]
pub mod object;

pub use auto::*;
pub use auto::functions::*;
#[allow(clippy::let_and_return)]
#[allow(clippy::let_unit_value)]
#[allow(clippy::too_many_arguments)]
#[allow(non_upper_case_globals)]
mod auto;

pub use gobject::*;
mod gobject;

mod bytes;
mod string;
pub mod char;
pub use char::*;
mod checksum;
pub mod closure;
mod enums;
mod file_error;
mod key_file;
pub mod prelude;
pub mod signal;
pub mod source;
pub use source::*;
mod time_val;
#[macro_use]
pub mod translate;
mod gstring;
pub use gstring::GString;
pub mod types;
mod utils;
pub use utils::*;
pub mod value;
pub mod variant;
mod variant_type;
mod main_context;
mod main_context_channel;
pub use main_context_channel::{Sender, SyncSender, Receiver};
mod date;
pub use date::Date;
mod value_array;
pub use value_array::ValueArray;
mod param_spec;
pub use param_spec::ParamSpec;
mod quark;
pub use quark::Quark;
mod rec_mutex;

pub mod send_unique;
pub use send_unique::{
    SendUniqueCell,
    SendUnique,
};

#[cfg(feature="futures")]
mod main_context_futures;
#[cfg(feature="futures")]
mod source_futures;
#[cfg(feature="futures")]
pub use source_futures::*;

// Actual thread IDs can be reused by the OS once the old thread finished.
// This works around it by using our own counter for threads.
//
// Taken from the fragile crate
use std::sync::atomic::{AtomicUsize, Ordering};
fn next_thread_id() -> usize {
    static mut COUNTER: AtomicUsize = AtomicUsize::new(0);
    unsafe { COUNTER.fetch_add(1, Ordering::SeqCst) }
}

pub(crate) fn get_thread_id() -> usize {
    thread_local!(static THREAD_ID: usize = next_thread_id());
    THREAD_ID.with(|&x| x)
}

#[macro_use]
#[cfg(any(feature = "dox", feature="subclassing"))]
pub mod subclass;
