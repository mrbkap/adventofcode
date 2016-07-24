use std::io;
use std::io::BufRead;

type Register = usize;

#[derive(Debug)]
enum Instr {
    Hlf(Register),
    Tpl(Register),
    Inc(Register),
    Jmp(i32),
    Jie(Register, i32),
    Jio(Register, i32),
}

fn register(r: &str) -> Register {
    match r {
        "a" => 0,
        "b" => 1,
        _ => { panic!(); }
    }
}

fn imm(word: &str) -> i32 {
    word.parse::<i32>().unwrap()
}

fn parse_line(line: &str) -> Instr {
    let split: Vec<_> = line.split_whitespace().collect();
    match split[0] {
        "hlf" => { Instr::Hlf(register(split[1])) }
        "tpl" => { Instr::Tpl(register(split[1])) }
        "inc" => { Instr::Inc(register(split[1])) }
        "jmp" => { Instr::Jmp(imm(split[1])) }
        "jie" => { Instr::Jie(register(split[1].trim_right_matches(",")), imm(split[2])) }
        "jio" => { Instr::Jio(register(split[1].trim_right_matches(",")), imm(split[2])) }
        _ => { panic!(); }
    }
}

fn main() {
    let mut regs = vec![0u32; 2];
    let mut prog: Vec<Instr> = Vec::new();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let unwrapped = line.unwrap();
        let line = unwrapped.trim();
        prog.push(parse_line(line));
    }

    let mut pc: i32 = 0;
    loop {
        let curpc = pc as usize;
        if curpc >= prog.len() {
            break;
        }

        pc += match prog[curpc] {
            Instr::Hlf(r) => { regs[r] /= 2; 1 }
            Instr::Tpl(r) => { regs[r] *= 3; 1 }
            Instr::Inc(r) => { regs[r] += 1; 1 }
            Instr::Jmp(offset) => { offset }
            Instr::Jie(r, offset) => {
                if regs[r] % 2 == 0 { offset } else { 1 }
            }
            Instr::Jio(r, offset) => {
                if regs[r] == 1 { offset } else { 1 }
            }
        }
    }

    println!("Register(a) = {}", regs[0]);
    println!("Register(b) = {}", regs[1]);
}
