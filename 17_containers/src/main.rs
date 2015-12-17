use std::io;
use std::io::BufRead;

const GOAL: u32 = 150;
const DEPTH_GOAL: usize = 4;

fn find_matches(v: &Vec<u32>, cur: u32, next: usize, depth: usize) -> u32 {
    if cur == GOAL {
        return if depth == DEPTH_GOAL { 1 } else { 0 };
    }
    if cur > GOAL {
        return 0;
    }

    let mut num_ways = 0;
    for i in next..v.len() {
        num_ways += find_matches(v, cur + v[i], i + 1, depth + 1);
    }
    return num_ways;
}

fn main() {
    let stdin = io::stdin();
    let containers: Vec<u32> = stdin.lock().lines().map(|l| l.unwrap().trim().parse::<u32>().unwrap()).collect();

    let mut num_ways = 0;
    for i in 0..containers.len() {
        num_ways += find_matches(&containers, containers[i], i + 1, 1);
    }

    println!("Number of ways: {}", num_ways);
}
