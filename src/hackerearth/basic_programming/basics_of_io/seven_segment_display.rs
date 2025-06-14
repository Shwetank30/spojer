// Alice got a number written in seven segment format where each segment was creatted used a matchstick.
// Example: If Alice gets a number 123 so basically Alice used 12 matchsticks for this number.
// Alice is wondering what is the numerically largest value that she can generate by using at most the matchsticks that she currently possess.Help Alice out by telling her that number
use std::{
    io::{self, Read},
    iter,
};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let t: usize = lines.next().unwrap().trim().parse().unwrap();
    for _ in 0..t {
        let n = lines.next().unwrap().trim();
        let total_sticks: usize = n.chars().map(matchsticks_for_digit).sum();
        let mut result = String::new();
        if total_sticks % 2 == 0 {
            result.extend(iter::repeat('1').take(total_sticks / 2));
        } else {
            result.push('7');
            result.extend(iter::repeat('1').take((total_sticks - 3) / 2))
        }
        println!("{}", result);
    }
}

fn matchsticks_for_digit(d: char) -> usize {
    match d {
        '0' => 6,
        '1' => 2,
        '2' => 5,
        '3' => 5,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 3,
        '8' => 7,
        '9' => 6,
        _ => 0,
    }
}
