use std::io::{self, BufRead};
use std::env;
use std::char;

const DEFAULT_SHIFT: i32 = 5;
const UPPERCASE_START: i32 = 65;
const LOWERCASE_START: i32 = 97;
const ALPHABET_SIZE: i32 = 26;

fn modulo(a: i32, b: i32) -> i32 {
    (a%b).abs()
}

fn to_ascii(i: i32) -> char {
    char::from_u32(i as u32).unwrap()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut shift = DEFAULT_SHIFT;
    if args.len() > 0 {
        shift = args[1].parse::<i32>().unwrap();
    }

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.expect("Could not read line from standard in");
        for c in line.chars() {
            let ascii = c as i32;
            if UPPERCASE_START <= ascii && UPPERCASE_START+ALPHABET_SIZE >= ascii {
                print!("{}", to_ascii(modulo((ascii - UPPERCASE_START) + shift, ALPHABET_SIZE) + UPPERCASE_START));
            } else if LOWERCASE_START <= ascii && LOWERCASE_START+ALPHABET_SIZE >= ascii {
                print!("{}", to_ascii(modulo((ascii - LOWERCASE_START) + shift, ALPHABET_SIZE) + LOWERCASE_START));
            } else {
                print!("{}", c);
            }
        }
        print!("\n");
    }
}
