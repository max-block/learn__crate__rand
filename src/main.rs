use rand::{Rng, prelude::SliceRandom};

fn main() {
    let mut rng = rand::thread_rng();

    // random from range
    dbg!(rng.gen_range(17..=23));

    // random from vec
    let vec_data = vec!["a", "b", "c"];
    dbg!(vec_data.choose(&mut rng)); // SliceRandom
}
