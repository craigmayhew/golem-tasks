use std::io::{self, Write};

extern crate lzma;

use lzma::LzmaWriter;
use std::io::prelude::*;
use std::fs::File;

fn main() -> io::Result<()> {
    let f = File::create("out.xz").unwrap();
	let mut f = LzmaWriter::new_compressor(f, 6).unwrap();

	write!(f, "It's a small world!").unwrap();
	f.finish().unwrap();
    
    Ok(())
}
