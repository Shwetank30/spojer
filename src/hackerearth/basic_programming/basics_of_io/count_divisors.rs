use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let lines = input.lines().next().unwrap().trim().split_whitespace();
    let mut nums = lines.map(|x| x.parse::<usize>().unwrap());
    let (l, r, k) = (
        nums.next().unwrap(),
        nums.next().unwrap(),
        nums.next().unwrap(),
    );
    let mut count = 0;
    for i in l..=r {
        if i % k == 0 {
            count += 1;
        }
    }
    println!("{}", count);
}
