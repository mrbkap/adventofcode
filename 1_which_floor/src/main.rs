use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let mut floor : i64 = 0;
    let mut pos : usize = 0;
    for (i, c) in buffer.trim().chars().enumerate() {
        match c {
            '(' => {
                floor = floor + 1;
            }
            ')' => {
                floor = floor - 1;
                if floor < 0 && pos == 0 {
                    pos = i + 1;
                }
            }
            c => { println!("Error, unexpected character {}", c); }
        }
    }

    println!("Floor: {}", floor);
    println!("First basement: {}", pos);
}
