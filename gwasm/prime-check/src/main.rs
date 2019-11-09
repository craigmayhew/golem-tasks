use std::env;
use std::fs;
use std::io::{self, Write};

extern crate prime_tools;

fn is_prime_check(possible_prime:String) -> String {
    let mut output:String = String::from(&possible_prime);

    if prime_tools::is_u64_prime(possible_prime.parse::<u64>().unwrap()) {
        output.push_str(" is prime!");
    } else {
        output.push_str(" is not prime :(");
    }

    output
}

fn main() -> io::Result<()> {
    //setup out.txt file
    let mut out_file = fs::File::create("out.txt")?;
    let mut output:String = String::from(" ");

    //get first argument
    let args = env::args().collect::<Vec<String>>();
    let possible_prime = args.get(1).map_or("no value found".to_owned(), |x| x.clone());
    
    //prime check
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