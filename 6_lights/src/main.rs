use std::io;
use std::io::BufRead;
use std::collections::HashMap;

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

fn iterate_range<F>(lights : &mut HashMap<u32, HashMap<u32, u32>>, r : Range, f : F) where F: Fn(&mut HashMap<u32, u32>, u32) {
    for row in r.start_row..r.end_row {
        if !lights.contains_key(&row) {
            lights.insert(row, HashMap::new());
        }

        let mut row_lights : &mut HashMap<u32, u32> = lights.get_mut(&row).unwrap();
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
        iterate_range(&mut lights, range, |row_lights : &mut HashMap<u32, u32>, col : u32| {
            match action {
                Action::TurnOn | Action::Toggle => {
                    let increase = match action {
                        Action::TurnOn => { 1 }
                        Action::Toggle => { 2 }
                        _ => { panic!(); }
                    };

                    if !row_lights.contains_key(&col) {
                        row_lights.insert(col, 0);
                    }

                    *row_lights.get_mut(&col).unwrap() += increase;
                }
                Action::TurnOff => {
                    if let Some(brightness) = row_lights.get_mut(&col) {
                        if *brightness > 0 {
                            *brightness = *brightness - 1;
                        }
                    }
                }
            }
        });
    }

    let mut brightness = 0u32;
    for (_, row_lights) in lights {
        for (_, light_bright) in row_lights {
            brightness += light_bright;
        }
    }

    println!("There is {} brightness" , brightness);
}
