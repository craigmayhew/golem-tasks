use std::env;
use std::io::{self};
use std::fs;

extern crate lzma_rs;

use lzma_rs::{lzma_compress};
use std::io::prelude::*;

fn write_file_to_disk(filename: String , data: Vec<u8>) -> io::Result<()> {
	let mut out_file = fs::File::create(filename)?;
	out_file.write_all(&data)?;

	Ok(())
}

fn get_first_arg() -> String {
	let args = env::args().collect::<Vec<String>>();
	let first_arg: String = args.get(1).map_or("no value found".to_owned(), |x| x.clone());
	first_arg
}

fn main() -> io::Result<()> {
	let filename: String = get_first_arg();
	
	let mut file = std::io::BufReader::new(std::fs::File::open(&filename).unwrap());

	let mut compressed: Vec<u8> = Vec::new();
	lzma_compress(&mut file, &mut compressed).unwrap();

	//write compressed content to disk
	let filename_7z:String = format!("{}.7z",&filename);
	write_file_to_disk(filename_7z, compressed).unwrap();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_first_arg_test() {
		//note test command is `cargo test test`
		assert_eq!(get_first_arg(), "test".to_owned());
    }
}