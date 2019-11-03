use std::env;
use std::fs;
use std::io::{self, Write};

extern crate md5;
extern crate rand;

use rand::Rng;
use rand::distributions::Alphanumeric;

fn main() -> io::Result<()> {

    /*let mut in_file = fs::File::open("in.txt")?;
    let mut input = String::new();
    in_file.read_to_string(&mut input)?;*/

    let difficulty:usize = 5;

    let mut hash = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .collect::<String>();

    let args = env::args().collect::<Vec<String>>();
    let arg1 = args.get(1).map_or("anonymous".to_owned(), |x| x.clone());
    hash.push_str(&arg1);

    for _n in 0..50_000_000 {
        let input_hash:String = hash;
        let mut attempt = String::from("My name is ZeroPointCraig and this file has a hash of ");
        attempt.push_str(&input_hash);
        hash = format!("{:x}", md5::compute(&attempt));
        if hash[0..difficulty] == input_hash[0..difficulty] {
            //only write out.txt if we find an answer!
            let mut out_file = fs::File::create("out.txt")?;
            out_file.write_all(&attempt.as_bytes())?;
        }
    }

    let mut out_file = fs::File::create("out.txt")?;
    out_file.write_all("".as_bytes())?;
    
    Ok(())
}