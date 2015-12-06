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

fn wrapping_paper_for(w : u64, h : u64, l : u64) -> u64 {
    let smallest = min(w, min(h, l));
    let second_side =
        if smallest == w {
            min(h, l)
        } else if smallest == h {
            min(w, l)
        } else {
            min(w, h)
        };

    (2*l*w + 2*w*h + 2*h*l) + smallest * second_side
}

fn main() {
    let mut total : u64 = 0;
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let (w, h, l) = parse_line(line.unwrap().trim());
        total += wrapping_paper_for(w, h, l);
    }

    println!("Total: {}", total);
}
