mod vector;
use vector::Vector;

mod rng;
use rng::Rng;

use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Clone, Copy)]
struct Tile {
    wall: bool,
}

#[derive(Clone, Copy)]
enum Direction {
    East,
    North,
    West,
    South,
}

#[derive(Clone, Copy)]
struct Connection {
    direction: Direction,
    pos: u32,
    angle: f64,
}

impl Connection {
    fn new() -> Connection {
        Connection {
            direction: Direction::East,
            pos: 0,
            angle: 0.0,
        }
    }
}

#[derive(Clone)]
struct Chunk {
    from: Connection,
    to: Connection,
    tiles: Box<[Tile]>,
}

impl Chunk {
    fn random(from: Connection, to: Direction) -> Chunk {
        Chunk::new(0)
    }

    fn draw(&self, width: usize) -> String {
        let trans = Vector::new(12, 15);
        let mut string = String::new();
        let mut grid = vec![vec![false; width]; width];
        for v in vector::circle_pts(10).map(|p| p + trans) {
            grid[v.x as usize][v.y as usize] = true;
        }
        for y in 0..width {
            for x in 0..width {
                string.push(if grid[x][y] { '#' } else { '.' });
            }
            string.push('\n');
        }
        string
    }
}

impl Chunk {
    fn new(chunk_size: usize) -> Chunk {
        let tiles = Vec::new();
        Chunk {
            from: Connection::new(),
            to: Connection::new(),
            tiles: tiles.into_boxed_slice(),
        }
    }
}

struct Racetrack {
    rng: Rng,
    chunk_size: usize,
    focus: Vector,
    max_angle: f64,
    chunks: [[Chunk; 5]; 5],
}

impl Racetrack {
    pub fn new(chunk_size: usize, seed: u64) -> Racetrack {
        let chunk = Chunk::new(chunk_size);
        let row = [
            chunk.clone(),
            chunk.clone(),
            chunk.clone(),
            chunk.clone(),
            chunk.clone(),
        ];
        Racetrack {
            rng: Rng::with_seed(seed),
            max_angle: 0.0,
            chunk_size,
            focus: Vector::ORIGIN,
            chunks: [
                row.clone(),
                row.clone(),
                row.clone(),
                row.clone(),
                row.clone(),
            ],
        }
    }

    pub fn move_focus(&mut self, delta: Vector) {}
}

fn main() {
    print!("{}", Chunk::new(0).draw(61));
}
