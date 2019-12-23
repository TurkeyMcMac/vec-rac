mod grid;
mod racetrack;
mod rng;
mod vector;

use vector::Vector;
use racetrack::Racetrack;
use std::env;
use std::io;
use std::str::FromStr;


fn main() {
    let mut builder = Racetrack::builder();
    if let Some(seed) = env::args().nth(1).and_then(|arg| u64::from_str(&arg).ok()) {
        builder = builder.seed(seed);
    }
    let mut rt = builder.view_dist(10).path_radius(4).build();
    let mut input = String::new();
    while io::stdin().read_line(&mut input).is_ok() {
        let dpos = match input.as_bytes().get(0) {
            Some(&b'w') => Vector::new(0, 1),
            Some(&b'a') => Vector::new(-1, 0),
            Some(&b's') => Vector::new(0, -1),
            Some(&b'd') => Vector::new(1, 0),
            Some(_) => Vector::ORIGIN,
            None => break,
        };
        rt.translate(dpos);
        print!("\n{}", rt.stringify('#', '.'));
        input.clear();
    }
}
