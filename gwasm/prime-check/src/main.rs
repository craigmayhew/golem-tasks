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

    //get first argument
    let args = env::args().collect::<Vec<String>>();
    let possible_prime = args.get(1).map_or("no value found".to_owned(), |x| x.clone());
    
    let output:String = is_prime_check(possible_prime);

    //wipe contents and write out entire file
    out_file.write_all(output.as_bytes())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_prime_check_test() {
        assert_eq!(is_prime_check("598421674037041".to_string()), "598421674037041 is prime!");
        assert_eq!(is_prime_check("901".to_string()), "901 is not prime :(");
        assert_eq!(is_prime_check("16384".to_string()), "16384 is not prime :(");
    }
}