use std::io;
use std::io::BufRead;
use std::collections::HashSet;

fn find_smallest(packages: &[u64], used: &mut HashSet<u64>, skip: usize, target: u64, cur: u64, prod: u64) -> (usize, u64) {
    if used.len() > 6 {
        return (7, 0);
    }
    if cur == target {
        let qte = used.iter().fold(1, |acc, p| acc * p);
        if qte < prod {
            println!("Found len {} ({:?}) {}", used.len(), used, qte);
        }
        return (used.len(), qte);
    }
    if cur > target {
        return (usize::max_value(), 0);
    }

    let mut smallest = usize::max_value();
    let mut qte = prod;
    for p in packages.iter().rev().skip(skip) {
        if used.contains(p) {
            continue;
        }

        used.insert(*p);
        let (next, nextqte) = find_smallest(packages, used, skip + 1, target, cur + p, qte);
        if next < smallest {
            smallest = next;
            if nextqte < qte {
                qte = nextqte;
            }
        }
        used.remove(p);
    }

    return (smallest, qte);
}

fn main() {
    let mut packages = Vec::new();
    let mut total = 0u64;
    let stdin = io::stdin();
    for maybeline in stdin.lock().lines() {
        let line = maybeline.unwrap();
        packages.push(line.trim().parse::<u64>().unwrap());
        total += *packages.last().unwrap();
    }

    total /= 4;
    let (len, qte) = find_smallest(&packages, &mut HashSet::new(), 0, total, 0, u64::max_value());
    println!("smallest was {} (qte: {})", len, qte);
}
