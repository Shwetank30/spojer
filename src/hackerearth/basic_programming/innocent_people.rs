use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let tag = input.trim();

    let letter = tag.chars().nth(2).unwrap();
    if is_vowel(letter) {
        println!("invalid");
        return;
    }

    let bytes = tag.as_bytes();
    let pairs = [(0, 1), (3, 4), (4, 5), (7, 8)];
    for &(i, j) in &pairs {
        if !bytes[i].is_ascii_digit() || !bytes[j].is_ascii_digit() {
            println!("invalid");
            return;
        }
        let d1 = (bytes[i] - b'0') as u32;
        let d2 = (bytes[j] - b'0') as u32;
        if (d1 + d2) % 2 != 0 {
            println!("invalid");
            return;
        }
    }
    println!("valid");
}

fn is_vowel(c: char) -> bool {
    matches!(c, 'A' | 'E' | 'I' | 'O' | 'U' | 'Y')
}
