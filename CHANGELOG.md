# Change Log

All user visible changes to this project will be documented in this file.
This project adheres to [Semantic Versioning](http://semver.org/), as described
for Rust libraries in [RFC #1105](https://github.com/rust-lang/rfcs/blob/master/text/1105-api-evolution.md)

## [0.2.0] - 2018-01-04

### Changed

* Depend on `libusb-sys` crate rather than directly linking to libusb ourselves.
* Generate bindings using `bindgen` at compile time, rather than checked into
  repo. This also added use of the bindgen option `constified_enum_module` which
  changes the API of the bindings slightly.
* Require use of `pkg-config` to get link arguments.

## [0.1.0] - 2017-04-14

### Added

* Initial release

[0.2.0]: https://github.com/astraw/libdc1394-sys/compare/0.1.0...0.2.0
