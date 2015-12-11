use std::io;
use std::io::BufRead;
use std::collections::HashMap;

struct Instruction {
    from: String,
    to: String,
    weight: u32,
}

fn do_find_shortest(edges: &Vec<Vec<u32>>) -> u32 {
    let mut best = u32::max_value();
    let mut visited: Vec<usize> = Vec::new();

    for i in 0..edges.len() {
        visited.push(i);
        let maybe = find_shortest_path(edges, &mut visited, 0);
        visited.pop();
        println!("maybe is {}", maybe);

        if maybe < best {
            best = maybe;
        }
    }
    return best;
}

fn find_shortest_path(edges: &Vec<Vec<u32>>, visited: &mut Vec<usize>, curr: u32) -> u32 {
    if visited.len() == edges.len() {
        return curr;
    }

    let last_idx = *visited.last().unwrap();
    let mut best_so_far = u32::max_value();
    for (idx, weight) in edges[last_idx].iter().enumerate() {
       if *weight == 0 || visited.iter().any(|idx2| idx == *idx2) {
           continue;
       }

       visited.push(idx);
       let better = find_shortest_path(edges, visited, *weight);
       visited.pop();
       if better < best_so_far {
           best_so_far = better;
       }
    }

    return best_so_far + curr;
}

fn main() {
    let stdin = io::stdin();
    let lines: Vec<_> =
        stdin.lock().lines().map(|l| l.unwrap().to_string()).filter(|ref l| !l.is_empty()).map(|l| {
            let arr: Vec<_> = l.split_whitespace().collect();
            Instruction {
                from: arr[0].to_string(),
                to: arr[2].to_string(),
                weight: arr[4].parse::<u32>().unwrap()
            }
        }).collect();

    let mut cities = HashMap::new();
    for line in &lines {
        let len = cities.len();
        cities.entry(&line.from).or_insert(len);
        let len = cities.len();
        cities.entry(&line.to).or_insert(len);
    }

    let mut edges: Vec<Vec<u32>> =
        (0..cities.len()).map(|_| vec![0; cities.len()]).collect();
    for line in &lines {
        let idx1 = *cities.get(&line.from).unwrap();
        let idx2 = *cities.get(&line.to).unwrap();
        edges[idx1][idx2] = line.weight;
        edges[idx2][idx1] = line.weight;
    }

    println!("{}", do_find_shortest(&edges));
}
