// Given two arrays, a[1], a[2], ... a[n] and b[1], b[2],... b[n], in each step we can set a[i] = a[i] - b[i] if a[i] >= b[i].
// We need to determine the minimum number of steps that are required to make all elements of array a equal.
// If it's not possible, return -1.

use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let n: usize = lines.next().unwrap().trim().parse().unwrap();
    let a: Vec<u32> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let b: Vec<u32> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // Brute-force approach
    let min_a = *a.iter().min().unwrap();
    let mut answer = None;

    // Try all possible x from 0 to min_a
    for x in (0..=min_a).rev() {
        let mut total_steps = 0;
        let mut possible = true;
        for i in 0..n {
            if b[i] == 0 {
                if a[i] != x {
                    possible = false;
                    break;
                }
            } else {
                if a[i] < x || (a[i] - x) % b[i] != 0 {
                    possible = false;
                    break;
                }
                total_steps += (a[i] - x) / b[i];
            }
        }
        if possible {
            answer = Some(total_steps);
            break;
        }
    }
    if let Some(ans) = answer {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}
