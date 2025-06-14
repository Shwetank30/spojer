use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let length: u32 = lines.next().unwrap().trim().parse().unwrap();
    let n: usize = lines.next().unwrap().trim().parse().unwrap();
    for line in lines.take(n) {
        let mut nums = line.split_whitespace().map(|x| x.parse::<u32>().unwrap());
        let (w, h) = (nums.next().unwrap(), nums.next().unwrap());

        match (w, h) {
            (w, h) if w < length || h < length => println!("UPLOAD ANOTHER"),
            (w, h) if w == h => println!("ACCEPTED"),
            _ => println!("CROP IT"),
        }
    }
}
