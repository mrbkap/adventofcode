use std::collections::HashSet;

#[derive(Clone, Copy, Debug)]
struct Player {
    health: i32,
    mana: i32,
    attack: i32,
}

#[derive(Clone)]
struct GameState {
    me: Player,
    boss: Player,
    effects: HashSet<Box<SpellCBs>, i32>,
}

trait SpellCBs {
    fn cast(&self, me: &mut Player, boss: &mut Player) -> i32;
    fn effect(&self, me: &mut Player, boss: &mut Player);
    fn cost(&self) -> i32;
}

#[derive(Clone)]
struct MagicMissile;

impl SpellCBs for MagicMissile {
    fn cast(&self, me: &mut Player, boss: &mut Player) -> i32 {
        boss.health -= 4;
        return 0;
    }

    fn effect(&self, me: &mut Player, boss: &mut Player) {
        // no-op
        panic!("not called");
    }

    fn cost(&self) -> i32 { 53 }
}

#[derive(Clone)]
struct Drain;

impl SpellCBs for Drain {
    fn cast(&self, me: &mut Player, boss: &mut Player) -> i32 {
        boss.health -= 2;
        me.health += 2;
        return 0;
    }

    fn effect(&self, me: &mut Player, boss: &mut Player) {
        // no-op
        panic!("not called");
    }

    fn cost(&self) -> i32 { 73 }
}

fn resolve_spells(state: &mut GameState) -> Option<i32> {
    for (s, r) in state.effects.iter_mut() {
        s.effect(&mut state.me, &mut state.boss);
        *r -= 1;
    }

    if state.boss.health <= 0 {
        return Some(0);
    }

    state.effects = state.effects.iter().filter(|(s, r)| r > 0).collect();
    return None;
}

fn step<Spells: SpellCBs>(mut state: GameState, i: usize, cost: usize, items: &[&Spells]) -> Option<i32> {
    if let Some(_) = resolve_spells(&mut state) {
        return Some(cost);
    }
    if items[i].cost() > state.me.mana {
        return None;
    }

    let efflen = items[i].cast(&mut state.me, &mut state.boss);
    if efflen > 0 {
        state.effects.push((items[i], efflen));
    }

    if state.boss.health <= 0 {
        return Some(cost + items[i].cost());
    }

    if let Some(_) = resolve_spells(&mut state) {
        return cost + items[i].cost;
    }

    if state.me.health -= state.boss.attack <= 0 {
        return None;
    }

    let best = i32::max_value();
    for j in 0..items.len() {
        if let Some(next) = step(state, j + 1, cost, items) {
            if next < best {
                best = next;
            }
        }
    }

    return if best != i32::max_value() { Some(best) } else { None };
}

fn main() {
    let initial = GameState { me: Player { health: 50, attack: 0, mana: 500 },
                              boss: Player { health: 55, attack: 8, mana: 0 },
                              effects: Vec::new(), };

    let spells: &[Box<SpellCBs>] = &[
        Box::new(MagicMissile),
        Box::new(Drain),
    ];

    let best = i32::max_value();
    for i in 0..spells.len() {
        if let Some(next) = step(initial, i, 0, spells) {
            if next < best {
                best = next;
            }
        }
    }

    println!("Best was {}", best);
}
