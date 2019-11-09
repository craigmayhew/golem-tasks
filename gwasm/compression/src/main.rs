use std::io::{self};

extern crate lzma_rs;

use lzma_rs::{xz_compress,xz_decompress};
use std::io::prelude::*;

fn main() -> io::Result<()> {
	let filename = "foo.xz";
	let mut f = std::io::BufReader::new(std::fs::File::open(filename).unwrap());
	// "decomp" can be anything that implements "std::io::Write"
	let mut decomp: Vec<u8> = Vec::new();
	xz_decompress(&mut f, &mut decomp).unwrap();
	// Decompressed content is now in "decomp"

    Ok(())
}
