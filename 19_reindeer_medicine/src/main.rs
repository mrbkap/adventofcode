use std::io;
use std::io::BufRead;
use std::collections::{HashMap, HashSet};

fn main() {
    let stdin = io::stdin();

    let mut replacements: HashMap<String, Vec<String>> = HashMap::new();
    let mut lines_iter = stdin.lock().lines();
    loop {
        let line = lines_iter.next().unwrap().unwrap();
        let trimmed = line.trim();
        if trimmed.is_empty() {
            break;
        }

        let words: Vec<_> = trimmed.split(" => ").collect();
        let looked_up = replacements.entry(words[0].to_string()).or_insert(Vec::new());
        looked_up.push(words[1].to_string());
    }

    let raw_molecule = lines_iter.next().unwrap().unwrap();
    let molecule = raw_molecule.trim();

    let mut transformed = HashSet::new();
    for (key, reps) in &replacements {
        for (i, _) in molecule.match_indices(key) {
            for rep in reps {
                let mutated = molecule[0..i].to_string() + rep + &molecule[(i + key.len())..];
                transformed.insert(mutated);
            }
        }
    }

    println!("there were {}", transformed.len());
}
