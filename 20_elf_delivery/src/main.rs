fn presents_for(minimum_presents: usize) -> usize {
    let div = minimum_presents / 11;
    let mut houses = vec![0; div];

    for elve in 1..div {
        let mut house_id = elve;
        let mut i = 0;
        while house_id < div && i < 50 {
            houses[house_id] += elve;
            house_id += elve;
            i += 1;
        }
    }

    houses.into_iter().position(|p| p >= div).unwrap()
}

fn main() {
    println!("{}", presents_for(36000000));
}
