/**
 * You are given an array A  of N elements. Now you need to choose the best index of this array A.
 * An index of the array is called best if the special sum of this index is maximum across the special sum of all the other indices.
 * To calculate the special sum for any index i, you pick the first element that is A[i] and add it to your sum.
 * Now you pick next two elements i.e. A[i+1] and A[i+2] and add them to your sum. Now you will pick the next 3 elements
 * and this continues till the index for which it is possible to pick the elements.
 * Find the best index and in the output print its corresponding special sum.
 * Note that there may be more than one best indices but you need to only print the maximum special sum.
 * Input: First line contains an integer N as input. Next line contains N space separated integers denoting the elements of the array A.
 * In the output you have to print an integer that denotes the maximum special sum
 * Constraints: 1 ≤ N ≤ 10^5, -10^7 ≤ A[i] ≤ 10^7
 */
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let arr: Vec<i64> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut prefix = vec![0i64; n + 1];
    for i in 0..n {
        prefix[i + 1] = prefix[i] + arr[i];
    }

    let mut max_special_sum = std::i64::MIN;

    for i in 0..n {
        let mut group_size = 1;
        let mut start = i;
        let mut special_sum = 0;

        while start + group_size <= n {
            let end = start + group_size;
            special_sum += prefix[end] - prefix[start];
            start = end;
            group_size += 1;
        }
        if special_sum > max_special_sum {
            max_special_sum = special_sum;
        }
    }
    println!("{}", max_special_sum);
}
