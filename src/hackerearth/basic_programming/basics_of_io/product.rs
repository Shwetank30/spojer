use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let _n: usize = lines.next().unwrap().trim().parse().unwrap();
    let arr = lines
        .next()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap());
    let modulo = 1_000_000_007u64;
    let mut product = 1u64;
    for num in arr {
        product = (product * num) % modulo;
    }
    println!("{}", product);
}
