use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let lines = input.lines().next().unwrap().trim().split_whitespace();
    let mut nums = lines.map(|x| x.parse::<u64>().unwrap());
    let (n, k) = (nums.next().unwrap(), nums.next().unwrap());
    let arr_vec = input
        .lines()
        .nth(1)
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let mut arr = arr_vec.clone();
    let mut _cost = 0;

    for i in 0..n as usize {
        if arr[i] > 0 {
            // Try to transfer to the nearest negatives within distance K to the right
            for j in (i + 1)..=usize::min(i + k as usize, n as usize - 1) {
                if arr[i] == 0 {
                    break;
                }
                if arr[j] < 0 {
                    let transfer = arr[i].min(-arr[j]);
                    arr[i] -= transfer;
                    arr[j] += transfer;
                    _cost += transfer * (j as i64 - i as i64).abs();
                }
            }
            // Try to transfer to the nearest negatives within distance K to the left
            for j in (i.saturating_sub(k as usize)..i).rev() {
                if arr[i] == 0 {
                    break;
                }
                if arr[j] < 0 {
                    let transfer = arr[i].min(-arr[j]);
                    arr[i] -= transfer;
                    arr[j] += transfer;
                    _cost += transfer * (j as i64 - i as i64).abs();
                }
            }
        } else {
            // For each negative value, try to "fill" it with positive values from within K distance
            // Try to fill from the right
            for j in (i + 1)..=usize::min(i + k as usize, n as usize - 1) {
                if arr[i] == 0 {
                    break;
                }
                if arr[j] > 0 {
                    let transfer = arr[j].min(-arr[i]);
                    arr[j] -= transfer;
                    arr[i] += transfer;
                    _cost += transfer * (j as i64 - i as i64).abs();
                }
            }
            // Try to fill from the left
            for j in (i.saturating_sub(k as usize)..i).rev() {
                if arr[i] == 0 {
                    break;
                }
                if arr[j] > 0 {
                    let transfer = arr[j].min(-arr[i]);
                    arr[j] -= transfer;
                    arr[i] += transfer;
                    _cost += transfer * (j as i64 - i as i64).abs();
                }
            }
        }
    }
    // Sum the absolute values after all possible transfers
    let remaining_sum: i64 = arr.iter().map(|&x| x.abs()).sum();
    println!("{}", remaining_sum);
}
