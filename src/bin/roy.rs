use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let length: u32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let w: u32 = parts.next().unwrap().parse().unwrap();
        let h: u32 = parts.next().unwrap().parse().unwrap();
        if w < length || h < length {
            println!("UPLOAD ANOTHER");
        } else {
            if w == h {
                println!("ACCEPTED");
            } else {
                println!("CROP IT");
            }
        }
    }
}
