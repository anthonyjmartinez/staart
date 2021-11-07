// staart is a Rust implementation of a tail-like program for Linux
// Copyright 2020-2021 Anthony Martinez
// // Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//!
//! `staart` is a Rust implementation of a tail-like program
//!
//! The library exposes public methods to allow other programs to follow a file
//! internally. These methods are exposed on a struct [`TailedFile`].
//!
//! # Example
//!
//! ```no_run
//! use std::thread::sleep;
//! use std::time::Duration;
//! use staart::{StaartError, TailedFile};
//!
//! fn main() -> Result<(), StaartError> {
//!     let delay = Duration::from_millis(100);
//!     let args: Vec<String> = std::env::args().collect();
//!     let path = &args[1].as_str();
//!     let mut f = TailedFile::new(path)?;
//!     loop {
//!        f.follow()?;
//!        sleep(delay);
//!     }
//! }
//! ```

use std::fs::{File, Metadata};
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::os::linux::fs::MetadataExt;
use std::path::Path;

mod errors;

pub use errors::StaartError;

type Result<T> = std::result::Result<T, StaartError>;

/// [`TailedFile`] tracks the state of a file being followed. It offers
/// methods for updating this state, and printing data to `stdout`.
pub struct TailedFile<T> {
    path: T,
    pos: u64,
    inode: u64,
}

impl<T: AsRef<Path> + Copy> TailedFile<T> {
    /// Creates an instance of `std::io::Result<staart::TailedFile>`
    ///
    /// # Example
    /// ```no_run
    /// let mut f = staart::TailedFile::new("/var/log/syslog");
    /// ```
    ///
    /// # Propagates Errors
    /// - If the path provided does not exist, or is not readable by the current user
    /// - If file metadata can not be read
    pub fn new(path: T) -> Result<TailedFile<T>> {
        let inode: u64;
        let pos: u64;

        {
            let f = File::open(path)?;
            let meta = f.metadata()?;
            pos = meta.len();
            inode = meta.st_ino();
        }

        Ok(TailedFile { path, pos, inode })
    }

    /// Reads new data for an instance of `staart::TailedFile` and returns
    /// `Result<[u8; 65536]>`
    pub fn read(&mut self, file: &File) -> Result<[u8; 65536]> {
        let mut reader = BufReader::with_capacity(65536, file);
        let mut data: [u8; 65536] = [0u8; 65536];

        reader.seek(SeekFrom::Start(self.pos))?;
        let n: u64 = reader.read(&mut data)?.try_into()?;

        self.pos += n;
        Ok(data)
    }

    /// Prints new data read on an instance of `staart::TailedFile` to `stdout`
    pub fn follow(&mut self) -> Result<()> {
        let fd = File::open(self.path)?;
        let meta = fd.metadata()?;
        self.check_rotate(&meta);
        self.check_truncate(&meta);
        let data = self.read(&fd)?;

        let printable = std::str::from_utf8(&data)?;
	print!("{}", printable);

        Ok(())
    }

    /// Checks for file rotation by inode comparision in Linux-like systems
    fn check_rotate(&mut self, meta: &Metadata) {
        let inode = meta.st_ino();
        if inode != self.inode {
            self.pos = 0;
            self.inode = inode;
        }

    }

    /// Checks for file truncation by length comparision to the previous read position
    fn check_truncate(&mut self, meta: &Metadata) {
        let inode = meta.st_ino();
        let len = meta.len();
        if inode == self.inode && len < self.pos {
            self.pos = 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{io::Write, os::linux::fs::MetadataExt};

    use super::*;

    #[test]
    fn tailed_file() {
        let dir = tempfile::tempdir().unwrap();
        let path = &dir.path().join("test.file");
        let _f = File::create(&path).unwrap();
        let tailed_file = TailedFile::new(&path);
        assert!(tailed_file.is_ok())
    }

    #[test]
    fn test_read() {
        let dir = tempfile::tempdir().unwrap();
        let path = &dir.path().join("test.file");
        let test_data = b"Some data";

        let mut f = File::create(&path).unwrap();
        let mut tailed_file = TailedFile::new(&path).unwrap();

        f.write_all(test_data).unwrap();

        let f = File::open(&path).unwrap();

        let data = tailed_file.read(&f).unwrap();
        assert_eq!(&data[..9], test_data);
        assert_eq!(tailed_file.pos, 9);
    }

    #[test]
    fn test_check_rotate() {
        let dir = tempfile::tempdir().unwrap();
        let path = &dir.path().join("test.file");
        let path2 = &dir.path().join("test2.file");
        let test_data = b"Some data";
        let more_test_data = b"fun";

        let mut f = File::create(&path).unwrap();
        f.write_all(test_data).unwrap();

        let mut tailed_file = TailedFile::new(&path).unwrap();

        std::fs::rename(&path, &path2).unwrap();

        let mut f = File::create(&path).unwrap();
        f.write_all(more_test_data).unwrap();

        tailed_file.check_rotate(&f.metadata().unwrap());
        assert_eq!(tailed_file.inode, f.metadata().unwrap().st_ino())
    }

    #[test]
    fn test_check_truncate() {
        let dir = tempfile::tempdir().unwrap();
        let path = &dir.path().join("test.file");
        let test_data = b"Some data";
        let more_test_data = b"fun";

        let mut f = File::create(&path).unwrap();
        f.write_all(test_data).unwrap();

        let mut tailed_file = TailedFile::new(&path).unwrap();

        let mut f = File::create(&path).unwrap();
        f.write_all(more_test_data).unwrap();

        tailed_file.check_truncate(&f.metadata().unwrap());
        assert_eq!(tailed_file.pos, 0)
    }
}
