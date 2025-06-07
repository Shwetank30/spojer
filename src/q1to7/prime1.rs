// Generate Prime Numbers between two numbers m & n where 1 <= m <= n <= 1000000000 for t test cases

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    for _ in 0..t {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let m: u64 = parts.next().unwrap().parse().unwrap();
        let n: u64 = parts.next().unwrap().parse().unwrap();
        for i in m..=n {
            if is_prime(i) {
                println!("{}", i);
            }
        }
    }
}

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 || n == 3 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    let limit = (n as f64).sqrt() as u64;
    for i in (3..limit + 1).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    true
}
