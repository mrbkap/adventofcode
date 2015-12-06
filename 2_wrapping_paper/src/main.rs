use std::io;
use std::cmp::min;
use std::io::BufRead;

fn parse_line(l : &str) -> (u64, u64, u64) {
    let v : Vec<&str> = l.split('x').collect();
    assert!(v.len() == 3);

    let w = v[0].parse::<u64>().unwrap();
    let h = v[1].parse::<u64>().unwrap();
    let l = v[2].parse::<u64>().unwrap();

    (w, h, l)
}

fn two_smallest(w : u64, h : u64, l : u64) -> (u64, u64) {
    let smallest = min(w, min(h, l));
    let second_side =
        if smallest == w {
            min(h, l)
        } else if smallest == h {
            min(w, l)
        } else {
            min(w, h)
        };

    (smallest, second_side)
}

fn wrapping_paper_for(w : u64, h : u64, l : u64) -> u64 {
    let (smallest, second_side) = two_smallest(w, h, l);
    (2*l*w + 2*w*h + 2*h*l) + smallest * second_side
}

fn ribbon_for(w : u64, h : u64, l : u64) -> u64 {
    let (smallest, second_side) = two_smallest(w, h, l);

    (smallest * 2 + second_side * 2) + w * h * l
}

fn main() {
    let mut total_wrapping_paper = 0u64;
    let mut total_ribbon = 0u64;
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let (w, h, l) = parse_line(line.unwrap().trim());
        total_wrapping_paper += wrapping_paper_for(w, h, l);
        total_ribbon += ribbon_for(w, h, l);
    }

    println!("Total wrapping paper: {}", total_wrapping_paper);
    println!("Total ribbon: {}", total_ribbon);
}
