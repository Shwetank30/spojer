// Find the Answer to life, the universe, and everything

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        if line == "42" {
            break;
        }
        println!("{}", line);
    }
}
