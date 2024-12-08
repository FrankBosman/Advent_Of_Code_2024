use std::ops::{Add, Sub};

#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
    pub fn multiply(&self, factor: i32) -> Self {
        Self::new(self.x * factor, self.y * factor)
    }

    pub fn in_bounds(&self, size: &Point) -> bool {
        self.x >= 0 && self.y >= 0 && self.x < size.x && self.y < size.y
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
