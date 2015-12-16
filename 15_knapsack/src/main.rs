struct Ingrediant {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

fn main() {
    let ingrediants = vec![
        Ingrediant { capacity: 5, durability: -1, flavor: 0, texture: 0, calories: 5 },
        Ingrediant { capacity: -1, durability: 3, flavor: 0, texture: 0, calories: 1 },
        Ingrediant { capacity: 0, durability: -1, flavor: 4, texture: 0, calories: 6 },
        Ingrediant { capacity: -1, durability: 0, flavor: 0, texture: 2, calories: 8 },
    ];

    let mut max = 0u32;
    for s in 1..101 {
        for pb in 1..(101 - s) {
            for f in 1..(101 - s - pb) {
                let sug = 100 - s - pb - f;

                let amts = vec![s, pb, f, sug];
                let mut total = vec![0i32; 4];
                let mut calories = 0u32;
                for i in 0..4 {
                    total[0] += ingrediants[i].capacity * amts[i];
                    total[1] += ingrediants[i].durability * amts[i];
                    total[2] += ingrediants[i].flavor * amts[i];
                    total[3] += ingrediants[i].texture * amts[i];
                    calories += (ingrediants[i].calories * amts[i]) as u32;
                }

                if calories != 500 {
                    continue;
                }

                let ans = total.iter().fold(1, |acc, i| std::cmp::max(0, *i) * acc) as u32;
                if ans > max {
                    max = ans;
                }
            }
        }
    }

    println!("max {}", max);
}
