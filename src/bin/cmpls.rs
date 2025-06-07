use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    for _ in 0..t {
        let line1 = lines.next().unwrap().unwrap();
        let mut sc = line1.split_whitespace();
        let s: usize = sc.next().unwrap().parse().unwrap();
        let c: usize = sc.next().unwrap().parse().unwrap();

        let seq: Vec<i64> = lines
            .next()
            .unwrap()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        // Build the difference table
        let mut table = Vec::new();
        table.push(seq.clone());
        while table.last().unwrap().len() > 1 {
            let prev = table.last().unwrap();
            let mut next = Vec::with_capacity(prev.len() - 1);
            for i in 1..prev.len() {
                next.push(prev[i] - prev[i - 1]);
            }
            // If all values are equal, we can stop
            if next.iter().all(|&x| x == next[0]) {
                table.push(next);
                break;
            }
            table.push(next);
        }

        // Extend the table by C columns
        let depth = table.len();

        let last_row = table.last_mut().unwrap();
        let val = *last_row.last().unwrap();
        for _ in 0..c {
            last_row.push(val);
        }

        for row in (0..depth - 1).rev() {
            for i in 0..c {
                let next_val =
                    table[row].last().unwrap() + table[row + 1][table[row + 1].len() - c + i];
                table[row].push(next_val);
            }
        }
        let result = &table[0][s..s + c];
        println!(
            "{}",
            result
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        );
    }
}
