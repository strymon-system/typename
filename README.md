typename
========

[![Build Status](https://travis-ci.org/strymon-system/typename.svg?branch=master)](https://travis-ci.org/strymon-system/typename)
[![Latest Version](https://img.shields.io/crates/v/typename.svg)](https://crates.io/crates/typename)
[![Documentation](https://docs.rs/typename/badge.svg)](https://docs.rs/typename)

A compatible, safe and stable alternative to Rust's
[`std::intrinsics::type_name`](https://doc.rust-lang.org/std/intrinsics/fn.type_name.html)
intrinsic.


> **DEPRECATION NOTICE**: This crate has been deprecated. The `type_name` intrinsic has been stablized in [Rust 1.38](https://blog.rust-lang.org/2019/09/26/Rust-1.38.0.html#std:any::type_name). Users of this crate are asked to migrate to [`std::any::type_name`](https://doc.rust-lang.org/std/any/fn.type_name.html).

# Example

```rust
extern crate typename;

use typename::TypeName;

fn main() {
    assert_eq!(String::type_name(), "std::string::String");
    assert_eq!(Vec::<i32>::type_name(), "std::vec::Vec<i32>");
    assert_eq!([0, 1, 2].type_name_of(), "[i32; 3]");
}
```

## Contribute

`typename` is part of the [Strymon](https://strymon-system.github.io/) project:

 - [Contribution Guide](https://strymon-system.github.io/docs/how-to-contribute)
 - [Mailing list](https://lists.inf.ethz.ch/mailman/listinfo/strymon-users)

## License

`typename` is primarily distributed under the terms of both the MIT license and the Apache License (Version 2.0), with portions covered by various BSD-like licenses.

See LICENSE-APACHE, and LICENSE-MIT for details.
