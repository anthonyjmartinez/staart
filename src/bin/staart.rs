// staart is a Rust implementation of a tail-like program for Linux
// Copyright 2020-2021 Anthony Martinez
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use staart::TailedFile;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let path = &args[1].as_str();
    let mut f = TailedFile::new(path)?;
    loop {
        f.follow()?;
        f.sleep();
    }
}
