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

struct Gate {
    name : String,
    value : MaybeResolved,
}

fn parse_line(l : &str) -> Gate {
    let v : Vec<&str> = l.split_whitespace().collect();

    match v[0].char_at(0) {
        'N' => {
            assert!(v.len() == 4);
            return Gate { name: v[-1].to_string(), value: MaybeResolved::Value(Value::Not(v[1].to_string())) };
        }
        '0'...'9' => {
            assert!(v.len() == 3);
            return Gate { name: v[-1].to_string(), value: MaybeResolved::Resolved(v[0].parse::<u32>().unwrap()) };
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

            return Gate { name: v[-1].to_string(), value: MaybeResolved::Value(value) };
        }
    }
}

fn resolve<'a>(env: &'a mut HashMap<String, Gate>, resolving: &'a mut HashSet<String>, gate: &'a mut Gate) -> u32 {
    if resolving.contains(&gate.name) {
        panic!("recursive lookup");
    }

    resolving.insert(gate.name.to_string());

    let result = match gate.value {
        MaybeResolved::Resolved(r) => {
            r
        }
        MaybeResolved::Value(Value::Not(ref s)) => {
            let mut sgate : &mut Gate;
            let rhs = resolve(env, resolving, &mut sgate);
            !rhs
        }
        MaybeResolved::Value(Value::And(ref lhs, ref rhs)) => {
            let lhsgate = env.get_mut(lhs).unwrap();
            let rhsgate = env.get_mut(rhs).unwrap();
            let lhsv = resolve(env, resolving, lhsgate);
            let rhsv = resolve(env, resolving, rhsgate);
            lhsv & rhsv
        }
        MaybeResolved::Value(Value::Or(ref lhs, ref rhs)) => {
            let lhsgate = env.get_mut(lhs).unwrap();
            let rhsgate = env.get_mut(rhs).unwrap();
            let lhsv = resolve(env, resolving, lhsgate);
            let rhsv = resolve(env, resolving, rhsgate);
            lhsv & rhsv
        }
        MaybeResolved::Value(Value::Lshift(ref lhs, howmuch)) => {
            let lhsgate = env.get_mut(lhs).unwrap();
            let lhsv = resolve(env, resolving, lhsgate);
            lhsv << howmuch
        }
        MaybeResolved::Value(Value::Rshift(ref lhs, howmuch)) => {
            let lhsgate = env.get_mut(lhs).unwrap();
            let lhsv = resolve(env, resolving, lhsgate);
            lhsv >> howmuch
        }
        MaybeResolved::Value(Value::Const(c)) => {
            c
        }
    };

    gate.value = MaybeResolved::Resolved(result);
    resolving.remove(&gate.name);

    return result;
}

fn main() {
    let mut env = HashMap::new();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let gate = parse_line(line.unwrap().trim());
        env.insert(gate.name, gate);
    }

    let mut resolving = HashSet::new();
    for gate in env.values() {
        resolve(&mut env, &mut resolving, &mut gate);
    }

    for (gatename, gate) in env.iter() {
        if let MaybeResolved::Resolved(v) = gate.value {
            println!("{} is {}", gatename, v);
        }
        else {
            panic!();
        }
    }
}
