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
    if s == s.chars().rev().collect::<String>() {
        println!("YES");
    } else {
        println!("NO");
    }
}
