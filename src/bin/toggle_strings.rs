use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let s = stdin
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .to_string();
    let arr = s.chars().map(|c: char| {
        if c.is_ascii_lowercase() {
            c.to_ascii_uppercase()
        } else {
            c.to_ascii_lowercase()
        }
    });
    let result: String = arr.collect();
    println!("{}", result);
}
