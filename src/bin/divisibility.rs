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
    let mut last_digit = 0u8;
    for num in arr {
        last_digit = (last_digit * 10 + (num % 10) as u8) % 10;
    }
    if last_digit == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
