use std::io;
use std::io::BufRead;

struct Rep {
    left: String,
    right: String,
}

fn try(m: &str, d: u32, c: usize, rs: &Vec<Rep>) -> Option<u32> {
    if m == "e" {
        return Some(d);
    }

    for r in rs.iter() {
        if let Some(idx) = m.find(&r.right) {
            let mm = m[0..idx].to_string() + &r.left + &m[(idx + r.right.len())..];
            if let Some(depth) = try(&mm, d + 1, c + 1, rs) {
                return Some(depth);
            }
        }
    }

    return None;
}

fn main() {
    let stdin = io::stdin();

    let mut replacements: Vec<Rep> = Vec::new();
    let mut lines_iter = stdin.lock().lines();
    loop {
        let line = lines_iter.next().unwrap().unwrap();
        let trimmed = line.trim();
        if trimmed.is_empty() {
            break;
        }

        let words: Vec<_> = trimmed.split(" => ").collect();
        let rep = Rep { left: words[0].to_string(), right: words[1].to_string(), };
        match replacements.iter().position(|r| { r.right.len() <= words[1].len() }) {
            Some(idx) => { replacements.insert(idx, rep); }
            None => { replacements.push(rep); }
        }
    }

    let raw_molecule = lines_iter.next().unwrap().unwrap();
    let molecule = raw_molecule.trim();

    let idx = try(molecule, 0, 0, &replacements);
    println!("{:?}", idx);
}
