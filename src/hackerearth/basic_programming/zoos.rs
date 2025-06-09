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
    let (x, y) = (
        s.chars()
            .filter(|&c| c == s.chars().next().unwrap())
            .count(),
        s.len()
            - s.chars()
                .filter(|&c| c == s.chars().next().unwrap())
                .count(),
    );
    if x * 2 == y {
        println!("Yes");
    } else {
        println!("No");
    }
}
