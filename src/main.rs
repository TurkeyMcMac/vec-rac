extern crate rayon;
extern crate getopts;

mod brain;
mod grid;
mod racetrack;
mod rng;
mod vector;

use brain::Brain;
use getopts::Options;
use racetrack::Racetrack;
use rayon::prelude::*;
use rng::Rng;
use std::env;
use std::io;
use std::iter;
use std::str::FromStr;
use std::thread;
use std::time::{Duration, SystemTime};
use vector::Vector;

fn options() -> Options {
    let mut opts = Options::new();
    opts.optopt("", "view-dist", "Set track view square radius, a positive integer", "DISTANCE");
    opts.optopt("", "path-radius", "Set track path radius, a positive integer", "RADIUS");
    opts.optopt("", "seed", "Set random seed to use, a positive integer", "SEED");
    opts.optopt("", "population", "Set genome population size, a positive integer", "SIZE");
    opts.optopt("", "mutation", "Set mutation rate, a positive decimal", "RATE");
    opts.optflag("h", "help", "Print this help information");
    opts
}

fn main() {
    let opts = options();
    let matches = opts.parse(env::args()).unwrap();
    let view_dist = matches.opt_str("view-dist").and_then(|arg| i32::from_str(&arg).ok()).unwrap_or(20);
    let path_radius = matches.opt_str("path-radius").and_then(|arg| i32::from_str(&arg).ok()).unwrap_or(4);
    let seed = matches.opt_str("seed").and_then(|arg| u64::from_str(&arg).ok())
        .unwrap_or_else(|| {
            // Seed the RNG from the system time now.
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0)
        });
    let mut population = matches.opt_str("population").and_then(|arg| usize::from_str(&arg).ok()).unwrap_or(10);
    if population == 0 {
        population = 2;
    } else if population & 1 == 1 {
        population += 1;
    }
    let mutation = matches.opt_str("mutation").and_then(|arg| f64::from_str(&arg).ok()).unwrap_or(0.05);
    let mut rng = Rng::with_seed(seed + 17);
    let rt = Racetrack::builder()
        .view_dist(i32::max(20, view_dist))
        .path_radius(path_radius)
        .seed(seed)
        .build();
    let mut vel = Vector::ORIGIN;
    let mut brains = iter::repeat_with(|| Brain::random(view_dist, &mut rng))
        .take(population)
        .collect::<Vec<_>>();
    let mut max_max_score = 0;
    loop {
        let mut results = brains
            .par_iter()
            .map(|brain| (brain.clone(), test_brain(brain, &rt, false)))
            .collect::<Vec<_>>();
        results.sort_by(|a, b| b.1.cmp(&a.1));
        results.truncate(population / 2);
        let max_score = results[0].1;
        if max_score > max_max_score {
            print!("\x07");
            max_max_score = max_score;
            test_brain(&results[0].0, &rt, true);
        }
        brains.clear();
        for (brain, _) in results.into_iter() {
            brains.push(brain.mutant(&mut rng, mutation));
            brains.push(brain);
        }
    }
}

fn draw_track(track: &Racetrack) {
    let view_dist = track.view_dist();
    print!("\x1b[H\x1b[J");
    for y in (-view_dist..=view_dist).rev() {
        for x in -view_dist..=view_dist {
            let pos = Vector::new(x, y);
            let c = if pos == Vector::ORIGIN {
                '@'
            } else if let Some(true) = track.get(pos) {
                '.'
            } else {
                ' '
            };
            print!("{}", c);
        }
        print!("\n");
    }
}

fn test_brain(brain: &Brain, track: &Racetrack, show: bool) -> i32 {
    let mut track = track.clone();
    let mut vel = Vector::new(0, 1);
    let mut pos = Vector::ORIGIN;
    let mut max_score = 0;
    let mut since_improved = 0;
    track.translate(Vector::ORIGIN);
    'tick_loop: loop {
        vel = vel + brain.compute_accel(vel, &track);
        for pt in Vector::ORIGIN.segment_pts(vel) {
            if let Some(false) = track.get(pt) {
                if show {
                    track.translate(pt);
                } else {
                    pos = pos + pt;
                }
                break 'tick_loop;
            }
        }
        pos = pos + vel;
        if pos.y > max_score {
            max_score = pos.y;
            since_improved = 0;
        } else if since_improved > 50 {
            break 'tick_loop;
        } else {
            since_improved += 1;
        }
        track.translate(vel);
        if show {
            draw_track(&track);
            println!("score: {}  velocity: {}", pos.y, vel);
            thread::sleep(Duration::from_millis(50));
        }
        if let Some(false) = track.get(Vector::ORIGIN) {
            break 'tick_loop;
        }
    }
    if show {
        draw_track(&track);
        println!("max score: {}", pos.y);
        thread::sleep(Duration::from_millis(150));
    }
    pos.y
}
