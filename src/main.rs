use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();

    // random from range
    dbg!(rng.gen_range(17..=23));
}
