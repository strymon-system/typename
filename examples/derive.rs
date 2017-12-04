// Copyright 2017 ETH Zurich. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
extern crate typename;

use typename::TypeName;

#[derive(TypeName)]
struct Custom<T: TypeName> {
    _t: T,
}

fn main() {
    assert_eq!(
        Custom::<i32>::type_name(),
        concat!(module_path!(), "::", "Custom<i32>")
    );
    let c = Custom { _t: 3.14 };
    println!("{}", c.type_name_of());
}
