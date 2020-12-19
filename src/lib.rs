/*
    staart is a Rust implementation of a tail-like program for Linux
    Copyright (C) 2020  Anthony Martinez

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use std::{thread,time};
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::fs::{File, Metadata};
use std::os::linux::fs::MetadataExt;

enum FileStatus {
    Unchanged,
    Updated,
    Rotated
}

pub struct TailedFile<'a> {
    path: &'a str,
    fd: File,
    delay: u64,
    meta: Metadata,
    now: time::Instant,
    pos: u64
}

impl<'a> TailedFile<'a> {
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

    pub fn update_status(&mut self) -> std::io::Result<()> {
        let status = self.check_updates()?;

        match status {
            FileStatus::Unchanged => {},
            FileStatus::Updated => { self.pos = self.meta.len() },
            FileStatus::Rotated => { self.pos = 0 },
        }

        Ok(())
    }

    pub fn read(&mut self) -> std::io::Result<Vec<u8>> {
        let mut reader = BufReader::new(&self.fd);
        let mut data: Vec<u8> = Vec::new();

        reader.seek(SeekFrom::Start(self.pos))?;
        reader.read_to_end(&mut data)?;

        Ok(data)
    }

    pub fn follow(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let data = self.read()?;
        if data.len() > 0 {
            self.update_status()?;
            let lines = String::from_utf8(data)?; // Will blow up if not data is not utf8
            print!("{}", lines);
            Ok(())
        } else { Ok(()) } // Only here to clear E0317, and no-op on empty data
    }

    pub fn set_delay(&mut self, d: u64) {
        self.delay = d;
    }

    pub fn sleep(&mut self) {
        thread::sleep(time::Duration::from_millis(self.delay));
    }
}
