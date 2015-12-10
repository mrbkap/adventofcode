use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let mut unencoded : usize = 0;
    let mut reencoded : usize = 0;
    for string in buffer.split('\n') {
        let trimmed = string.trim().to_string();
        if trimmed.len() == 0 {
            continue;
        }

        reencoded += 2;
        let mut iter = trimmed.chars();
        while let Some(c) = iter.next() {
            unencoded += 1;
            match c {
                '"' | '\\' => {
                    reencoded += 2; // \"
                }
                _ => { reencoded += 1 }
            }
        }
    }

    println!("Count: {} chars: {} diff: {}", unencoded, reencoded, reencoded - unencoded);
}
