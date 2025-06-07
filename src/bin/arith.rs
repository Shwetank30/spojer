// Write a calculator that makes the output result formatted in the same way as usually used with manual calculations
// With addition and subtraction, the numbers are written below each other.
// Multiplication is a little bit more complex: first of all, we make a partial result for every digit of one of the numbers, and then sum the results together.

use std::char::from_digit;
use std::io::{self, BufRead};
fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    for _ in 0..t {
        let line = lines.next().unwrap().unwrap();
        let result = calculate_expression(&line);
        if !result.is_empty() {
            println!("{}", result);
        } else {
            eprintln!("Error processing expression: {}", line);
        }
    }
}

fn calculate_expression(expression: &str) -> String {
    if let Some(idx) = expression.find('+') {
        let (a, b) = (&expression[..idx].trim(), &expression[idx + 1..].trim());
        return format_addition(a, b);
    } else if let Some(idx) = expression.find('-') {
        let (a, b) = (&expression[..idx].trim(), &expression[idx + 1..].trim());
        return format_subtraction(a, b);
    } else if let Some(idx) = expression.find('*') {
        let (a, b) = (&expression[..idx].trim(), &expression[idx + 1..].trim());
        return format_multiplication(a, b);
    } else {
        eprintln!("Unsupported operation in expression: {}", expression);
    }
    String::new()
}

fn format_addition(a: &str, b: &str) -> String {
    let sum = add_strings(a, b);
    let op = '+';
    let b_with_op = format!("{}{}", op, b);

    let sep_len = b_with_op.len().max(sum.len());
    let max_len = a.len().max(sep_len);
    let a_padded = format!("{:>width$}", a, width = max_len);
    let b_padded = format!("{:>width$}", b_with_op, width = max_len);
    let sum_padded = format!("{:>width$}", sum, width = max_len);

    let mut dash_line = String::new();
    for _ in 0..(max_len - sep_len) {
        dash_line.push(' ');
    }
    for _ in 0..sep_len {
        dash_line.push('-');
    }
    format!("{}\n{}\n{}\n{}", a_padded, b_padded, dash_line, sum_padded)
}

fn format_subtraction(a: &str, b: &str) -> String {
    let diff = subtract_strings(a, b);
    let op = '-';
    let b_with_op = format!("{}{}", op, b);

    let sep_len = b_with_op.len().max(diff.len());
    let max_len = a.len().max(sep_len);

    let a_padded = format!("{:>width$}", a, width = max_len);
    let b_padded = format!("{:>width$}", b_with_op, width = max_len);
    let diff_padded = format!("{:>width$}", diff, width = max_len);

    // Separator: right-aligned, sep_len dashes
    let mut dash_line = String::new();
    for _ in 0..(max_len - sep_len) {
        dash_line.push(' ');
    }
    for _ in 0..sep_len {
        dash_line.push('-');
    }

    format!("{}\n{}\n{}\n{}", a_padded, b_padded, dash_line, diff_padded)
}

fn format_multiplication(a: &str, b: &str) -> String {
    let prod = multiply_strings(a, b);
    let op = '*';
    let max_len = a.len().max(b.len() + 1).max(prod.len());
    let a_padded = format!("{:>width$}", a, width = max_len);
    let b_with_op = format!("{}{}", op, b);
    let b_padded = format!("{:>width$}", b_with_op, width = max_len);
    let mut lines = vec![a_padded.clone(), b_padded.clone()];

    let mut partial_results = Vec::new();
    let mut partial_widths = Vec::new();

    if b.len() > 1 {
        for (i, digit) in b.chars().rev().enumerate() {
            let part = multiply_strings(a, &digit.to_string());
            let mut part_line = format!("{:>width$}", part, width = max_len);
            if i > 0 {
                part_line = part_line[i..].to_string() + &" ".repeat(i);
            }
            partial_widths.push(part_line.trim_end().len());
            partial_results.push(part_line);
        }
    }

    // First separator: as long as the max of (b_with_op.len(), first partial product), right-aligned
    let sep1_len = if !partial_results.is_empty() {
        b_with_op.len().max(partial_results[0].trim_start().len())
    } else {
        b_with_op.len().max(prod.len())
    };
    let sep1_start = max_len - sep1_len;
    let mut sep1 = String::new();
    for _ in 0..sep1_start {
        sep1.push(' ');
    }
    for _ in 0..sep1_len {
        sep1.push('-');
    }
    lines.push(sep1);

    // Add partials
    if !partial_results.is_empty() {
        for p in &partial_results {
            lines.push(p.clone());
        }
        if partial_results.len() > 1 {
            // Second separator: as long as the product, right-aligned
            let mut sep2 = String::new();
            for _ in 0..(max_len - prod.len()) {
                sep2.push(' ');
            }
            for _ in 0..prod.len() {
                sep2.push('-');
            }
            lines.push(sep2);
            lines.push(format!("{:>width$}", prod, width = max_len));
        } else {
            lines.push(format!("{:>width$}", prod, width = max_len));
        }
    } else {
        lines.push(format!("{:>width$}", prod, width = max_len));
    }

    lines.join("\n")
}

fn add_strings(a: &str, b: &str) -> String {
    let mut a = a.chars().rev().collect::<Vec<_>>();
    let mut b = b.chars().rev().collect::<Vec<_>>();
    let mut carry = 0;
    let mut res = Vec::new();
    let n = a.len().max(b.len());
    a.resize(n, '0');
    b.resize(n, '0');
    for i in 0..n {
        let sum = a[i].to_digit(10).unwrap() + b[i].to_digit(10).unwrap() + carry;
        res.push(from_digit(sum % 10, 10).unwrap());
        carry = sum / 10;
    }
    if carry > 0 {
        res.push(from_digit(carry, 10).unwrap());
    }
    let s = res
        .iter()
        .rev()
        .collect::<String>()
        .trim_start_matches('0')
        .to_string();
    if s.is_empty() {
        "0".to_string()
    } else {
        s
    }
}

fn subtract_strings(a: &str, b: &str) -> String {
    let mut a = a.chars().rev().collect::<Vec<_>>();
    let mut b = b.chars().rev().collect::<Vec<_>>();
    let mut res = Vec::new();
    let n = a.len().max(b.len());
    a.resize(n, '0');
    b.resize(n, '0');
    let mut borrow = 0;
    for i in 0..n {
        let mut d1 = a[i].to_digit(10).unwrap() as i32 - borrow;
        let d2 = b[i].to_digit(10).unwrap() as i32;
        if d1 < d2 {
            d1 += 10;
            borrow = 1;
        } else {
            borrow = 0;
        }
        res.push(from_digit((d1 - d2) as u32, 10).unwrap());
    }
    while res.len() > 1 && *res.last().unwrap() == '0' {
        res.pop();
    }
    res.iter().rev().collect()
}

fn multiply_strings(a: &str, b: &str) -> String {
    let a: Vec<u32> = a.chars().rev().map(|c| c.to_digit(10).unwrap()).collect();
    let b: Vec<u32> = b.chars().rev().map(|c| c.to_digit(10).unwrap()).collect();
    let mut res = vec![0; a.len() + b.len()];
    for i in 0..a.len() {
        for j in 0..b.len() {
            res[i + j] += a[i] * b[j];
        }
    }
    for i in 0..res.len() - 1 {
        res[i + 1] += res[i] / 10;
        res[i] %= 10;
    }
    while res.len() > 1 && *res.last().unwrap() == 0 {
        res.pop();
    }
    res.iter()
        .rev()
        .map(|d| from_digit(*d, 10).unwrap())
        .collect()
}
