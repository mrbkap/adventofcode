use std::io::{self, Read};

fn convert(s: &str) -> String {
    if s.is_empty() {
        return s.to_string();
    }

    let mut output = String::new();
    let mut cnt = 0u32;
    let mut last = 0 as char;
    for (i, c) in s.chars().enumerate() {
        if c == last {
            cnt += 1;
            continue;
        }

        if i != 0 {
            output.push_str(&cnt.to_string());
            output.push_str(&last.to_string());
        }

        last = c;
        cnt = 1;
    }

    output.push_str(&cnt.to_string());
    output.push_str(&last.to_string());

    output
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    buffer = buffer.trim().to_string();

    for _ in 0..50 {
        buffer = convert(&buffer);
    }

    println!("Result: {}", buffer.len());
}
