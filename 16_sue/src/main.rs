//#![feature(step_by)]

use std::io;
use std::io::BufRead;
use std::collections::HashMap;

struct Sue {
    number: u32,
    attrs: HashMap<String, u32>,
}

impl Sue {
    fn from_attrs(l: &str) -> Sue {
        let words: Vec<_> = l.split_whitespace().collect();
        let number = words[1].trim_right_matches(':').parse::<u32>().unwrap();

        let mut rval = Sue { number: number, attrs: HashMap::new() };
        for i in 2..words.len() {
            if i % 2 != 0 {
                continue;
            }
            let key = words[i].trim_right_matches(':');
            let val = words[i + 1].trim_right_matches(',').parse::<u32>().unwrap();

            rval.attrs.insert(key.to_string(), val);
        }

        return rval;
    }
}

fn main() {
    let target_sue = "Sue 0: children: 3, cats: 7, samoyeds: 2, pomeranians: 3, akitas: 0, vizslas: 0, goldfish: 5, trees: 3, cars: 2, perfumes: 1";
    let target = Sue::from_attrs(target_sue);

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let unwrapped = line.unwrap();
        let attrs = unwrapped.trim();
        if attrs.is_empty() {
            continue;
        }

        let mut matched = true;
        let needle = Sue::from_attrs(&attrs);
        for (k, a) in &needle.attrs {
            match target.attrs.get(k) {
                Some(targetval) => {
                    match k.as_ref() {
                        "cats" | "trees" => {
                            if a < targetval {
                                matched = false;
                                break;
                            }
                        }
                        "pomeranians" | "goldfish" => {
                            if a >= targetval {
                                matched = false;
                                break;
                            }
                        }
                        _ => {
                            if a != targetval {
                                matched = false;
                                break;
                            }
                        }
                    }
                }
                None => {
                    matched = false;
                    break;
                }
            }
        }

        if !matched {
            continue;
        }

        println!("sue {} matched! {:?}", needle.number, needle.attrs);
    }
}
