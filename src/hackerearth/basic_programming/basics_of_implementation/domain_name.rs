/** Task
Check if the given string forms a valid Fully Qualified Domain name.
Assumptions
Hostnames are composed of a series of labels concatenated with dots. Each label is 1 to 63 characters long, and may contain:
    • The ASCII letters a-z (in a case-insensitive manner),
    • Digits 0-9
    • Hyphen ('-').
Additionally
    • Labels cannot start or end with hyphens
    • Labels can start with numbers
    • Max length of ASCII hostname including dots is 253 characters (not counting trailing dots)
    • Underscores are not allowed in hostnames (but are allowed in other DNS types)
Some more assumptions are:
    • TLD is at least 2 characters and only a-z
    • We want at least 1 level above TLD
    • No trailing dots after TLD
Input format
Note: This is the input format you must use to provide custom input (available above the Compile and Test buttons).
    • The first line contains T denoting the number of test cases.
    • Followed by T Lines :
        ○ Each T line contains a String denoting the Full Domain name.
Output format
For each test case in a new line, print either true for Valid or false for Invalid
Constraints
1 ≤ T ≤ 100
1 ≤ Domain.length ≤ 200
Sample Input
10
3m.com
3m-.com
xy.com
hero.app.hackerearth.com
google.com
App.Hackerearth.com
app.app
russia.ru
spain.in
you.online
Sample Output
true
false
true
true
true
true
true
true
true
true
*/
use std::io::{self, BufRead};

fn is_valid_fqdn(domain: &str) -> bool {
    if domain.len() > 253 {
        return false;
    }

    if domain.ends_with('.') {
        return false;
    }

    let labels: Vec<&str> = domain.split('.').collect();
    if labels.len() < 2 {
        return false;
    }
    let tld = labels.last().unwrap();
    if tld.len() < 2 || tld.len() > 63 || !tld.chars().all(|c| c.is_ascii_alphabetic()) {
        return false;
    }
    for &label in &labels[..labels.len() - 1] {
        let len = label.len();
        if len == 0 || len > 63 {
            return false;
        }
        let chars: Vec<char> = label.chars().collect();
        if !chars.iter().all(|&c| c.is_ascii_alphanumeric() || c == '-') {
            return false;
        }
        if chars[0] == '-' || chars[len - 1] == '-' {
            return false;
        }
        if chars.iter().any(|&c| c == '_') {
            return false;
        }
    }
    true
}
fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    for _ in 0..t {
        let domain = lines.next().unwrap().unwrap().trim().to_string();
        println!("{}", is_valid_fqdn(&domain));
    }
}
