extern crate getopts;
extern crate rayon;

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
use std::iter;
use std::process;
use std::str::FromStr;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, SystemTime};
use vector::Vector;

fn options() -> Options {
    let mut opts = Options::new();
    opts.optflag("h", "help", "Print this help information.");
    opts.optflag("v", "version", "Print version information.");
    opts.optopt(
        "",
        "view-dist",
        "Set how far a racer can see in each cardinal direction. This is a positive integer. The default is 20. This cannot be more than display-dist.",
        "DISTANCE",
    );
    opts.optopt(
        "",
        "display-dist",
        "Set how far you can see in each cardinal direction. This is a positive integer. The default is 20. This cannot be less than view-dist.",
        "DISTANCE",
    );
    opts.optopt(
        "",
        "path-radius",
        "Set track path radius. This is a positive integer. The default is 4.",
        "RADIUS",
    );
    opts.optopt(
        "",
        "seed",
        "Set random seed to use. This is a positive integer. The default is decided randomly.",
        "SEED",
    );
    opts.optopt(
        "",
        "population",
        "Set genome population size. This is a positive integer. It may be rounded a bit. The default is 10.",
        "SIZE",
    );
    opts.optopt(
        "",
        "mutation",
        "Set mutation rate, a positive decimal. The default is 0.05.",
        "RATE",
    );
    opts
}

fn print_help(opts: &Options) -> String {
    let name = env::args().nth(0).unwrap_or("(anonymous)".to_string());
    format!(
        "{}\n\n{}\n",
        opts.short_usage(&name),
        opts.usage("Simulate vector racers.")
    )
}

fn main() {
    let opts = options();
    let matches = opts.parse(env::args()).unwrap_or_else(|err| {
        eprint!("{}\n\n{}", err, print_help(&opts));
        process::exit(1)
    });
    if matches.opt_present("help") {
        print!("{}", print_help(&opts));
        process::exit(0);
    } else if matches.opt_present("version") {
        println!("vec-rac version 0.3.0");
        process::exit(0);
    }
    let view_dist = matches
        .opt_str("view-dist")
        .and_then(|arg| i32::from_str(&arg).ok());
    let display_dist = matches
        .opt_str("display-dist")
        .and_then(|arg| i32::from_str(&arg).ok());
    let (view_dist, display_dist) = match (view_dist, display_dist) {
        (Some(v), Some(d)) => (v, i32::max(v, d)),
        (Some(v), None) => (v, v),
        (None, Some(d)) => (d, d),
        (None, None) => (20, 20),
    };
    let path_radius = matches
        .opt_str("path-radius")
        .and_then(|arg| i32::from_str(&arg).ok())
        .unwrap_or(4);
    let seed = matches
        .opt_str("seed")
        .and_then(|arg| u64::from_str(&arg).ok())
        .unwrap_or_else(|| {
            // Seed the RNG from the system time now.
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0)
        });
    let population = matches
        .opt_str("population")
        .and_then(|arg| usize::from_str(&arg).ok())
        .map(|pop| {
            if pop == 0 {
                2
            } else if pop & 1 == 1 {
                pop + 1
            } else {
                pop
            }
        })
        .unwrap_or(10);
    let mutation = matches
        .opt_str("mutation")
        .and_then(|arg| f64::from_str(&arg).ok())
        .unwrap_or(0.05);
    let mut rng = Rng::with_seed(seed + 17);
    let track_builder = Racetrack::builder().path_radius(path_radius).seed(seed);
    let track = track_builder.clone().view_dist(view_dist).build();
    let mut brains = iter::repeat_with(|| Brain::random(view_dist, &mut rng))
        .take(population)
        .collect::<Vec<_>>();
    let mut max_max_score = 0;
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let displayed_track = track_builder.view_dist(display_dist).build();
        for brain in rx {
            test_brain(&brain, &displayed_track, true);
        }
    });
    loop {
        let mut results = brains
            .par_iter()
            .map(|brain| (brain.clone(), test_brain(brain, &track, false)))
            .collect::<Vec<_>>();
        results.sort_by(|(_, (score_a, time_a)), (_, (score_b, time_b))| {
            score_b.cmp(&score_a).then(time_b.cmp(&time_a))
        });
        results.truncate(population / 2);
        let max_score = (results[0].1).0;
        if max_score > max_max_score {
            print!("\x07");
            max_max_score = max_score;
            tx.send(results[0].0.clone()).unwrap();
        }
        brains.clear();
        for (brain, _) in results.into_iter() {
            brains.push(brain.mutant(&mut rng, mutation));
            brains.push(brain);
        }
    }
}

fn clear_terminal() {
    print!("\x1b[H\x1b[J");
}

fn draw_track(track: &Racetrack) {
    let view_dist = track.view_dist();
    clear_terminal();
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

fn test_brain(brain: &Brain, track: &Racetrack, show: bool) -> (i32, usize) {
    let mut track = track.clone();
    let mut time = 0usize;
    let mut vel = Vector::ORIGIN;
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
        time = time.saturating_add(1);
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
    (pos.y, time)
}
