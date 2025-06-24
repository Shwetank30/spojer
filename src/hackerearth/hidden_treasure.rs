/**
 * Alice is given an array of integers nums. She wants to determine how many pairs (i,j) exist such that:
    • 0≤i<j<nums.length
    • The sum of digits of nums[i] is equal to the sum of digits of nums[j].
She believes that the correct count of these pairs will reveal a crucial clue needed to decode the next part of the scroll. Your task is to help Alice compute this number so she can continue her quest.
Task
Return the number of special pairs to assist Alice in uncovering the hidden secret.
Function description
Complete the function solve() provided in the editor. This function takes the following two parameters and returns the required answer:
    • N: Represents the number of elements in the array
    • nums: A list of N integers
Input format
    • The first line contains a single integer N(size of the array).
    • The second line contains N space-separated integers representing the array nums.
Output format
For each test case, print the required answer on a new line.
Constraints
    • 2≤N≤5∗10^5
    • 1≤nums[i]≤10^9
Sample Input
4
51 71 17 42
Sample Output
2
*/
use std::{
    collections::HashMap,
    io::{self, BufRead},
};

fn digit_sum(mut n: u32) -> u32 {
    let mut sum = 0;
    while n > 0 {
        sum += n % 10;
        n /= 10;
    }
    sum
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let _n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let nums: Vec<u32> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut freq = HashMap::new();
    for &num in &nums {
        let s = digit_sum(num);
        *freq.entry(s).or_insert(0u64) += 1;
    }

    let mut ans = 0u64;
    for &count in freq.values() {
        if count > 1 {
            ans += count * (count - 1) / 2;
        }
    }
    println!("{}", ans);
}
