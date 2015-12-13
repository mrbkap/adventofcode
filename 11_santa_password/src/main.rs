use std::io::{self, Read};

fn inc(s: &str) -> String {
    let z = 'z' as u8;

    let mut carry = true;
    let mut newpass: String = s.trim().chars().rev().map(|c| {
        let next = if !carry {
            c
        } else {
            let d = (c as u8) + 1;
            if d > z { carry = true; 'a' } else { carry = false; d as char }
        };

        return next;
    }).collect();

    if carry {
        newpass.push('a');
    }

    return newpass.chars().rev().collect();
}

fn valid(s: &str) -> bool {
    if s.len() < 3 {
        return false;
    }

    let mut firstdouble: char = 0 as char;
    let mut seconddouble: char = 0 as char;
    let mut back2: u8 = 0;
    let mut back1: u8 = 0;
    let mut found = false;
    for c in s.chars() {
        match c {
            'i' | 'o' | 'l' => { return false; }
            _ => { }
        }

        if back2 + 1 == back1 && back1 + 1 == (c as u8) {
            found = true;
        }

        if back1 == (c as u8) {
            if firstdouble == (0 as char) {
                firstdouble = c;
            } else if c != firstdouble {
                seconddouble = c;
            }
        }

        back2 = back1;
        back1 = c as u8;
    }

    if !found {
        return false;
    }

    return seconddouble != (0 as char);
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let mut newpass: String = buffer.trim().to_string();
    loop {
        newpass = inc(&newpass);
        println!("trying {}", newpass);
        if valid(&newpass) {
            break;
        }
    }

    println!("new pass {}", newpass);
}
