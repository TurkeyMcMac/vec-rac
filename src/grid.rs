use crate::vector::Vector;

#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub struct Grid {
    width: usize,
    height: usize,
    v_off: Vector,
    grid: Vec<bool>,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Grid {
        Grid {
            width,
            height,
            v_off: Vector {
                x: (width as i32) / 2,
                y: (height as i32) / 2,
            },
            grid: vec![false; width * height],
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<bool> {
        if x < self.width && y < self.height {
            Some(self.grid[x + y * self.width])
        } else {
            None
        }
    }

    pub fn v_get(&self, pos: Vector) -> Option<bool> {
        let pos = pos + self.v_off;
        self.get(pos.x as usize, pos.y as usize)
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut bool> {
        if x < self.width && y < self.height {
            Some(&mut self.grid[x + y * self.width])
        } else {
            None
        }
    }

    pub fn v_get_mut(&mut self, pos: Vector) -> Option<&mut bool> {
        let pos = pos + self.v_off;
        self.get_mut(pos.x as usize, pos.y as usize)
    }

    pub fn clear(&mut self) {
        for cell in self.grid.iter_mut() {
            *cell = false;
        }
    }
}
