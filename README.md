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

This project uses [GPL-3.0+](https://www.gnu.org/licenses/gpl-3.0.html).

Copyright (C) 2020-2021 Anthony Martinez
