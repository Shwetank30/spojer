use std::{
    collections::HashMap,
    io::{self, Read},
};

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
    // Within this array, we have to find the element that occurs most frequently.
    // If there are multiple elements with the same max frequency, then we need to return the total number of such elements with max frequency.
    let mut frequency_map = HashMap::new();
    for num in arr {
        *frequency_map.entry(num).or_insert(0) += 1;
    }
    let mut max_frequency = 0;
    let mut count_of_max_frequency = 0;
    for &frequency in frequency_map.values() {
        if frequency > max_frequency {
            max_frequency = frequency;
            count_of_max_frequency = 1;
        } else if frequency == max_frequency {
            count_of_max_frequency += 1;
        }
    }
    println!("{}", count_of_max_frequency);
}
