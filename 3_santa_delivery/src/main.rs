use std::io;
use std::io::Read;
use std::collections::HashSet;

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
struct Point {
    x: i64,
    y: i64,
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let mut houses_visited = 1u64;
    let mut current_santa = Point { x: 0, y: 0 };
    let mut current_robo = Point { x: 0, y: 0 };
    let mut robo_turn = false;
    let mut visited = HashSet::<Point>::new();

    visited.insert(current_santa);

    for c in buffer.trim().chars() {
        let mut current : &mut Point = if robo_turn {
            &mut current_robo
        } else {
            &mut current_santa
        };
        robo_turn = !robo_turn;

        match c {
            '>' => {
                current.x += 1;
            }
            '<' => {
                current.x -= 1;
            }
            'v' => {
                current.y += 1;
            }
            '^' => {
                current.y -= 1;
            }
            _ => {
                println!("Unexpected character {}", c);
                continue;
            }
        }

        if visited.contains(&current) {
            continue;
        }

        visited.insert(*current);
        houses_visited += 1;
    }

    println!("They visited {} houses", houses_visited);
}
