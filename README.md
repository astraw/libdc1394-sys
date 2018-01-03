# libdc1394-sys - rust FFI bindings for libdc1394 [![Version][version-img]][version-url] [![Status][status-img]][status-url]

## Overview

This crate has FFI bindings to libdc1394.

## Prerequisites

Requires libdc1394-2 and libusb-1.0. On Ubuntu 16.04, this can be installed with
`apt-get install libdc1394-22-dev libusb-1.0-0-dev`. Also requires Rust
`bindgen` and `pkg-config` crates and their prerequisites.

## Building

    cargo build

## License

Licensed under either of

* Apache License, Version 2.0,
  (./LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license (./LICENSE-MIT or http://opensource.org/licenses/MIT)
  at your option.

Note that libdc1394 itself is licensed under the GNU LGPL.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.

## Code of conduct

Anyone who interacts with libdc1394-sys in any space including but not limited to
this GitHub repository is expected to follow our
[code of conduct](https://github.com/astraw/libdc1394-sys/blob/master/code_of_conduct.md)

[status-img]: https://travis-ci.org/astraw/libdc1394-sys.svg?branch=master
[status-url]: https://travis-ci.org/astraw/libdc1394-sys
[version-img]: https://img.shields.io/crates/v/libdc1394-sys.svg
[version-url]: https://crates.io/crates/libdc1394-sys
