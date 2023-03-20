# Rust Decklink

Blackmagic Design Decklink SDK bindings for Rust.  
This aims to be an easy to use and safe wrapper around the official C++ SDK.
This makes use of a [C wrapper](https://github.com/Julusian/decklink-c) as rust does not support using some of the needed C++ directly.

Note: This is very incomplete, but is working. The examples try to follow the official examples as closely as possible

Currently only tested on Linux x64, other platforms will likely need some work. Help on that is appreciated!

## Installation


You will need to have the Decklink drivers (>=10.9.12) installed on your machine and a compatible device connected to do much with it.

CMake is also required for cargo to build the C library.

## Usage

See the examples for more information.

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.