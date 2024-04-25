// staart is a Rust implementation of a tail-like program for Linux
// Copyright 2020-2024 Anthony Martinez
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
//!        f.read_and(|d| print!("{}", std::str::from_utf8(d).unwrap()))?;
//!        sleep(delay);
//!     }
//! }
//! ```

use std::fs::{File, Metadata};
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::path::Path;

mod errors;

pub use errors::StaartError;

type Result<T> = std::result::Result<T, StaartError>;

/// [`TailedFile`] tracks the state of a file being followed. It offers
/// methods for updating this state, and printing data to `stdout`.
pub struct TailedFile<T> {
    path: T,
    pos: u64,
    meta: Metadata,
}

impl<T> TailedFile<T>
where
    T: AsRef<Path> + Copy,
{
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
        let f = File::open(path)?;
        let meta = f.metadata()?;
        let pos = meta.len();

        Ok(TailedFile { path, pos, meta })
    }

    /// Reads new data for an instance of `staart::TailedFile` and returns
    /// `Result<Vec<u8>>`
    ///
    /// Prior to reading the file, it is checked for rotation and/or truncation.
    pub fn read(&mut self) -> Result<Vec<u8>> {
	let fd = File::open(self.path)?;
	self.check_rotate(&fd)?;
	self.check_truncate(&fd)?;
        let mut reader = BufReader::with_capacity(65536, &fd);
        let mut data: [u8; 65536] = [0u8; 65536];
        reader.seek(SeekFrom::Start(self.pos))?;
        let n: u64 = reader.read(&mut data)?.try_into()?;

        self.pos += n;

        let data: Vec<u8> = data.into_iter().take(n.try_into()?).collect();

        Ok(data)
    }

    /// Passes `&Vec<u8>` read from the tailed file to a user-defined function returning the unit type ()`.
    pub fn read_and<F: Fn(&[u8])>(&mut self, f: F) -> Result<()> {
	let data = self.read()?;

	f(&data);
	
	Ok(())
    }

    /// Checks for file rotation by inode comparision in Linux-like systems
    #[cfg(target_os = "linux")]
    fn check_rotate(&mut self, fd: &File) -> Result<()> {
        use std::os::linux::fs::MetadataExt;
        let meta = fd.metadata()?;
        let inode = meta.st_ino();
        if inode != self.meta.st_ino() {
            self.pos = 0;
            self.meta = meta;
        }

        Ok(())
    }

    /// Checks for file rotation by creation time comparision in Windows systems
    #[cfg(target_os = "windows")]
    fn check_rotate(&mut self, fd: &File) -> Result<()> {
        use std::os::windows::fs::MetadataExt;

        let meta = fd.metadata()?;
        let created_at = meta.creation_time();
        if created_at != self.meta.creation_time() {
            self.pos = 0;
            self.meta = meta;
        }

        Ok(())
    }


    /// Checks for file rotation by inode comparision in MacOS systems
    #[cfg(target_os = "macos")]
    fn check_rotate(&mut self, fd: &File) -> Result<()> {
        use std::os::unix::fs::MetadataExt;
        let meta = fd.metadata()?;
        let inode = meta.ino();
        if inode != self.meta.ino() {
            self.pos = 0;
            self.meta = meta;
        }

        Ok(())
    }

    /// Checks for file truncation by length comparision to the previous read position
    #[cfg(target_os = "linux")]
    fn check_truncate(&mut self, fd: &File) -> Result<()> {
        use std::os::linux::fs::MetadataExt;
        let meta = fd.metadata()?;
        let inode = meta.st_ino();
        let len = meta.len();
        if inode == self.meta.st_ino() && len < self.pos {
            self.pos = 0;
        }

        Ok(())
    }

    /// Checks for file truncation by length comparision to the previous read position
    #[cfg(target_os = "windows")]
    fn check_truncate(&mut self, fd: &File) -> Result<()> {
        use std::os::windows::fs::MetadataExt;
        let meta = fd.metadata()?;
        let created_at = meta.creation_time();
        let len = meta.len();
        if created_at == self.meta.creation_time() && len < self.pos {
            self.pos = 0;
        }

        Ok(())
    }

    /// Checks for file truncation by length comparision to the previous read position
    #[cfg(target_os = "macos")]
    fn check_truncate(&mut self, fd: &File) -> Result<()> {
        use std::os::unix::fs::MetadataExt;
        let meta = fd.metadata()?;
        let inode = meta.ino();
        let len = meta.len();
        if inode == self.meta.ino() && len < self.pos {
            self.pos = 0;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use super::*;

    #[test]
    fn tailed_file() {
        let dir = tempfile::tempdir().unwrap();
        let path = &dir.path().join("test.file");
        let _f = File::create(path).unwrap();
        let tailed_file = TailedFile::new(&path);
        assert!(tailed_file.is_ok())
    }

    #[test]
    fn test_read() {
        let dir = tempfile::tempdir().unwrap();
        let path = &dir.path().join("test.file");
        let test_data = b"Some data";

        let mut f = File::create(path).unwrap();
        let mut tailed_file = TailedFile::new(&path).unwrap();

        f.write_all(test_data).unwrap();

        let data = tailed_file.read().unwrap();
        assert_eq!(data.len(), test_data.len());
        assert_eq!(tailed_file.pos, 9);
    }

    #[test]
    fn test_check_rotate() {
        let dir = tempfile::tempdir().unwrap();
        let path = &dir.path().join("test.file");
        let path2 = &dir.path().join("test2.file");
        let test_data = b"Some data";
        let more_test_data = b"fun";

        let mut f = File::create(path).unwrap();
        f.write_all(test_data).unwrap();

        let mut tailed_file = TailedFile::new(&path).unwrap();

        std::fs::rename(path, path2).unwrap();

        let mut f = File::create(path).unwrap();
        f.write_all(more_test_data).unwrap();

        tailed_file.check_rotate(&f).unwrap();

        #[cfg(target_os = "linux")]
        {
            use std::os::linux::fs::MetadataExt;
            assert_eq!(tailed_file.meta.st_ino(), f.metadata().unwrap().st_ino())
        }

        #[cfg(target_os = "windows")]
        {
            use std::os::windows::fs::MetadataExt;
            assert_eq!(
                tailed_file.meta.creation_time(),
                f.metadata().unwrap().creation_time()
            )
        }
    }

    #[test]
    fn test_check_truncate() {
        let dir = tempfile::tempdir().unwrap();
        let path = &dir.path().join("test.file");
        let test_data = b"Some data";
        let more_test_data = b"fun";

        let mut f = File::create(path).unwrap();
        f.write_all(test_data).unwrap();

        let mut tailed_file = TailedFile::new(&path).unwrap();

        let mut f = File::create(path).unwrap();
        f.write_all(more_test_data).unwrap();

        tailed_file.check_truncate(&f).unwrap();
        assert_eq!(tailed_file.pos, 0)
    }
}
