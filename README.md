# About

`staart` is tail-like implementation in Rust.
The binary expects exactly one argument which is the file to be followed.
Tailing of the file starts at the end of the file, and continues from there.
If a file is rotated with a new file created with the same name the program
will continue following the named file, and not the previous version. In the
case of truncation, data are printed from the start of the file.

## Binary

`staart` offers a Rust std-lib only binary crate capable of following a
file. If the file is rotated, the new file will be followed. Following will
start at the beginning should truncation be detected. Usage is simple:

`staart <path/to/file.ext>`

The full feature set of `tail` is not replicated here. `staart` will always
start from the end of the file, and print all subsequently appearing lines
to `stdout`. If non-utf8 code points are found an error is printed to `stderr`.

If the path given to `staart` does not exist for three open attempts, the
application exits with status code 1.

## Library

`staart` can be used as a library exposing methods to the `TailedFile`
struct it creates should there be a need to follow a file from directly
within a more complicated application.

Documentation can be found [here](https://docs.rs/staart/).


## Roadmap

- [ ] Support following files in Windows environments
- [x] Customize error handling such that missing file handles do not crash the program

### License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

### Contact

To discuss features, offer assistance, or get help plese join the project's [Matrix room](https://matrix.to/#/#staart:txrx.staart.one).

Copyright (C) 2020-2021 Anthony Martinez
