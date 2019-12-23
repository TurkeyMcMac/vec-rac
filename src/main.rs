mod vector;
use vector::Vector;


mod rng;
use rng::Rng;

mod grid;
use grid::Grid;

mod racetrack;
use racetrack::Racetrack;

use std::cmp;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::iter;
use std::str::FromStr;

/*
#[derive(Clone, Debug)]
struct Racetrack {
    rng: Rng,
    forward: bool,
    path_vel_x: i32,
    path_pos: Vector,
    chunk_side: usize,
    chunks: [[Box<[bool]>; 3]; 3],
    carving_ring: Vec<Vector>,
}

#[derive(Clone, Debug)]
struct RacetrackBuilder {
    seed: Option<u64>,
    windiness: Option<i32>,
    path_radius: Option<i32>,
    view_radius: Option<i32>,
}

impl RacetrackBuilder {
    pub fn seed(mut self, seed: u64) -> RacetrackBuilder {
        self.seed = Some(seed);
        self
    }

    pub fn windiness(mut self, windiness: i32) -> RacetrackBuilder {
        self.windiness = Some(windiness);
        self
    }

    pub fn path_radius(mut self, path_radius: i32) -> RacetrackBuilder {
        self.path_radius = Some(path_radius);
        self
    }

    pub fn view_radius(mut self, radius: i32) -> RacetrackBuilder {
        self.view_radius = Some(radius);
        self
    }

    pub fn build(self) -> Racetrack {
        Racetrack::from_builder(self)
    }
}

impl Racetrack {
    pub fn builder() -> RacetrackBuilder {
        RacetrackBuilder {
            seed: None,
            windiness: None,
            path_radius: None,
            view_radius: None,
        }
    }

    fn from_builder(builder: RacetrackBuilder) -> Racetrack {
        let seed = builder.seed.unwrap_or(0);
        let windiness = builder.windiness.map(i32::abs).unwrap_or(1);
        let path_radius = builder.path_radius.map(i32::abs).unwrap_or(5);
        let view_radius = builder.view_radius.map(i32::abs).unwrap_or(3);
        let chunk_radius = cmp::max(path_radius, view_radius);
        let chunk_side = (chunk_radius as usize) * 2 + 1;
        let chunk = vec![false; chunk_side * chunk_side].into_boxed_slice();
        Racetrack {
            forward: true,
            rng: Rng::with_seed(seed),
            path_vel_x: 0,
            path_pos: Vector::ORIGIN,
            carving_ring: make_ring(path_radius),
            //max_dist: chunk_radius * 3 + 1,
            chunk_side,
            chunks: [
                [chunk.clone(), chunk.clone(), chunk.clone()],
                [chunk.clone(), chunk.clone(), chunk.clone()],
                [chunk.clone(), chunk.clone(), chunk.clone()],
            ],
        }
    }

    fn get_mut(&mut self, pos: Vector) -> Option<&mut bool> {
        //        if i32::abs(pos.x) <= self.max_dist && i32::abs(pos.y) <= max_dist {
        //        }
        None
    }

    fn carve_path(&mut self) {
        for &pt in &self.carving_ring {}
    }

    fn move_path_forward(&mut self) {
        self.path_vel_x += (self.rng.forward() / (Rng::RAND_MAX / 3)) as i32 - 1;
        self.path_pos.x += self.path_vel_x;
        self.path_pos.y += 1;
    }

    fn move_path_backward(&mut self) {
        self.path_vel_x -= (self.rng.backward() / (Rng::RAND_MAX / 3)) as i32 - 1;
        self.path_pos.x -= self.path_vel_x;
        self.path_pos.y -= 1;
    }
}
*/

fn make_ring(path_radius: i32) -> Vec<Vector> {
    let path_radius = i32::abs(path_radius);
    if path_radius >= 2 {
        let hole: HashSet<Vector> = vector::circle_pts(path_radius - 2).collect();
        vector::circle_pts(path_radius)
            .filter(|pt| !hole.contains(pt))
            .collect()
    } else {
        vector::circle_pts(path_radius).collect()
    }
}

fn main() {
    /*
    let mut circle = make_ring(4);
    let mut rng = Rng::with_seed(
    );
    let mut grid = Grid::new(35, 35);
    let mut pos = Vector::new(18, -23);
    let mut vel = 0;
    while pos.y < 20 {
        let last_pos = pos;
        pos.y += 1;
        pos.x += vel;
        vel += (rng.forward() / (Rng::RAND_MAX / 3)) as i32 - 1;
        last_pos
            .segment_pts(pos)
            .flat_map(|p1| circle.iter().map(move |&p2| p1 + p2))
            .for_each(|pt| {
                grid.v_get_mut(pt).map(|c| *c = true);
            });
    }
    print!("{}", grid.stringify('#', '.'));
    */
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
