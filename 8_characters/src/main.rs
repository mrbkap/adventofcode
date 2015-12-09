use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let mut count : usize = 0;
    let mut chars : usize = 0;
    for string in buffer.split('\n') {
        let trimmed = string.trim().to_string();
        if trimmed.len() == 0 {
            continue;
        }

        count += 2;
        let contents = &trimmed[1..trimmed.len() - 1];
        let mut iter = contents.chars();
        while let Some(c) = iter.next() {
            count += 1;
            match c {
                '\\' => {
                    match iter.nth(0) {
                        Some('\\') | Some('"') => {
                            count += 1;
                            chars += 1;
                        }
                        Some('x') => {
                            match iter.nth(1) {
                                Some(_) => { count += 3; chars += 1; }
                                None => { panic!(); }
                            }
                        }
                        _ => { panic!("bad escape sequence {}", c); }
                    }
                }
                _ => { chars += 1; }
            }
        }
    }

    println!("Count: {} chars: {} diff: {}", count, chars, count - chars);
}
