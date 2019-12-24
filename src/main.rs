mod brain;
mod grid;
mod racetrack;
mod rng;
mod vector;

use brain::Brain;
use racetrack::Racetrack;
use rng::Rng;
use std::env;
use std::io;
use std::str::FromStr;
use std::thread;
use std::time::{Duration, SystemTime};
use vector::Vector;

fn main() {
    let seed = env::args()
        .nth(1)
        .and_then(|arg| u64::from_str(&arg).ok())
        .unwrap_or_else(|| {
            // Seed the RNG from the system time now.
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0)
        });
    let mut rng = Rng::with_seed(seed + 17);
    let mut rt = Racetrack::builder()
        .view_dist(20)
        .path_radius(4)
        .seed(seed)
        .build();
    let mut vel = Vector::ORIGIN;
    let brain = Brain::random(20, &mut rng);
    loop {
        vel = vel + brain.compute_accel(vel, &rt);
        rt.translate(vel);
        print!("\x1b[H\x1b[J{}", rt.stringify('#', '.'));
        println!("vel: {}", vel);
        thread::sleep(Duration::from_millis(100));
    }
}
