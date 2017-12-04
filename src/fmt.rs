// Copyright 2017 ETH Zurich. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Hidden support module for formatting type names of the form
//! `some::path::Name<TArg1, TArg2>`. It will omit printing angle brackets
//! if no generic type parameters are provided.
//!
//! # Examples
//!
//! ```
//! use typename::{fmt, TypeName};
//!
//! struct Foo<T, U> {
//!     some_t: T,
//!     some_u: U,
//! }
//!
//! impl<T: TypeName, U: TypeName> TypeName for Foo<T, U> {
//!     fn fmt(f: &mut ::std::fmt::Formatter) -> std::fmt::Result {
//!         typename::fmt::TypeFormatter::new(f, concat!(module_path!(), "::Foo"))
//!             .type_param::<T>()
//!             .type_param::<U>()
//!             .finish()
//!     }
//! }
//!
//! fn main() {
//!     assert_eq!(Foo::<i32, f32>::type_name(), concat!(module_path!(), "::Foo<i32, f32>"));
//! }
//! ```
use std::fmt;

use super::TypeName;

/// A builder struct for formatting type names.
///
/// It's interface is very similar to the debug buildes in `std::fmt`, e.g.
/// `DebugTuple`.
pub struct TypeFormatter<'a, 'b: 'a> {
    fmt: &'a mut fmt::Formatter<'b>,
    result: fmt::Result,
    has_params: bool,
}

impl<'a, 'b: 'a> TypeFormatter<'a, 'b> {
    /// Create a new builder which eagerliy writes to the provided
    /// `Formatter`.
    pub fn new(fmt: &'a mut fmt::Formatter<'b>, name: &str) -> Self {
        let result = fmt.write_str(name);
        TypeFormatter {
            fmt,
            result,
            has_params: false,
        }
    }

    /// Attaches the type name of the provided type argument to the type name.
    pub fn type_param<T: TypeName>(&mut self) -> &mut Self {
        self.result = self.result.and_then(|_| {
            if self.has_params {
                self.fmt.write_str(", ")?
            } else {
                self.fmt.write_str("<")?
            };

            <T as TypeName>::fmt(self.fmt)
        });

        self.has_params = true;
        self
    }

    /// Finishes output and returns any error encountered.
    pub fn finish(&mut self) -> fmt::Result {
        if self.has_params {
            self.result = self.result.and_then(|_| self.fmt.write_str(">"));
        }
        self.result
    }
}
