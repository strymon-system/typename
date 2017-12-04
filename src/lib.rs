// Copyright 2017 ETH Zurich. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! This crate provides a compatible, safe and stable alternative to Rust's
//! [`std::intrinsics::type_name`](https://doc.rust-lang.org/std/intrinsics/fn.type_name.html)
//! intrinsic. This is achieved through the [`TypeName`](trait.TypeName.html)
//! trait which is implemented for most types in `std` and can be implemented
//! manually for custom types.
//!
//! This crate also exposes a procedural macro to automatically derive the
//! trait using `#[derive(TypeName)]`. This is an optional dependency which can
//! be disabled by opting out from the `derive` feature in Cargo.
//!
//! # Examples
//!
//! ```
//! use typename::TypeName;
//!
//! fn main() {
//!     assert_eq!(String::type_name(), "std::string::String");
//!     assert_eq!(Vec::<i32>::type_name(), "std::vec::Vec<i32>");
//!     assert_eq!([0, 1, 2].type_name_of(), "[i32; 3]");
//! }
//! ```
//!
//! You can derive the `TypeName` trait for custom types as follows:
//!
//! ```rust,ignore
//! #[macro_use] extern crate typename;
//!
//! #[derive(TypeName)]
//! struct Custom<T: TypeName> {
//!     some_t: T,
//! }
//! ```
//! # Prior work
//!
//! This crate is inspired by the [`named_type`](https://docs.rs/named_type) crate
//! which provides a similar interface. However, its output is not compatible with
//! the `type_name` intrinsic, as its name does not contain the names of the
//! concrete instanstances of the type parameters.

#![cfg_attr(test, feature(test, core_intrinsics))]

#[warn(missing_docs)]
#[cfg(feature = "derive")]
#[allow(unused_imports)]
#[macro_use]
extern crate typename_derive;
#[cfg(feature = "derive")]
#[doc(hidden)]
pub use typename_derive::*;

use std::marker::PhantomData;

mod imp;
#[doc(hidden)]
pub mod fmt;

/// Trait which returns the canonical name of the implementing type.
///
/// ## Derivable
///
/// This trait can be used with `#[derive(TypeName)]`. Any generic parameters
/// to a type must implement `TypeName` as well.
///
/// ## How can I implement `TypeName`?
///
/// `TypeName` requires that the `fmt` function formats the type name exactly
/// the same way as returned by the
/// [`std::intrinsics::type_name`](https://doc.rust-lang.org/std/intrinsics/fn.type_name.html)
/// intrinsic. It requires that the name contains the fully qualified path to the type
/// (which can be obtained through the `module_path!()` macro), as well as
/// the concrete values of its generic parameters.
pub trait TypeName {
    /// Formats the fully qualified type name using the given formatter.
    fn fmt(f: &mut std::fmt::Formatter) -> std::fmt::Result;

    /// Returns the canoncial, concrete name of a type as a string.
    /// # Examples
    ///
    /// ```
    /// use typename::TypeName;
    ///
    /// assert_eq!(String::type_name(), "std::string::String");
    /// ```
    fn type_name() -> String {
        DisplayType::<Self>(PhantomData).to_string()
    }

    /// Returns the canoncial type of a value as a string.
    /// # Examples
    ///
    /// ```
    /// use typename::TypeName;
    ///
    /// assert_eq!(vec![0, 1, 2].type_name_of(), "std::vec::Vec<i32>");
    /// ```
    fn type_name_of(&self) -> String {
        Self::type_name()
    }
}

/// Wrapper type which implements `Display` to call into `TypeName`
struct DisplayType<T: ?Sized>(PhantomData<T>);

/// A `Display` impl is probably the easiest way to format into a `String`
impl<T: TypeName + ?Sized> std::fmt::Display for DisplayType<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        <T as TypeName>::fmt(f)
    }
}
