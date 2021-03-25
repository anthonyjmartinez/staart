# About

`staart` is tail-like implementation in Rust for Linux systems.
The binary expects exactly one argument which is the file to be followed.
Tailing of the file starts at the end of the file, and continues from there.
If a file is rotated with a new file created with the same name the program
will continue following the named file, and not the previous version. The
program will crash if non-utf8 code points are encountered during tailing.

## Binary

`staart` offers a Rust std-lib only binary crate capable of following a
file consiting of utf-8 code points. If the file is rotated, the new file
will be followed. Usage is simple:

`staart <path/to/file.ext>`

The full feature set of `tail` is not replicated here. `staart` will always
start from the end of the file, and print all subsequently appearing lines
to `stdout`.

## Library

`staart` can be used as a library exposing methods to the `TailedFile`
struct it creates should there be a need to follow a file from directly
within a more complicated application.

Documentation can be found [here](https://docs.rs/staart/).

### License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

Copyright (C) 2020-2021 Anthony Martinez
