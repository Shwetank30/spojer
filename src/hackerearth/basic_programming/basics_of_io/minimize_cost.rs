use std::{
    collections::VecDeque,
    io::{self, Read},
};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let lines = input.lines().next().unwrap().trim().split_whitespace();
    let mut nums = lines.map(|x| x.parse().unwrap());
    let (n, k) = (nums.next().unwrap(), nums.next().unwrap());
    let arr: Vec<i64> = input
        .lines()
        .nth(1)
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut arr = arr;
    let mut neg_queue: VecDeque<usize> = VecDeque::new();
    let mut pos_queue: VecDeque<usize> = VecDeque::new();

    // Collect all negative indices
    for i in 0..n {
        if arr[i] < 0 {
            neg_queue.push_back(i);
        } else if arr[i] > 0 {
            pos_queue.push_back(i);
        }
    }

    while let (Some(&i), Some(&j)) = (pos_queue.front(), neg_queue.front()) {
        if (i as isize - j as isize).abs() as usize > k {
            // If too far apart, pop the one that's behind
            if i < j {
                pos_queue.pop_front();
            } else {
                neg_queue.pop_front();
            }
        } else {
            // Transfer as much as possible
            let transfer = arr[i].min(-arr[j]);
            arr[i] -= transfer;
            arr[j] += transfer;
            if arr[i] == 0 {
                pos_queue.pop_front();
            }
            if arr[j] == 0 {
                neg_queue.pop_front();
            }
        }
    }
    // Sum the absolute values after all possible transfers
    let result: i64 = arr.iter().map(|&x| x.abs()).sum();
    println!("{}", result);
}
