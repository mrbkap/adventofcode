use std::io;
use std::io::BufRead;
use std::collections::{HashMap, HashSet};

enum Value {
    Const(u32),
    And(String, String),
    Or(String, String),
    Lshift(String, u32),
    Rshift(String, u32),
    Not(String),
}

enum MaybeResolved {
    Value(Value),
    Resolved(u32),
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

        match v[0].char_at(0) {
            'N' => {
                assert!(v.len() == 4);
                self.gates.insert(v[-1].to_string(), MaybeResolved::Value(Value::Not(v[1].to_string())));
            }
            '0'...'9' => {
                assert!(v.len() == 3);
                self.gates.insert(v[-1].to_string(), MaybeResolved::Resolved(v[0].parse::<u32>().unwrap()));
            }
            _  => {
                assert!(v.len() == 5);
                let value = match v[1] {
                    "AND" => { Value::And(v[0].to_string(), v[2].to_string()) }
                    "OR" => { Value::Or(v[0].to_string(), v[2].to_string()) }
                    "LSHIFT" => { Value::Lshift(v[0].to_string(), v[2].parse::<u32>().unwrap()) }
                    "RSHIFT" => { Value::Rshift(v[0].to_string(), v[2].parse::<u32>().unwrap()) }
                    _ => { panic!(); }
                };

                self.gates.insert(v[-1].to_string(), MaybeResolved::Value(value));
            }
        }
    }

    pub fn resolve(self: &'a mut GateGame) {
        let mut resolving = HashSet::new();
        let keys = {
            self.gates.keys().copy()
        };
        for key in keys {
            self._resolve(&key, &mut resolving);
        }
    }

    fn _resolve(self: &'a mut GateGame, gname: &str, resolving: &mut HashSet<String>) -> u32 {
        if resolving.contains(gname) {
            panic!("recursive lookup");
        }

        resolving.insert(gname.to_string());
        let mut value = self.gates.entry(gname.to_string()).or_insert_with(|| { panic!() });

        let result = match *value {
            MaybeResolved::Resolved(r) => {
                r
            }
            MaybeResolved::Value(Value::Not(ref s)) => {
                let rhs = self._resolve(s, &mut resolving);
                !rhs
            }
            MaybeResolved::Value(Value::And(ref lhs, ref rhs)) => {
                let lhsv = self._resolve(lhs, &mut resolving);
                let rhsv = self._resolve(rhs, &mut resolving);
                lhsv & rhsv
            }
            MaybeResolved::Value(Value::Or(ref lhs, ref rhs)) => {
                let lhsv = self._resolve(lhs, &mut resolving);
                let rhsv = self._resolve(rhs, &mut resolving);
                lhsv & rhsv
            }
            MaybeResolved::Value(Value::Lshift(ref lhs, howmuch)) => {
                let lhsv = self._resolve(lhs, &mut resolving);
                lhsv << howmuch
            }
            MaybeResolved::Value(Value::Rshift(ref lhs, howmuch)) => {
                let lhsv = self._resolve(lhs, &mut resolving);
                lhsv >> howmuch
            }
            MaybeResolved::Value(Value::Const(c)) => {
                c
            }
        };

        *value = MaybeResolved::Resolved(result);
        resolving.remove(gname);

        return result;
    }

    fn iter(self: &GateGame) -> std::collections::hash_map::Iter<String, MaybeResolved> {
        return self.gates.iter();
    }
}

/*
fn resolve<'a>(env: &'a mut HashMap<String, Gate>, resolving: &'a mut HashSet<String>, gate: &'a mut Gate) -> u32 {
}
*/

fn main() {
    let mut game = GateGame::new();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        game.handle_line(line.unwrap().trim());
    }

    for (gatename, value) in game.iter() {
        if let MaybeResolved::Resolved(v) = *value {
            println!("{} is {}", gatename, v);
        }
        else {
            panic!();
        }
    }
}
