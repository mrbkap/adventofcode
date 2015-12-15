extern crate regex;

use std::io;
use std::io::BufRead;
use std::collections::HashMap;
use regex::Regex;

struct Constraint {
    actor: String,
    effect: i32,
    actee: String,
}

fn table_score(edges: &Vec<Vec<i32>>, table: &[usize]) -> i32 {
    let mut score = 0;
    for i in 0..table.len() {
        let cur = table[i];
        let next = table[(i + 1) % table.len()];
        score += edges[cur][next] + edges[next][cur];
    }

    return score;
}

fn do_find_best(edges: &Vec<Vec<i32>>) -> i32 {
    let mut best = 0i32;
    let mut visited: Vec<usize> = Vec::new();

    for i in 0..edges.len() {
        visited.push(i);
        let maybe = find_best(edges, &mut visited);
        visited.pop();

        if maybe > best {
            best = maybe;
        }
    }
    return best;
}

fn find_best(edges: &Vec<Vec<i32>>, visited: &mut Vec<usize>) -> i32 {
    if visited.len() == edges.len() {
        return table_score(edges, &visited);
    }

    let mut best_so_far = 0;
    for idx in 0..edges.len() {
       if visited.iter().any(|idx2| idx == *idx2) { continue; }

       visited.push(idx);
       let better = find_best(edges, visited);
       visited.pop();
       if better > best_so_far {
           best_so_far = better;
       }
    }

    return best_so_far;
}

fn main() {
    let re = Regex::new(r"(?P<actor>\w+).*(?P<factor>gain|lose) (?P<effect>\d+).* (?P<actee>\w+)\.$").unwrap();
    let stdin = io::stdin();
    let consts: Vec<_> = stdin.lock().lines().map(|l| l.unwrap().trim().to_string()).filter(|l| !l.is_empty()).map(|l| {
        let caps = re.captures(&l).unwrap();
        let mut effect = caps.name("effect").unwrap().parse::<i32>().unwrap();
        if caps.name("factor").unwrap() == "lose" {
            effect *= -1;
        }

        return Constraint { actor: caps.name("actor").unwrap().to_string(),
                            effect: effect,
                            actee: caps.name("actee").unwrap().to_string(), }
    }).collect();

    let mut actors = HashMap::new();
    for ref i in &consts {
        let len = actors.len();
        actors.entry(&*i.actor).or_insert(len);
    }
    let len = actors.len();
    actors.insert("_me", len);

    let mut edges: Vec<Vec<i32>> =
        (0..actors.len()).map(|_| vec![0; actors.len()]).collect();
    for ref c in &consts {
        edges[*actors.get(&*c.actor).unwrap()][*actors.get(&*c.actee).unwrap()] = c.effect;
    }

    println!("best: {}", do_find_best(&edges));
}
