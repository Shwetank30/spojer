/**
 * Problem
Given an array A of N integers. Find the number of unordered pairs (i,j), 1≤i,j≤N such that
    • L≤A[i]+A[j]≤R
    • A[i]⊕A[j] is odd where ⊕ represents bitwise XOR operation.
Input format
    • The first line contains three space-separated integers N L R.
    • The next line contains N space-separated integers denoting the array A
Output format
Print the number of unordered pairs (i,j) which satisfy the above conditions in a new line.
Constraints
1≤N≤10^5
1≤L≤R≤10^18
1≤A[i]≤10^18
*/
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = lines.next().unwrap().unwrap();
    let mut first_iter = first_line.split_whitespace();
    let n: usize = first_iter.next().unwrap().parse().unwrap();
    let l: i64 = first_iter.next().unwrap().parse().unwrap();
    let r: i64 = first_iter.next().unwrap().parse().unwrap();

    let a: Vec<i64> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut even = Vec::new();
    let mut odd = Vec::new();
    for &num in &a {
        if num % 2 == 0 {
            even.push(num);
        } else {
            odd.push(num);
        }
    }
    even.sort_unstable();
    odd.sort_unstable();

    fn count_pairs(a: &Vec<i64>, b: &Vec<i64>, l: i64, r: i64) -> u64 {
        let mut count = 0u64;
        for &x in a {
            let left = b.partition_point(|&y| y < l - x);
            let right = b.partition_point(|&y| y <= r - x);
            count += (right - left) as u64;
        }
        count
    }
    let mut ans = count_pairs(&even, &odd, l, r);
    ans += count_pairs(&odd, &even, l, r);

    println!("{}", ans / 2);
}
