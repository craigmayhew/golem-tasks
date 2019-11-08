use std::env;
use std::fs;
use std::io::{self, Write};

extern crate prime_tools;

fn main() -> io::Result<()> {
    let mut out_file = fs::File::create("out.txt")?;
    let mut output:String = String::from(" ");

    let args = env::args().collect::<Vec<String>>();
    let possible_prime = args.get(1).map_or("no value found".to_owned(), |x| x.clone());
    
    output.push_str(&possible_prime);
    if prime_tools::is_u64_prime(possible_prime.parse::<u64>().unwrap()) {
        output.push_str(" is prime!");
    } else {
        output.push_str(" is not prime :(");
    }

    //wipe contents and write out entire file
    out_file.write_all(output.as_bytes())?;

    Ok(())
}