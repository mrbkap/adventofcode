use std::io;
use std::io::BufRead;
use std::collections::{HashMap, HashSet};

#[derive(Clone)]
enum Operand {
    Const(u16),
    Gate(String),
}

#[derive(Clone)]
enum Value {
    And(Operand, Operand),
    Or(Operand, Operand),
    Lshift(Operand, u16),
    Rshift(Operand, u16),
    Not(Operand),
    Move(Operand),
}

#[derive(Clone)]
enum MaybeResolved {
    Value(Value),
    Resolved(u16),
}

struct GateGame {
    gates: HashMap<String, MaybeResolved>
}

impl <'a> GateGame {
    pub fn new() -> GateGame {
        return GateGame { gates: HashMap::new() };
    }

    fn parse_operand(word: &str) -> Operand {
        match word.chars().next().unwrap() {
            '0'...'9' => {
                Operand::Const(word.parse::<u16>().unwrap())
            }
            _ => {
                Operand::Gate(word.to_string())
            }
        }
    }

    pub fn handle_line(self: &'a mut GateGame, l: &str) {
        let v : Vec<&str> = l.split_whitespace().collect();

        let target = v.last().unwrap().to_string();

        // Only prefix operator.
        if v[0] == "NOT" {
            assert!(v.len() == 4);
            let rhs = GateGame::parse_operand(v[1]);
            self.gates.insert(target, MaybeResolved::Value(Value::Not(rhs)));
            return;
        }

        let lhs = GateGame::parse_operand(v[0]);
        // Move is the only unary operator.
        if v[1] == "->" {
            self.gates.insert(target, MaybeResolved::Value(Value::Move(lhs)));
            return;
        }

        let rhs = GateGame::parse_operand(v[2]);
        let value = match v[1] {
            "AND" => { Value::And(lhs, rhs) }
            "OR" => { Value::Or(lhs, rhs) }
            "LSHIFT" => {
                // Must shift by constant numbers.
                if let Operand::Const(shift) = rhs {
                    Value::Lshift(lhs, shift)
                } else {
                    panic!("bad left shift");
                }
            }
            "RSHIFT" => {
                // Must shift by constant numbers.
                if let Operand::Const(shift) = rhs {
                    Value::Rshift(lhs, shift)
                } else {
                    panic!("bad right shift");
                }
            }
            _ => { panic!(); }
        };

        self.gates.insert(target, MaybeResolved::Value(value));
    }

    pub fn resolve(&mut self) {
        let mut resolving = HashSet::new();
        let v: Vec<_> = self.gates.keys().cloned().collect();
        for key in v {
            self._resolve(&Operand::Gate(key), &mut resolving);
        }
    }

    pub fn resolve_gate(&mut self, gate: &str) -> u16 {
        let mut resolving = HashSet::new();
        self._resolve(&Operand::Gate(gate.to_string()), &mut resolving)
    }

    fn _resolve(self: &'a mut GateGame, o: &Operand, resolving: &mut HashSet<String>) -> u16 {
        if let &Operand::Const(c) = o {
            return c;
        }

        let gname: &str = if let &Operand::Gate(ref g) = o { g } else { panic!() };
        if resolving.contains(gname) {
            panic!("recursive lookup");
        }

        resolving.insert(gname.to_string());
        let value = self.gates.get(gname).unwrap().clone();

        let result = match value {
            MaybeResolved::Resolved(r) => {
                r
            }
            MaybeResolved::Value(Value::Not(ref s)) => {
                let rhs = self._resolve(s, resolving);
                !rhs
            }
            MaybeResolved::Value(Value::Move(ref s)) => {
                let rhs = self._resolve(s, resolving);
                rhs
            }
            MaybeResolved::Value(Value::And(ref lhs, ref rhs)) => {
                let lhsv = self._resolve(lhs, resolving);
                let rhsv = self._resolve(rhs, resolving);
                lhsv & rhsv
            }
            MaybeResolved::Value(Value::Or(ref lhs, ref rhs)) => {
                let lhsv = self._resolve(lhs, resolving);
                let rhsv = self._resolve(rhs, resolving);
                lhsv | rhsv
            }
            MaybeResolved::Value(Value::Lshift(ref lhs, howmuch)) => {
                let lhsv = self._resolve(lhs, resolving);
                lhsv << howmuch
            }
            MaybeResolved::Value(Value::Rshift(ref lhs, howmuch)) => {
                let lhsv = self._resolve(lhs, resolving);
                lhsv >> howmuch
            }
        };

        self.gates.insert(gname.to_string(), MaybeResolved::Resolved(result));
        resolving.remove(gname);

        return result;
    }

    fn iter(self: &GateGame) -> std::collections::hash_map::Iter<String, MaybeResolved> {
        return self.gates.iter();
    }
}

fn main() {
    let mut game = GateGame::new();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        game.handle_line(line.unwrap().trim());
    }

    // Part two.
    game.gates.insert("b".to_string(), MaybeResolved::Resolved(3176u16));

    // Not strictly necessary, but nice to do.
    game.resolve();
    for (gatename, value) in game.iter() {
        match value {
            &MaybeResolved::Resolved(v) => {
                println!("{} is {}", gatename, v);
            }
            _ => {
                panic!();
            }
        }
    }

    // OK all on its own.
    println!("a is {}", game.resolve_gate("a"));
}
