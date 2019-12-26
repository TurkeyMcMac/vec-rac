extern crate rayon;

mod brain;
mod grid;
mod racetrack;
mod rng;
mod vector;

use brain::Brain;
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
    let rt = Racetrack::builder()
        .view_dist(20)
        .path_radius(4)
        .seed(seed)
        .build();
    let mut vel = Vector::ORIGIN;
    let mut brains = iter::repeat_with(|| Brain::random(20, &mut rng))
        .take(10)
        .collect::<Vec<_>>();
    let mut max_max_score = 0;
    loop {
        let mut results = brains
            .par_iter()
            .map(|brain| (brain.clone(), test_brain(brain, &rt)))
            .collect::<Vec<_>>();
        results.sort_by(|a, b| b.1.cmp(&a.1));
        results.truncate(5);
        let max_score = results[0].1;
        if max_score > max_max_score {
            max_max_score = max_score;
            show_brain(&results[0].0, &rt);
            print!("\x1b[H\x1b[JMax score: {}", max_max_score);
        }
        brains.clear();
        for (brain, _) in results.into_iter() {
            brains.push(brain.mutant(&mut rng, 0.05));
            brains.push(brain);
        }
    }
}

fn test_brain(brain: &Brain, track: &Racetrack) -> i32 {
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
                pos = pos + pt;
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
        if let Some(false) = track.get(Vector::ORIGIN) {
            break 'tick_loop;
        }
    }
    pos.y
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

fn show_brain(brain: &Brain, track: &Racetrack) {
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
                track.translate(pt);
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
        draw_track(&track);
        println!("score: {}  velocity: {}", pos.y, vel);
        thread::sleep(Duration::from_millis(50));
        if let Some(false) = track.get(Vector::ORIGIN) {
            break 'tick_loop;
        }
    }
    draw_track(&track);
    println!("max score: {}", pos.y);
    thread::sleep(Duration::from_millis(150));
}
