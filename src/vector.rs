use std::fmt;
use std::iter;
use std::mem;
use std::ops;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, Debug)]
pub struct Vector {
    pub x: i32,
    pub y: i32,
}

impl Vector {
    pub const ORIGIN: Vector = Vector { x: 0, y: 0 };

    pub fn new(x: i32, y: i32) -> Vector {
        Vector { x, y }
    }

    pub fn segment_pts(self, other: Vector) -> SegmentPts {
        let disp_x = (other.x - self.x) as f64;
        let disp_y = (other.y - self.y) as f64;
        let hypot = f64::hypot(disp_x, disp_y);
        SegmentPts {
            from: self,
            to: other,
            x: self.x as f64,
            y: self.y as f64,
            dx: disp_x / hypot,
            dy: disp_y / hypot,
            done: false,
        }
    }
}

impl ops::Add for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        Vector::new(self.x + other.x, self.y + other.y)
    }
}

impl ops::Sub for Vector {
    type Output = Vector;

    fn sub(self, other: Vector) -> Vector {
        Vector::new(self.x - other.x, self.y - other.y)
    }
}

impl ops::Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Vector {
        Vector::new(-self.x, -self.y)
    }
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Clone, PartialEq)]
pub struct SegmentPts {
    from: Vector,
    to: Vector,
    x: f64,
    y: f64,
    dx: f64,
    dy: f64,
    done: bool,
}

impl Iterator for SegmentPts {
    type Item = Vector;

    fn next(&mut self) -> Option<Vector> {
        if self.done {
            None
        } else if self.from == self.to {
            self.done = true;
            Some(self.from)
        } else {
            self.x += self.dx;
            self.y += self.dy;
            Some(mem::replace(
                &mut self.from,
                Vector {
                    x: (self.x + 0.5) as i32,
                    y: (self.y + 0.5) as i32,
                },
            ))
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.done {
            (0, Some(0))
        } else {
            (1, None)
        }
    }
}

impl iter::FusedIterator for SegmentPts {}

#[derive(Clone, PartialEq)]
pub struct CirclePts {
    pos: Vector,
    radius: i32,
    end_y: i32,
}

pub fn circle_pts(radius: i32) -> CirclePts {
    let abs_rad = i32::abs(radius);
    CirclePts {
        radius: abs_rad,
        pos: Vector::new(-abs_rad, -1),
        end_y: 0,
    }
}

impl Iterator for CirclePts {
    type Item = Vector;

    fn next(&mut self) -> Option<Vector> {
        if self.pos.x > self.radius {
            None
        } else if self.pos.y < self.end_y {
            self.pos.y += 1;
            Some(self.pos)
        } else {
            self.pos.x += 1;
            if self.pos.x <= self.radius {
                let r = self.radius as f64;
                let x = self.pos.x as f64;
                let y = f64::sqrt(r * r - x * x);
                self.end_y = y as i32;
                self.pos.y = -self.end_y;
                Some(self.pos)
            } else {
                None
            }
        }
    }
}

impl iter::FusedIterator for CirclePts {}
