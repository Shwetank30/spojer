/**
 * Problem
Given an array A of N integers.
Find the count of subarrays whose product of elements is a perfect cube.
Input Format:
    • First line contains an integer N.
    • Next line contains N space-separated integers denoting the array elements.
Output Format:
Print the number of subarrays whose product of elements is a perfect cube.
Constraints:
1≤N≤10^5
1≤A[i]≤10^2

Sample Input
4
2 4 6 36
Sample Output
3
*/
use std::{
    collections::HashMap,
    io::{self, BufRead},
};

fn main() {
    let mut prime_list = vec![];
    let mut is_prime = vec![true; 101];
    for i in 2..=100 {
        if is_prime[i] {
            prime_list.push(i);
            let mut j = i * 2;
            while j <= 100 {
                is_prime[j] = false;
                j += i;
            }
        }
    }
    let mut factor_table = vec![vec![0u8; prime_list.len()]; 101];
    for num in 2..=100 {
        let mut n = num;
        for (pi, &p) in prime_list.iter().enumerate() {
            while n % p == 0 {
                factor_table[num][pi] += 1;
                n /= p;
            }
        }
    }

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let _n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let a: Vec<usize> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let mut count = HashMap::new();
    let mut prefix = vec![0u8; prime_list.len()];
    count.insert(prefix.clone(), 1u64);

    let mut ans = 0u64;
    for &num in &a {
        for (i, &exp) in factor_table[num].iter().enumerate() {
            prefix[i] = (prefix[i] + exp) % 3;
        }
        ans += *count.get(&prefix).unwrap_or(&0);
        *count.entry(prefix.clone()).or_insert(0) += 1;
    }
    println!("{}", ans);
}
