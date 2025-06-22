/**
 * Problem
You are given an array A of length N.  You take an array B of length N such that B[i] = 0 for each 1 <= i <= N. You can apply the following operation on B any number of times:
    • Choose (N - 1) distinct indices and add 1 to each of those indices.
Task
Find the number of operations required to convert array B into array A by applying the given operation. Print -1 if it is impossible to do so.
Function description
Complete the function solve() provided in the editor. This function takes the following two parameters and returns the required answer:
    • N: Represents the length of array A.
    • A: Represents the array A.
Input format
Note: This is the input format that you must use to provide custom input (available above the Compile and Test button).
    • The first line contains T, denoting the number of test cases. T also specifies the number of times you have to run the solve function on a different set of inputs.
    • For each test case:
        ○ The first line contains N, denoting the size of array A.
        ○ The second line contains N space-separated integers A[1], A[2], ....., A[N], denoting the elements of  array A.
Output format
For each test case, print the number of operations required to convert array B into array A by applying the given operation or -1 if it is impossible to do so.
Constraints
1≤T≤10^4
2≤N≤10^5
0≤Ai≤10^9
Sum of N over all test cases does not exceed 2*10^5
Sample Input
2
3
3 1 0
2
0 2

Sample Output
-1
2

*/
use std::io::{self, BufRead};

fn solve(n: usize, a: &[u64]) -> i64 {
    let sum: u64 = a.iter().sum();
    let max_a = *a.iter().max().unwrap();
    if n == 1 {
        return if a[0] == 0 { 0 } else { -1 };
    }
    if sum % (n as u64 - 1) != 0 {
        return -1;
    }
    let k = sum / (n as u64 - 1);
    if max_a > k {
        return -1;
    }
    k as i64
}
fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    for _ in 0..t {
        let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
        let a: Vec<u64> = lines
            .next()
            .unwrap()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        println!("{}", solve(n, &a));
    }
}
