// You are conducting a contest at your college. This contest consists of two problems and n
//  participants. You know the problem that a candidate will solve during the contest.

// You provide a balloon to a participant after he or she solves a problem. There are only green and purple-colored balloons available in a market. Each problem must have a balloon associated with it as a prize for solving that specific problem. You can distribute balloons to each participant by performing the following operation:

// Use green-colored balloons for the first problem and purple-colored balloons for the second problem
// Use purple-colored balloons for the first problem and green-colored balloons for the second problem
// You are given the cost of each balloon and problems that each participant solve. Your task is to print the minimum price that you have to pay while purchasing balloons.

// Input format:
// First line: T (number of test cases, 1 <= T <= 10)
// For each test case:
//   First line: Cost of green and purple balloons
//   Second line: n (number of participants, 1 <= n <= 10)
//   Next n lines: Each line has two integers (0 or 1) indicating if the participant solved problem 1 and/or 2
// Output format
// For each test case, print the minimum cost that you have to pay to purchase balloons.
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let t: usize = lines.next().unwrap().trim().parse().unwrap();

    for _ in 0..t {
        let costs = lines.next().unwrap();
        let mut cost_iter = costs.split_whitespace().map(|x| x.parse::<u32>().unwrap());
        let green = cost_iter.next().unwrap();
        let purple = cost_iter.next().unwrap();
        let n: usize = lines.next().unwrap().trim().parse().unwrap();

        let mut p1_solves = 0;
        let mut p2_solves = 0;
        for _ in 0..n {
            let line = lines.next().unwrap();
            let mut nums = line.split_whitespace().map(|x| x.parse::<u32>().unwrap());
            let p1 = nums.next().unwrap();
            let p2 = nums.next().unwrap();
            p1_solves += p1;
            p2_solves += p2;
        }
        let cost1 = p1_solves * green + p2_solves * purple;
        let cost2 = p1_solves * purple + p2_solves * green;
        println!("{}", cost1.min(cost2));
    }
}
