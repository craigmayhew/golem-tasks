use std::io::{self};

extern crate xz2;

use xz2::read::{XzEncoder, XzDecoder};
use std::io::prelude::*;

fn main() -> io::Result<()> {
    // Round trip some bytes from a byte source, into a compressor, into a
	// decompressor, and finally into a vector.
	let data = "Hello, World!".as_bytes();
	let compressor = XzEncoder::new(data, 9);
	let mut decompressor = XzDecoder::new(compressor);

	let mut contents = String::new();
	decompressor.read_to_string(&mut contents).unwrap();
	assert_eq!(contents, "Hello, World!");
    
    Ok(())
}
