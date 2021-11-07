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

use staart::{StaartError, TailedFile};

type Result<T> = std::result::Result<T, StaartError>;

fn main() -> Result<()> {
    const DEFAULT_DELAY: Duration = Duration::from_millis(100);
    const OPEN_ERR_LIMIT: u8 = 3;

    let args: Vec<String> = std::env::args().collect();
    let path = Path::new(&args[1]);
    let mut f = TailedFile::new(path)?;
    let mut open_errors: u8 = 0;

    loop {
	if let Err(e) = f.follow() {
	    match e {
		StaartError::IO(err) if err.kind() == std::io::ErrorKind::NotFound => {
		    if open_errors >= OPEN_ERR_LIMIT {
			eprintln!("Failed to open: {}, more than {} times. Exiting!", path.display(), open_errors);
			std::process::exit(1);
		    } else {
			open_errors += 1;
		    }
		},
		StaartError::Utf8(_) => {
		    eprintln!("encountered non-utf8 bytes on read")
		},
		_ => {
		    return Err(e)
		}
	    }
	}

        sleep(DEFAULT_DELAY);
    }
}
