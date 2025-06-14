// You live in a village. The village can be represented as a line that contains n grids.
// Each grid can be denoted as a house that is marked as H or a blank space that is marked as .
// A person lives in each house. A person can move to a grid if it is adjacent to that person.
// Therefore, the grid must be present on the left and right side of that person.
// Now, you are required to put some fences that can be marked as B on some blank spaces so that the village can be divided into several pieces.
// A person cannot walk past a fence but can walk through a house.
// You are required to divide the house based on the following rules:

// A person cannot reach a house that does not belong to that specific person.
// The number of grids each person can reach is the same and it includes the grid in which the house is situated.
// In order to show that you are enthusiastic and if there are many answers, then you are required to print the one where most fences are placed.
// Your task is to decide whether there is a possible solution. Print the possible solution.

// Input format

// First line: An integer n that represents the number of grids ( 1 <= n <= 20 )
// Second line: n characters that indicate the villages that are represented as H or .

// Output format

// The output must be printed in the following format:

// First line: If possible, then print YES. Otherwise, print NO.
// Second line: If the answer is YES, then print the way to do so.
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let n: usize = lines.next().unwrap().trim().parse().unwrap();
    let mut village: Vec<char> = lines.next().unwrap().trim().chars().collect();

    // Check for impossible case (adjacent houses)
    // If there is any "HH" in the village, it's impossible to divide
    let mut impossible = false;
    for i in 0..n - 1 {
        if village[i] == 'H' && village[i + 1] == 'H' {
            impossible = true;
            break;
        }
    }
    if impossible {
        println!("NO");
    } else {
        // Greedily place fences on all blanks for maximum fences
        for i in 0..n {
            if village[i] == '.' {
                village[i] = 'B';
            }
        }
        println!("YES");
        let result: String = village.iter().collect();
        println!("{}", result);
    }
}
