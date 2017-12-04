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

extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[doc(hidden)]
#[proc_macro_derive(TypeName)]
pub fn derive_topic_type(input: TokenStream) -> TokenStream {
    let source = input.to_string();
    let ast = syn::parse_derive_input(&source).unwrap();
    let expanded = impl_topic_type(&ast);
    expanded.parse().unwrap()
}

fn impl_topic_type(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    let ty_params = ast.generics.ty_params.iter().map(|p| &p.ident);
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    quote! {
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
    }
}
