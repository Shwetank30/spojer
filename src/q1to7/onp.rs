// Transform algebraic expressions in infix notation to postfix notation (Reverse Polish Notation).

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    for _ in 0..t {
        let line = lines.next().unwrap().unwrap();
        let result = inflix_to_postfix(&line);
        if !result.is_empty() {
            println!("{}", result);
        } else {
            eprintln!("Error processing expression: {}", line);
        }
    }
    if t == 0 {
        eprintln!("No expressions provided.");
    }
}

fn inflix_to_postfix(expression: &str) -> String {
    let mut stack: Vec<char> = Vec::new();
    let precedence = |c: char| match c {
        '+' | '-' => 1,
        '*' | '/' => 2,
        '^' => 3,
        _ => 0,
    };
    let is_operator = |c: char| matches!(c, '+' | '-' | '*' | '/' | '^');
    let is_operand = |c: char| c.is_alphanumeric() || c == '_';
    let mut output = String::new();
    let mut tokens = expression.chars().peekable();
    while let Some(&c) = tokens.peek() {
        if c.is_whitespace() {
            tokens.next();
            continue;
        }
        if is_operand(c) {
            output.push(c);
            tokens.next();
        } else if is_operator(c) {
            while let Some(&top) = stack.last() {
                if is_operator(top) && precedence(top) >= precedence(c) {
                    output.push(stack.pop().unwrap());
                } else {
                    break;
                }
            }
            stack.push(c);
            tokens.next();
        } else if c == '(' {
            stack.push(c);
            tokens.next();
        } else if c == ')' {
            while let Some(top) = stack.pop() {
                if top == '(' {
                    tokens.next();
                    break;
                }
                output.push(top);
            }
        } else {
            // Handle unexpected characters
            eprintln!("Unexpected character: {}", c);
            return String::new();
        }
    }
    while let Some(top) = stack.pop() {
        if top != '(' {
            output.push(top);
        } else {
            eprintln!("Mismatched parentheses");
            return String::new();
        }
    }
    if output.is_empty() {
        eprintln!("Empty output, possibly due to mismatched parentheses or invalid expression");
        return String::new();
    }
    output.push('\n');
    output
}
