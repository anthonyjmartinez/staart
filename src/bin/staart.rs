// staart is a Rust implementation of a tail-like program for Linux
// Copyright 2020-2021 Anthony Martinez
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::path::Path;
use std::thread::sleep;
use std::time::Duration;

use staart::TailedFile;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    const DEFAULT_DELAY: Duration = Duration::from_millis(100);
    let args: Vec<String> = std::env::args().collect();
    let path = Path::new(&args[1]);
    let mut f = TailedFile::new(path)?;
    loop {
        f.follow()?;
        sleep(DEFAULT_DELAY);
    }
}
