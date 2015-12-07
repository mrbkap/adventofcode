use std::io;
use std::io::BufRead;

fn word_is_nice(word : &str) -> bool {
    let mut last : char = 0 as char;
    let mut saw_dup = false;
    let mut num_vowels = 0u32;
    for c in word.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => { num_vowels += 1; }
            'b' => {
                if last == 'a' {
                    return false;
                }
            }
            'd' => {
                if last == 'c' {
                    return false;
                }
            }
            'q' => {
                if last == 'p' {
                    return false;
                }
            }
            'y' => {
                if last == 'x' {
                    return false;
                }
            }
            _ => { }
        }

        if c == last {
            saw_dup = true;
        }

        last = c;
    }

    return saw_dup && num_vowels >= 3;
}

fn main() {
    let mut num_nice = 0u64;

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        if word_is_nice(line.unwrap().trim()) {
            num_nice += 1;
        }
    }

    println!("Total nice strings: {}", num_nice);
}
