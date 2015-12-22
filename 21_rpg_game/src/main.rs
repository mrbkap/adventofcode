#[derive(Clone, Copy, Debug)]
struct Player {
    health: i32,
    attack: i32,
    armor: i32,
}

#[derive(Debug)]
struct Item {
    cost: u32,
    armor: i32,
    attack: i32,
}

fn play_game(mut me: Player, mut boss: Player, items: &[&Item]) -> bool {
    let mut cur = &mut me;
    let mut other = &mut boss;
    let mut my_turn = true;

    for i in items {
        cur.attack += i.attack;
        cur.armor += i.armor;
    }


    loop {
        other.health -= std::cmp::max(1, cur.attack - other.armor);
        if other.health <= 0 {
            return my_turn;
        }

        my_turn = !my_turn;
        std::mem::swap(cur, other);
    }
}

fn first_star(me: &Player, boss: &Player, avail_weapon: &[Item], avail_armor: &[Item], avail_ring: &[Item]) {
    let mut min = u32::max_value();
    for w in avail_weapon.iter() {
        for a in avail_armor.iter() {
            for r in avail_ring.iter() {
                for r2 in avail_ring.iter() {
                    if r.cost > 0 && r2.cost > 0 && r.cost == r2.cost {
                        continue;
                    }
                    let cost = w.cost + a.cost + r.cost + r2.cost;
                    let won = play_game(*me, *boss, &[w, a, r, r2]);
                    if won && cost < min {
                        min = cost
                    }
                }
            }
        }
    }

    println!("{}", min);
}

fn second_star(me: &Player, boss: &Player, avail_weapon: &[Item], avail_armor: &[Item], avail_ring: &[Item]) {
    let mut max = 0;
    for w in avail_weapon.iter() {
        for a in avail_armor.iter() {
            for r in avail_ring.iter() {
                for r2 in avail_ring.iter() {
                    if r.cost > 0 && r2.cost > 0 && r.cost == r2.cost {
                        continue;
                    }
                    let cost = w.cost + a.cost + r.cost + r2.cost;
                    let won = play_game(*me, *boss, &[w, a, r, r2]);
                    if !won && cost > max {
                        max = cost
                    }
                }
            }
        }
    }

    println!("{}", max);
}

fn main() {
    let me = Player { health: 100, attack: 0, armor: 0 };
    let boss = Player { health: 100, attack: 8, armor: 2 };

    let avail_weapon = [
        Item { cost: 8, attack: 4, armor: 0 },
        Item { cost: 10, attack: 5, armor: 0 },
        Item { cost: 25, attack: 6, armor: 0 },
        Item { cost: 40, attack: 7, armor: 0 },
        Item { cost: 74, attack: 8, armor: 0 },
    ];
    let avail_armor = [
        Item { cost: 0, attack: 0, armor: 0 },
        Item { cost: 13, attack: 0, armor: 1 },
        Item { cost: 31, attack: 0, armor: 2 },
        Item { cost: 53, attack: 0, armor: 3 },
        Item { cost: 75, attack: 0, armor: 4 },
        Item { cost: 102, attack: 0, armor: 5 },
    ];
    let avail_ring = [
        Item { cost: 0, attack: 0, armor: 0 },
        Item { cost: 25, armor: 1, attack: 0 },
        Item { cost: 50, armor: 2, attack: 0 },
        Item { cost: 100, armor: 3, attack: 0 },
        Item { cost: 20, armor: 0, attack: 1 },
        Item { cost: 40, armor: 0, attack: 2 },
        Item { cost: 80, armor: 0, attack: 3 },
    ];

    first_star(&me, &boss, &avail_weapon, &avail_armor, &avail_ring);
    second_star(&me, &boss, &avail_weapon, &avail_armor, &avail_ring);
}
