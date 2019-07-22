// Copyright 2017 ETH Zurich. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! This module contains all the implementations of `TypeName` for the
//! different types found in `std`.
use std;

macro_rules! derive_type_name {
    ( $prefix:ident $(:: $suffix:ident)* $(<$($ty_param:ident),+>)*) => {
        impl$(<$($ty_param: $crate::TypeName),*>)* $crate::TypeName
            for $prefix $(:: $suffix)* $(<$($ty_param),*>)*
        {
            fn fmt(fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                let _ty_name = concat!(stringify!($prefix), $("::", stringify!($suffix)),*);
                $crate::fmt::TypeFormatter::new(fmt, _ty_name)
                    $( $(.type_param::<$ty_param>() )* )*
                    .finish()
            }
        }
    }
}

macro_rules! derive_tuple_type_name {
    ( $head:ident, $($tail:ident),+ ) => {
        impl<$head: $crate::TypeName, $($tail: $crate::TypeName),+>
            $crate::TypeName
            for ( $head, $($tail,)+ )
        {
            fn fmt(fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                fmt.write_str("(")?;
                $head::fmt(fmt)?;
                $(
                    fmt.write_str(", ")?;
                    $tail::fmt(fmt)?;
                )+
                fmt.write_str(")")
            }
        }
    }
}

macro_rules! derive_array_type_name {
    ($size:expr) => {
        impl<T: $crate::TypeName> $crate::TypeName for [T; $size] {
            fn fmt(fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                fmt.write_str("[")?;
                T::fmt(fmt)?;
                fmt.write_str(concat!("; ", $size, "]"))
            }
        }
    }
}

derive_type_name!(u8);
derive_type_name!(u16);
derive_type_name!(u32);
derive_type_name!(u64);
derive_type_name!(usize);

derive_type_name!(i8);
derive_type_name!(i16);
derive_type_name!(i32);
derive_type_name!(i64);
derive_type_name!(isize);

derive_type_name!(f32);
derive_type_name!(f64);

derive_type_name!(char);
derive_type_name!(bool);
derive_type_name!(str);

impl<'a, T: ::TypeName> ::TypeName for &'a T {
    fn fmt(fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        fmt.write_str("&")?;
        T::fmt(fmt)
    }
}

impl<'a, T: ::TypeName> ::TypeName for &'a mut T {
    fn fmt(fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        fmt.write_str("&mut ")?;
        T::fmt(fmt)
    }
}

derive_type_name!(std::string::String);
derive_type_name!(std::marker::PhantomData<T>);
derive_type_name!(std::vec::Vec<T>);
derive_type_name!(std::collections::HashMap<K, V>);
derive_type_name!(std::boxed::Box<T>);
derive_type_name!(std::option::Option<T>);
derive_type_name!(std::result::Result<T, E>);
derive_type_name!(std::rc::Rc<T>);
derive_type_name!(std::path::PathBuf);
derive_type_name!(std::sync::Arc<T>);
derive_type_name!(std::sync::RwLock<T>);
derive_type_name!(std::sync::mpsc::Receiver<T>);
derive_type_name!(std::sync::mpsc::Sender<T>);

// the empty and singleton tuple are special cases
impl ::TypeName for () {
    fn fmt(fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        fmt.write_str("()")
    }
}

impl<A: ::TypeName> ::TypeName for (A,) {
    fn fmt(fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        fmt.write_str("(")?;
        A::fmt(fmt)?;
        fmt.write_str(",)")
    }
}

derive_tuple_type_name!(A, B);
derive_tuple_type_name!(A, B, C);
derive_tuple_type_name!(A, B, C, D);
derive_tuple_type_name!(A, B, C, D, E);
derive_tuple_type_name!(A, B, C, D, E, F);
derive_tuple_type_name!(A, B, C, D, E, F, G);
derive_tuple_type_name!(A, B, C, D, E, F, G, H);
derive_tuple_type_name!(A, B, C, D, E, F, G, H, I);
derive_tuple_type_name!(A, B, C, D, E, F, G, H, I, J);
derive_tuple_type_name!(A, B, C, D, E, F, G, H, I, J, K);
derive_tuple_type_name!(A, B, C, D, E, F, G, H, I, J, K, L);
derive_tuple_type_name!(A, B, C, D, E, F, G, H, I, J, K, L, M);
derive_tuple_type_name!(A, B, C, D, E, F, G, H, I, J, K, L, M, N);
derive_tuple_type_name!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O);
derive_tuple_type_name!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P);
derive_tuple_type_name!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q);
derive_tuple_type_name!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R);
derive_tuple_type_name!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S);
derive_tuple_type_name!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T);
derive_tuple_type_name!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U);
derive_tuple_type_name!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V);
derive_tuple_type_name!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W);
derive_tuple_type_name!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X);
derive_tuple_type_name!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y);
derive_tuple_type_name!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z);
derive_tuple_type_name!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, AA);
derive_tuple_type_name!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, AA, AB);
derive_tuple_type_name!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, AA, AB, AC);
derive_tuple_type_name!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, AA, AB, AC, AD);
derive_tuple_type_name!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, AA, AB, AC, AD, AE);
derive_tuple_type_name!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, AA, AB, AC, AD, AE, AF);

derive_array_type_name!(0);
derive_array_type_name!(1);
derive_array_type_name!(2);
derive_array_type_name!(3);
derive_array_type_name!(4);
derive_array_type_name!(5);
derive_array_type_name!(6);
derive_array_type_name!(7);
derive_array_type_name!(8);
derive_array_type_name!(9);
derive_array_type_name!(10);
derive_array_type_name!(11);
derive_array_type_name!(12);
derive_array_type_name!(13);
derive_array_type_name!(14);
derive_array_type_name!(15);
derive_array_type_name!(16);
derive_array_type_name!(17);
derive_array_type_name!(18);
derive_array_type_name!(19);
derive_array_type_name!(20);
derive_array_type_name!(21);
derive_array_type_name!(22);
derive_array_type_name!(23);
derive_array_type_name!(24);
derive_array_type_name!(25);
derive_array_type_name!(26);
derive_array_type_name!(27);
derive_array_type_name!(28);
derive_array_type_name!(29);
derive_array_type_name!(30);
derive_array_type_name!(31);
derive_array_type_name!(32);

#[cfg(test)]
mod tests {
    use TypeName;
    use std::intrinsics::type_name;

    fn assert_eq_name<T: TypeName>() {
        assert_eq!(unsafe { type_name::<T>() }, T::type_name());
    }

    #[test]
    fn primitive_types() {
        assert_eq_name::<u8>();
        assert_eq_name::<u16>();
        assert_eq_name::<u32>();
        assert_eq_name::<u64>();
        assert_eq_name::<usize>();

        assert_eq_name::<i8>();
        assert_eq_name::<i16>();
        assert_eq_name::<i32>();
        assert_eq_name::<i64>();
        assert_eq_name::<isize>();

        assert_eq_name::<f32>();
        assert_eq_name::<f64>();

        assert_eq_name::<char>();
        assert_eq_name::<bool>();
        assert_eq_name::<()>();
    }


    #[test]
    fn array_types() {
        assert_eq_name::<[i32; 0]>();
        assert_eq_name::<[bool; 4]>();
        assert_eq_name::<[(char, i32); 10]>();
        assert_eq_name::<&mut [usize; 32]>();
    }

    #[test]
    fn ref_types() {
        assert_eq_name::<&u8>();
        assert_eq_name::<&mut i32>();
        // FIXME(swicki): DSTs currently don't work
        //assert_eq_name::<&str>();
        //assert_eq_name::<&mut [T]>();
    }

    #[test]
    fn tuples_types() {
        assert_eq_name::<(i32,)>();
        assert_eq_name::<(i32, i32)>();
        assert_eq_name::<(i32, f32, u8)>();
        assert_eq_name::<(i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8)>();
    }

    #[test]
    fn std_types() {
        assert_eq_name::<Box<i32>>();
        assert_eq_name::<Vec<String>>();
        assert_eq_name::<&mut Box<Vec<(bool, i32)>>>();
        assert_eq_name::<[::std::rc::Rc<i32>; 5]>();
        assert_eq_name::<::std::marker::PhantomData<&i32>>();
        assert_eq_name::<::std::sync::Arc<i32>>();
        assert_eq_name::<::std::sync::mpsc::Receiver<i32>>();
        assert_eq_name::<::std::sync::mpsc::Sender<i32>>();
        assert_eq_name::<::std::path::PathBuf>();
    }
}
