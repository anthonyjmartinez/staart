// staart is a Rust implementation of a tail-like program for Linux
// Copyright 2020-2021 Anthony Martinez
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//!
//! `staart` is a Rust implementation of a tail-like program for Linux systems
//!
//! The library exposes public methods to allow other programs to follow a file
//! internally. These methods are exposed on a struct [`TailedFile`].
//!
//! # Example
//!
//! ```rust
//! use staart::TailedFile;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let args: Vec<String> = std::env::args().collect();
//!     let path = &args[1].as_str();
//!     let mut f = TailedFile::new(path)?;
//!     loop {
//!        f.follow()?;
//!        f.sleep();
//!     }
//! }
//! ```

use std::{thread,time};
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::fs::{File, Metadata};
use std::os::linux::fs::MetadataExt;

enum FileStatus {
    Unchanged,
    Updated,
    Rotated
}

/// [`TailedFile`] tracks the state of a file being followed. It offers
/// methods for updating this state, and printing data to `stdout`. The
/// user may define the duration between updates if operating in a loop
/// with delay.

pub struct TailedFile<'a> {
    path: &'a str,
    fd: File,
    delay: u64,
    meta: Metadata,
    now: time::Instant,
    pos: u64
}

impl<'a> TailedFile<'a> {

    /// Creates an instance of `std::io::Result<staart::TailedFile>`
    /// 
    /// # Example
    /// `let mut f = staart::TailedFile::new("/var/log/syslog");`
    /// 
    /// # Defaults
    /// - `delay`: 100ms
    ///  
    /// # Propagates Errors
    /// - If the path provided does not exist, or is not readable by the current user
    /// - If file metadata can not be read

    pub fn new(path: &str) -> std::io::Result<TailedFile> {
        let fd = File::open(path)?;
        let delay = 100;
        let meta = fd.metadata()?;
        let now = time::Instant::now();
        let pos = meta.len();

        Ok(TailedFile {
            path,
            fd,
            delay,
            meta,
            now,
            pos
        })
    }

    fn open(&self, path: &str) -> std::io::Result<File> {
        Ok(File::open(path)?)
    }

    fn metadata(&self, fd: &File) -> std::io::Result<Metadata> {
        Ok(fd.metadata()?)
    }

    fn check_updates(&mut self) -> std::io::Result<FileStatus> {
        const THRESHOLD: time::Duration = time::Duration::from_secs(5);
        let current = time::Instant::now();
        let new_meta = self.metadata(&self.fd)?;
        if new_meta.len() != self.meta.len() && new_meta.st_ino() == self.meta.st_ino() {
            self.meta = new_meta;
            self.now = current;
            return Ok(FileStatus::Updated);
        } else if new_meta.len() == self.meta.len() && current.duration_since(self.now) > THRESHOLD {
            let new_fd = self.open(self.path)?;
            let new_file_meta = self.metadata(&new_fd)?;
            if new_file_meta.st_ino() != self.meta.st_ino() {
                self.fd = new_fd;
                self.meta = new_file_meta;
                self.now = current;
                return Ok(FileStatus::Rotated);
            } else {
                return Ok(FileStatus::Unchanged);
            }
        } else {
            return Ok(FileStatus::Unchanged);
        }
    }

    /// Updates the status of an instance of `staart::TailedFile`
    /// 
    /// File metadata are refreshed, and the position is updated if changes have occured
    /// since the last read operation.

    pub fn update_status(&mut self) -> std::io::Result<()> {
        let status = self.check_updates()?;

        match status {
            FileStatus::Unchanged => {},
            FileStatus::Updated => { self.pos = self.meta.len() },
            FileStatus::Rotated => { self.pos = 0 },
        }

        Ok(())
    }

    /// Reads new data for an instance of `staart::TailedFile` and returns
    /// `std::io::Result<Vec<u8>>`

    pub fn read(&mut self) -> std::io::Result<Vec<u8>> {
        let mut reader = BufReader::new(&self.fd);
        let mut data: Vec<u8> = Vec::new();

        reader.seek(SeekFrom::Start(self.pos))?;
        reader.read_to_end(&mut data)?;

        Ok(data)
    }

    /// Prints new data read on an instance of `staart::TailedFile` to `stdout`

    pub fn follow(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let data = self.read()?;
        if data.len() > 0 {
            self.update_status()?;
            let lines = String::from_utf8(data)?;
            print!("{}", lines);
	}

	Ok(())
    }

    /// Sets a delay duration, in milliseconds, for an instance of `staart::TailedFile`.
    /// This value is used when calling `staart::TailedFile::sleep()`.
    
    pub fn set_delay(&mut self, d: u64) {
        self.delay = d;
    }

    /// Sleeps for `staart::TailedFile.delay` milliseconds

    pub fn sleep(&mut self) {
        thread::sleep(time::Duration::from_millis(self.delay));
    }
}
