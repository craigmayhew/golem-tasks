use std::env;
use std::io::{self};
use std::fs;

extern crate lzma_rs;

use lzma_rs::{lzma2_compress};
use std::io::prelude::*;

fn write_file_to_disk(filename: String , data: Vec<u8>) -> io::Result<()> {
	let mut out_file = fs::File::create(filename)?;
	out_file.write_all(&data)?;

	Ok(())
}

fn main() -> io::Result<()> {
	let args = env::args().collect::<Vec<String>>();
	let filename = args.get(1).map_or("no value found".to_owned(), |x| x.clone());
	
	let mut file = std::io::BufReader::new(std::fs::File::open(&filename).unwrap());

	let filename_7z = format!("{}.7z",&filename);
	let mut compressed: Vec<u8> = Vec::new();
	lzma2_compress(&mut file, &mut compressed).unwrap();

	//write compressed content to disk
	let mut out_file = fs::File::create(filename_7z)?;
	out_file.write_all(&compressed)?;

    Ok(())
}
