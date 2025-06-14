/**
 * Rhezo got a string S, a character C and an integer P as a birthday gift from his best friend JK.
* He got curious about the gift and started analysing it. He found out that the maximum frequency of character C over all the P length substrings of S is equal to some integer Z.
* He doesn't know programming, so asks you to find the largest position, where, if we insert some character after that position,
* the maximum frequency of C over all the P length substrings becomes Z+1. If there is no such position, output 1
* If the answer is the position before the first character, output 0.
* Input:
* First line of the input contains a string S.
* Second line of the input contains the character C.
* Next and the last line contains a single integer P.
* Output:
* Print an answer in a separate line.
* Constraints:
*	• 1≤|S|≤10^3
*	• 1≤P≤|S|
*	• S consists of lowercase alphabetic characters
* C is any lowercase alphabetic character
*/
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let s = lines.next().unwrap().unwrap().trim().to_string();
    let c = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .chars()
        .next()
        .unwrap();
    let p: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    let n = s.len();
    let s_chars: Vec<char> = s.chars().collect();

    let mut max_freq = 0;
    let mut freq = 0;
    for i in 0..p.min(n) {
        if s_chars[i] == c {
            freq += 1;
        }
    }
    max_freq = freq;
    for i in p..n {
        if s_chars[i - p] == c {
            freq -= 1;
        }
        if s_chars[i] == c {
            freq += 1;
        }
        if freq > max_freq {
            max_freq = freq;
        }
    }
    let z = max_freq;

    let mut answer = -1;
    for insert_pos in 0..=n {
        let mut new_chars = Vec::with_capacity(n + 1);
        new_chars.extend_from_slice(&s_chars[..insert_pos]);
        new_chars.push(c);
        new_chars.extend_from_slice(&s_chars[insert_pos..]);

        let mut freq = 0;
        for i in 0..p.min(n + 1) {
            if new_chars[i] == c {
                freq += 1;
            }
        }
        if freq == z + 1 {
            answer = insert_pos as i32;
        }
        for i in p..(n + 1) {
            if new_chars[i - p] == c {
                freq -= 1;
            }
            if new_chars[i] == c {
                freq += 1;
            }
            if freq == z + 1 {
                answer = insert_pos as i32;
            }
        }
    }
    println!("{}", answer);
}
