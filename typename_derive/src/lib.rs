// Copyright 2017 ETH Zurich. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Procedural macro for deriving the `TypeName` trait.
//!
//! # Examples
//!
//! ```rust,ignore
//! use typename::TypeName;
//!
//! #[derive(TypeName)]
//! struct Custom<T: TypeName> {
//!     some_t: T,
//! }
//!
//! fn main() {
//!     assert_eq!(Custom::<i32>::type_name(), concat!(module_path!(), "::", "Custom<i32>"));
//! }

extern crate proc_macro;

#[macro_use]
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::DeriveInput;

#[doc(hidden)]
#[proc_macro_derive(TypeName)]
pub fn derive_topic_type(input: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let ty_params = ast
        .generics
        .type_params()
        .map(|p| p.ident.clone())
        .collect::<Vec<_>>();
    {
        // Add a T: TypeName bound to the impl for each type parameter in the type that we're
        // deriving TypeName for.
        let where_clause = ast.generics.make_where_clause();
        for ty_name in &ty_params {
            where_clause
                .predicates
                .push(parse_quote! { #ty_name: ::typename::TypeName });
        }
    }
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let expanded = quote! {
        impl #impl_generics ::typename::TypeName for #name #ty_generics #where_clause {
            fn fmt(f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                let _ty_name = concat!(module_path!(), "::", stringify!(#name));
                ::typename::fmt::TypeFormatter::new(f, _ty_name)
                    #(
                        .type_param::< #ty_params >()
                    )*
                    .finish()
            }
        }
    };
    TokenStream::from(expanded)
}
