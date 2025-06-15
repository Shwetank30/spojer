/**
 * Problem
You are given an array A of size N. You can perform the following operation on array A:
    • Select two indices i and j such that 1 ≤  i, j ≤  N. (note that i and j can be equal)
    • Assign Ai = Ai + 2
    • Then assign Aj = Aj - 1
You need to make all the elements of the array equal to zero.
Task
Determine the minimum number of operations required to make all the elements of A equal to zero. Else, print -1 if it is not possible to do so.
Function description
Complete the function solve() provided in the editor. This function takes the following parameters and returns the required answer:
    • N: Represents the size of the array A
    • A: Represents the elements of array A.
Input format
Note: This is the input format that you must use to provide custom input (available above the Compile and Test button).
    • The first line contains T, denoting the number of test cases. T also specifies the number of times you have to run the solve() function on a different set of inputs.
    • For each test case:
        ○ The first line contains N, denoting the size of array A.
        ○ The second line contains space-separated values, denoting the elements of array A.
Output format For each test case, print the output on a new line. Either the minimum number of operations required to make all the elements of A equal to zero or print -1 if it is impossible to do so.
Constraints
1≤T≤1000
1≤N≤10^5
−10^9≤Ai≤10^9 ∀ 1≤i≤N
∑Ni≤10^5 ∀ 1≤i≤T .
*/
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    for _ in 0..t {
        let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
        let a: Vec<i64> = lines
            .next()
            .unwrap()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect();
        println!("{}", solve(n, &a));
    }
}

fn solve(n: usize, a: &[i64]) -> i64 {
    let result = -a.iter().sum::<i64>();
    let mut cnt = 0i64;
    for &x in a {
        if x < 0 {
            cnt += (-x) / 2;
        } else {
            cnt -= x;
        }
    }
    if cnt >= 0 {
        result
    } else {
        -1
    }
}
