
struct Tile {
    wall: bool,
}

struct Chunk {
    east: Option<usize>,
    north: Option<usize>,
    west: Option<usize>,
    south: Option<usize>,
    tiles: Box<[Tile]>,
}

struct Racetrack {
    chunk_size: usize,
    focus: Vector,
    windiness: 
    chunks: [[Chunk; 3]; 3],
}

impl Racetrack {
    pub fn move_focus(&mut self, delta: Vector) {
        
    }


}
