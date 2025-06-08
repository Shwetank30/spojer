use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let n: usize = stdin
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse()
        .unwrap();
    let result = factorial(n);
    println!("{}", result);
}

fn factorial(n: usize) -> usize {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}
