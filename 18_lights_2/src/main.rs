use std::io;
use std::io::Read;

const N: usize = 100;

fn btoi(b: bool) -> u8 {
    if b { 1 } else { 0 }
}

fn count_neighbors(buf: &Vec<Vec<bool>>, r: usize, c: usize) -> u8 {
    return btoi(buf[r - 1][c - 1]) + btoi(buf[r - 1][c]) + btoi(buf[r - 1][c + 1]) +
           btoi(buf[r][c - 1])     + 0                   + btoi(buf[r][c + 1]) +
           btoi(buf[r + 1][c - 1]) + btoi(buf[r + 1][c]) + btoi(buf[r + 1][c + 1]);
}

fn step(buf: &Vec<Vec<bool>>, work_buf: &mut Vec<Vec<bool>>) {
    for r in 1..(N + 1) {
        for c in 1..(N + 1) {
            let neighbors = count_neighbors(buf, r, c);
            work_buf[r][c] = if buf[r][c] { match neighbors { 2|3 => true, _ => false } } else { neighbors == 3 };
        }
    }
}

fn main() {
    let mut buf1: Vec<Vec<bool>> = (0..(N + 2)).map(|_| vec![false; (N + 2)]).collect();
    let mut buf2: Vec<Vec<bool>> = (0..(N + 2)).map(|_| vec![false; (N + 2)]).collect();

    let mut cur_buf: &mut Vec<Vec<bool>> = &mut buf1;
    let mut other_buf: &mut Vec<Vec<bool>> = &mut buf2;

    let stdin = io::stdin();
    let mut bytes = stdin.bytes();
    for r in 1..(N + 1) {
        for c in 1..(N + 1) {
            let next: char = bytes.next().unwrap().unwrap() as char;
            cur_buf[r][c] = next == '#';
        }

        match bytes.next().unwrap().unwrap() as char {
            '\n' => {}
            _ => { panic!("unexpected character"); }
        }
    }

    for _ in 0..100 {
        step(cur_buf, other_buf);
        std::mem::swap(cur_buf, other_buf);
    }

    let mut num_on: u32 = 0;
    for r in 1..(N + 1) {
        for c in 1..(N + 1) {
            if cur_buf[r][c] {
                num_on += 1;
            }
        }
    }

    println!("Num on: {}", num_on);
}
