use crate::grid::Grid;
use crate::rng::Rng;
use crate::vector::{self, Vector};

use std::collections::HashSet;
use std::time::SystemTime;

#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub struct Racetrack {
    grid: Grid,
    path_vel_x: i32,
    path_pos: Vector,
    path_radius: i32,
    carving_ring: Box<[Vector]>,
    view_dist: i32,
    rng: Rng,
}

#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub struct RacetrackBuilder {
    view_dist: Option<i32>,
    path_radius: Option<i32>,
    seed: Option<u64>,
}

impl RacetrackBuilder {
    fn new() -> RacetrackBuilder {
        RacetrackBuilder {
            view_dist: None,
            path_radius: None,
            seed: None,
        }
    }

    pub fn view_dist(mut self, view_dist: i32) -> RacetrackBuilder {
        self.view_dist = Some(view_dist);
        self
    }

    pub fn path_radius(mut self, path_radius: i32) -> RacetrackBuilder {
        self.path_radius = Some(path_radius);
        self
    }

    pub fn seed(mut self, seed: u64) -> RacetrackBuilder {
        self.seed = Some(seed);
        self
    }

    pub fn build(self) -> Racetrack {
        Racetrack::from_builder(self)
    }
}

impl Racetrack {
    /// The version of the code. Things may work differently between versions.
    // This must be updated whenever RNG_VERSION is updated.
    pub const VERSION: u32 = 1;
    // RNG_VERSION
    const RNG_VERSION: u32 = 1;

    pub fn builder() -> RacetrackBuilder {
        RacetrackBuilder::new()
    }

    fn from_builder(builder: RacetrackBuilder) -> Racetrack {
        let view_dist = i32::abs(builder.view_dist.unwrap_or(20));
        let path_radius = i32::abs(builder.path_radius.unwrap_or(4));
        let seed = builder.seed.unwrap_or_else(|| {
            // Seed the RNG from the system time now.
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0)
        });
        let view_width = (view_dist * 2 + 1) as usize;
        Racetrack {
            grid: Grid::new(view_width, view_width),
            path_vel_x: 0,
            path_pos: Vector::ORIGIN,
            path_radius,
            carving_ring: make_ring(path_radius).into_boxed_slice(),
            view_dist,
            rng: Rng::with_seed(seed),
        }
    }

    pub fn translate(&mut self, dpos: Vector) {
        // TODO: inefficient
        self.grid.clear();
        self.path_pos = self.path_pos - dpos;
        let limit = self.view_dist + self.path_radius;
        while self.path_pos.y >= -limit {
            self.move_path_south();
        }
        while self.path_pos.y <= limit {
            let from = self.path_pos;
            self.move_path_north();
            for center in from.segment_pts(self.path_pos) {
                if i32::abs(center.x) < self.view_dist + self.path_radius {
                    for &pt in self.carving_ring.iter() {
                        self.grid.v_get_mut(center + pt).map(|c| *c = true);
                    }
                }
            }
        }
    }

    fn move_path_north(&mut self) {
        self.path_pos.x += self.path_vel_x;
        self.path_vel_x += (self.rng.forward() / (Rng::RAND_MAX / 3)) as i32 - 1;
        self.path_pos.y += 1;
    }

    fn move_path_south(&mut self) {
        self.path_vel_x -= (self.rng.backward() / (Rng::RAND_MAX / 3)) as i32 - 1;
        self.path_pos.x -= self.path_vel_x;
        self.path_pos.y -= 1;
    }

    pub fn get(&self, pos: Vector) -> Option<bool> {
        self.grid.v_get(pos)
    }

    pub fn view_dist(&self) -> i32 {
        self.view_dist
    }

    pub fn stringify(&self, track: char, wall: char) -> String {
        self.grid.stringify(track, wall)
    }
}

fn make_ring(radius: i32) -> Vec<Vector> {
    let radius = i32::abs(radius);
    if radius >= 2 {
        let hole: HashSet<Vector> = vector::circle_pts(radius - 2).collect();
        vector::circle_pts(radius)
            .filter(|pt| !hole.contains(pt))
            .collect()
    } else {
        vector::circle_pts(radius).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_is_correct() {
        assert!(
            Racetrack::RNG_VERSION == Rng::VERSION,
            "Update Racetrack::{VERSION, RNG_VERSION}"
        );
    }
}
