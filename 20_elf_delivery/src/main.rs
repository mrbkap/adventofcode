fn presents_for(h: u32) -> u32 {
    let mut ps = 0u32;
    let target = ((h as f64).sqrt() as u32) + 1;
    for i in 1..(target + 1) {
        if h % i == 0 {
            ps += i;
            ps += h / i;
        }
    }

    return ps * 10;
}

fn main() {
    let mut h = 1;
    loop {
        let ps = presents_for(h);
        println!("House {} received {} presents", h, ps);
        if ps >= 36000000 {
            println!("House {} received {} presents", h, ps);
            break;
        }
        h += 1;
    }
}
