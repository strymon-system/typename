// Copyright 2017 ETH Zurich. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
extern crate typename_derive;
extern crate typename;

use typename::TypeName;

#[derive(TypeName)]
struct UnitStruct;

#[derive(TypeName)]
enum SomeEnum {
    _A,
    _B,
}

// You don't need to add a TypeName bound to the struct's type parameter; we'll automatically add
// that bound to the TypeName impl that gets derived.
#[derive(Clone, TypeName)]
struct TupleStruct<T>(T);

// Test that we correctly add the TypeName bound to the impl even if there are other bounds on the
// type parameter, and even if you (now redundantly) specify a TypeName bound yourself.
#[derive(TypeName)]
struct Struct<T: Clone, S: Clone + TypeName> {
    _t: T,
    _s: S,
}

#[test]
fn check_type_name() {
    assert_eq!("tests::UnitStruct", UnitStruct::type_name());
    assert_eq!(
        "tests::TupleStruct<tests::UnitStruct>",
        TupleStruct::<UnitStruct>::type_name()
    );
    assert_eq!(
        "tests::Struct<tests::UnitStruct, i32>",
        Struct::<UnitStruct, i32>::type_name()
    );
    assert_eq!("tests::SomeEnum", SomeEnum::type_name());
}
