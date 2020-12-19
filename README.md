# About

`staart` is tail-like implementation in Rust for Linux systems.
The binary expects exactly one argument which is the file to be followed.
Tailing of the file starts at the end of the file, and continues from there.
If a file is rotated with a new file created with the same name the program
will continue following the named file, and not the previous version. The
program will crash if non-utf8 characters are encountered during tailing.

## Status

This program, and the accompanying library, were developed out of necessity
and to satisfy a specific use case. It works for the author, but may not
work for you. Issues will be fixed if and when the author has the time, but
all constructive comments and suggestions are appreciated.

Consider this beta software.

### TODO

- Write documentation for the library
- Write documentation for the binary
- Write tests

### Notes

This project started life as `tail_chaser`. This particular name was not
intentionally offensive, but very well could be taken that way. As such,
the author has renamed the project to the Dutch word for `tail`. There
were no changes beyond the name between `0.1.3` in the original project
and `0.1.4` in the new, so any uses of the binary or library crates should
be unaffected. The primary difference is that all future maintenance of
the project will be under the new name.

At the same time as the name change, the project source was moved from
GitHub to GitLab.

### License

This project uses [GPL-3.0+](https://www.gnu.org/licenses/gpl-3.0.html).

Copyright (C) 2020 Anthony Martinez
