/**
 * Given an array A of length N. Find the number of ordered pairs (i,j) which satisfy the given condition
    • (A[i]−A[j])/(i−j)=1
    • A[i]−A[j] should be divisible by i−j
    • i≠j
Input format
    • The first line contains an integer N.
    • The second line of each test case contains N space-separated integers, denoting the elements of array A.
Output format
Print the number of ordered pairs (i, j) which satisfy the above condition in a new line.
Constraints
1≤N≤5×10^5
1≤A[i]≤10^6
*/
use std::{
    collections::HashMap,
    io::{self, BufRead},
};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let _n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let a: Vec<i64> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let mut frequency = HashMap::new();
    for (i, &val) in a.iter().enumerate() {
        *frequency.entry(val - i as i64).or_insert(0u64) += 1;
    }
    let mut ans = 0u64;
    for &count in frequency.values() {
        if count > 1 {
            ans += count * (count - 1);
        }
    }
    println!("{}", ans);
}
