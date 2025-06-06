// For a given positive integer k, output the smallest palindrome greater than k.

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    for _ in 0..t {
        let line = lines.next().unwrap().unwrap();
        let trimmed = line.trim();
        if !trimmed.chars().all(|c| c.is_digit(10)) {
            eprintln!("Error processing input: {}", line);
            continue;
        }
        let result = next_palindrome_str(trimmed);
        println!("{}", result);
    }
}

fn next_palindrome_str(s: &str) -> String {
    let len = s.len();
    let chars: Vec<u8> = s.bytes().collect();

    // Check if all digits are '9'
    if chars.iter().all(|&c| c == b'9') {
        let mut res = String::with_capacity(len + 1);
        res.push('1');
        for _ in 0..len - 1 {
            res.push('0');
        }
        res.push('1');
        return res;
    }

    // Mirror left to right
    let mut left = chars.clone();
    for i in 0..len / 2 {
        left[len - 1 - i] = left[i];
    }

    // If mirrored version is greater than input, return it
    if left > chars {
        return String::from_utf8(left).unwrap();
    }

    // Otherwise, increment the middle and mirror again
    let mut inc = chars.clone();
    let mut mid = (len - 1) / 2;
    loop {
        if inc[mid] < b'9' {
            inc[mid] += 1;
            if len % 2 == 0 && mid != len - 1 - mid {
                inc[len - 1 - mid] += 1;
            }
            break;
        } else {
            inc[mid] = b'0';
            inc[len - 1 - mid] = b'0';
        }
        if mid == 0 {
            break;
        }
        mid -= 1;
    }

    // Mirror again
    for i in 0..len / 2 {
        inc[len - 1 - i] = inc[i];
    }
    String::from_utf8(inc).unwrap()
}
