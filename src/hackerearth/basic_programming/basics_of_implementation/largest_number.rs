/**
 * Problem
Given an integer N which has D digits. You have to delete exactly K digits in integer N.
Find out the largest possible number which can be built from N after removing exactly K digits.
Input Format:
    • First line contains two space separated integers N K.
Output Format:
Print the largest possible number which can be built from N after removing exactly K digits.
Constraints:
1≤N≤10^18
1≤K≤3
K≤D
Sample Input
3412 1
Sample Output
412
*/
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.lock().read_line(&mut line).unwrap();
    let mut parts = line.split_whitespace();
    let n_str = parts.next().unwrap();
    let k: usize = parts.next().unwrap().parse().unwrap();

    let digits: Vec<char> = n_str.chars().collect();
    let mut stack = Vec::new();
    let mut to_remove = k;
    for &d in &digits {
        while to_remove > 0 && !stack.is_empty() && stack.last().unwrap() < &d {
            stack.pop();
            to_remove -= 1;
        }
        stack.push(d);
    }
    while to_remove > 0 {
        stack.pop();
        to_remove -= 1;
    }

    let result: String = stack.iter().collect();
    println!("{}", result);
}
