// staart is a Rust implementation of a tail-like program for Linux
// Copyright 2020-2024 Anthony Martinez
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

/// Errors for staart
#[derive(Debug)]
pub enum StaartError {
    IO(std::io::Error),
    Utf8(std::str::Utf8Error),
    IntError(std::num::TryFromIntError),
}

impl std::fmt::Display for StaartError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            StaartError::IO(..) => {
                write!(f, "encountered IO error")
            }
            StaartError::Utf8(..) => {
                write!(f, "encountered UTF8 error")
            }
            StaartError::IntError(..) => {
                write!(f, "encountered integer conversion error")
            }
        }
    }
}

impl std::error::Error for StaartError {}

impl From<std::io::Error> for StaartError {
    fn from(err: std::io::Error) -> Self {
        StaartError::IO(err)
    }
}

impl From<std::str::Utf8Error> for StaartError {
    fn from(err: std::str::Utf8Error) -> Self {
        StaartError::Utf8(err)
    }
}

impl From<std::num::TryFromIntError> for StaartError {
    fn from(err: std::num::TryFromIntError) -> Self {
        StaartError::IntError(err)
    }
}
