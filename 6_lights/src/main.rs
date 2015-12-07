use std::io;
use std::io::BufRead;
use std::collections::{HashMap, HashSet};

struct Range {
    start_row : u32,
    end_row : u32,
    start_col : u32,
    end_col : u32,
}

enum Action {
    TurnOn,
    TurnOff,
    Toggle
}

fn parse_point(word : &str) -> (u32, u32) {
    let point : Vec<&str> = word.split(',').collect();
    assert!(point.len() == 2);

    let start = point[0].parse::<u32>().unwrap();
    let end = point[1].parse::<u32>().unwrap();
    return (start, end);
}

fn parse_line(line : &str) -> (Action, Range) {
    let words : Vec<&str> = line.split_whitespace().collect();
    let mut range_start = 1;
    let action = match words[0] {
        "toggle" => { Action::Toggle }
        _ => {
            range_start = 2;
            match words[1] {
                "on" => { Action::TurnOn }
                "off" => { Action::TurnOff }
                _ => { panic!() }
            }
        }
    };

    assert!(words.len() == range_start + 3);
    let (start_row, start_col) = parse_point(&words[range_start]);
    let (end_row, end_col) = parse_point(&words[range_start + 2]);

    let range = Range {
        start_row: start_row,
        start_col: start_col,
        end_row: end_row + 1,
        end_col: end_col + 1,
    };

    return (action, range);
}

fn iterate_range<F>(lights : &mut HashMap<u32, HashSet<u32>>, r : Range, f : F) where F: Fn(&mut HashSet<u32>, u32) {
    for row in r.start_row..r.end_row {
        if !lights.contains_key(&row) {
            lights.insert(row, HashSet::new());
        }

        let mut row_lights : &mut HashSet<u32> = lights.get_mut(&row).unwrap();
        for col in r.start_col..r.end_col {
            f(row_lights, col);
        }
    }
}

fn main() {
    let mut lights = HashMap::new();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let (action, range) = parse_line(line.unwrap().trim());
        iterate_range(&mut lights, range, |row_lights : &mut HashSet<u32>, col : u32| {
            match action {
                Action::TurnOn => { row_lights.insert(col); }
                Action::TurnOff => { row_lights.remove(&col); }
                Action::Toggle => {
                    if row_lights.contains(& col) {
                        row_lights.remove(& col);
                    } else {
                        row_lights.insert(col);
                    }
                }
            }
        });
    }

    let mut num_lights = 0u32;
    for (_, row_lights) in lights {
        num_lights += row_lights.len() as u32;
    }

    println!("There are {} lights" , num_lights);
}
