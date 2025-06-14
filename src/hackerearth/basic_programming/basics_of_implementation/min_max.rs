/**
* Given an array of integers . Check if all the numbers between minimum and maximum number in array exist's within the array .
* Print 'YES' if numbers exist otherwise print 'NO'(without quotes).
* Input:
* Integer N denoting size of array
* Next line contains N space separated integers denoting elements in array
* Output:
* Output your answer
* Constraints:
* 1<= N <= 1000
* 1<= a[i] <= 100
*/
use std::{collections::HashSet, io::{self, BufRead}};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let arr: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse array element"))
        .collect();

    if arr.is_empty() {
        println!("YES");
        return;
    }

    let min = *arr.iter().min().unwrap();
    let max = *arr.iter().max().unwrap();

    let set: HashSet<_> = arr.iter().cloned().collect();
    let all_exist = (min..=max).all(|num| set.contains(&num));

    if all_exist {
        println!("YES");
    } else {
        println!("NO");
    }
}
