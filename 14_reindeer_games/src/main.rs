extern crate regex;

use std::io;
use std::io::BufRead;
use std::cmp::max;
use regex::Regex;

struct ReindeerInfo {
    speed: u32,
    range: u32,
    resttime: u32,
}

enum Status {
    Flying(u32, u32, u32),
    Resting(u32, u32),
}

struct Flier {
    status: Status,
    points: u32,
}

fn main() {
    let stdin = io::stdin();
    let re = Regex::new(r"(\d+) km.* (\d+) seconds.* (\d+) seconds").unwrap();
    let deer: Vec<ReindeerInfo> = stdin.lock().lines().map(|l| l.unwrap().trim().to_string()).filter(|l| !l.is_empty()).map(|l| {
        let caps = re.captures(&l).unwrap();
        return ReindeerInfo { speed: caps.at(1).unwrap().parse::<u32>().unwrap(),
                              range: caps.at(2).unwrap().parse::<u32>().unwrap(),
                              resttime: caps.at(3).unwrap().parse::<u32>().unwrap() };
    }).collect();

    let mut herd: Vec<Flier> = deer.iter().map(|d| Flier { status: Status::Flying(d.speed, d.range, 0), points: 0 }).collect();
    for _ in 1..2504 {
        let mut leaders: Vec<usize> = Vec::new();
        let mut furthest = 0u32;

        for (i, f) in herd.iter_mut().enumerate() {
            let mut tester: &mut Status = &mut f.status;
            match tester {
                &mut Status::Flying(speed, range, traveled) => {
                    let newpos = traveled + speed;
                    if range == 1 {
                        *tester = Status::Resting(deer[i].resttime, newpos);
                    } else {
                        *tester = Status::Flying(speed, range - 1, newpos);
                    }

                    if newpos > furthest {
                        leaders.truncate(0);
                        leaders.push(i);
                        furthest = newpos;
                    } else if newpos == furthest {
                        leaders.push(i);
                    }
                }
                &mut Status::Resting(timeleft, traveled) => {
                    if timeleft == 1 {
                        *tester = Status::Flying(deer[i].speed, deer[i].range, traveled);
                    } else {
                        *tester = Status::Resting(timeleft - 1, traveled);
                    }

                    if traveled > furthest {
                        leaders.truncate(0);
                        leaders.push(i);
                        furthest = traveled;
                    } else if traveled == furthest {
                        leaders.push(i);
                    }
                }
            }
        }

        for leader in &leaders {
            herd[*leader].points += 1;
        }
    }

    println!("Furthest {}", herd.iter().fold(0, |acc, d| max(acc, max(d.points, acc))));
}
