use std::io;
use std::io::BufRead;
use std::collections::{HashMap, HashSet};

#[derive(Clone)]
enum Value {
    And(String, String),
    Or(String, String),
    Lshift(String, u16),
    Rshift(String, u16),
    Not(String),
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

    pub fn handle_line(self: &'a mut GateGame, l: &str) {
        let v : Vec<&str> = l.split_whitespace().collect();

        match v[0].chars().next().unwrap() {
            'N' => {
                assert!(v.len() == 4);
                self.gates.insert(v.last().unwrap().to_string(), MaybeResolved::Value(Value::Not(v[1].to_string())));
            }
            '0'...'9' => {
                assert!(v.len() == 3);
                self.gates.insert(v.last().unwrap().to_string(), MaybeResolved::Resolved(v[0].parse::<u16>().unwrap()));
            }
            _  => {
                assert!(v.len() == 5);
                let value = match v[1] {
                    "AND" => { Value::And(v[0].to_string(), v[2].to_string()) }
                    "OR" => { Value::Or(v[0].to_string(), v[2].to_string()) }
                    "LSHIFT" => { Value::Lshift(v[0].to_string(), v[2].parse::<u16>().unwrap()) }
                    "RSHIFT" => { Value::Rshift(v[0].to_string(), v[2].parse::<u16>().unwrap()) }
                    _ => { panic!(); }
                };

                self.gates.insert(v.last().unwrap().to_string(), MaybeResolved::Value(value));
            }
        }
    }

    pub fn resolve(&mut self) {
        let mut resolving = HashSet::new();
        let v: Vec<_> = self.gates.keys().cloned().collect();
        for key in v {
            self._resolve(&key, &mut resolving);
        }
    }

    fn _resolve(self: &'a mut GateGame, gname: &str, resolving: &mut HashSet<String>) -> u16 {
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

    game.resolve();

    for (gatename, value) in game.iter() {
        if let MaybeResolved::Resolved(v) = *value {
            println!("{} is {}", gatename, v);
        }
        else {
            panic!();
        }
    }
}
