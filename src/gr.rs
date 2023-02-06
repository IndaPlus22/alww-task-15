use rand::Rng;
pub fn make_massive_dipshit_file() {
    let elem = 100000;
    let com = 100000;
    println!("{} {}", elem, com);
    let mut rng = rand::thread_rng();
    let mut d = 1;
    for _ in 0..com {
        if d % 10000 == 0 {
            let a = 3;
            let b = rng.gen_range(1..=elem);
            println!("{} {}", a, b);
        } else {
            let a = rng.gen_range(1..=2);
            let b = rng.gen_range(1..=elem);
            let c = rng.gen_range(1..=elem);
            println!("{} {} {}", a, b, c);
        }
        d += 1;
    }
}
