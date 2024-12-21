use std::fmt::{Display, Formatter};
use std::ops::{Add, Rem, Sub};
use crate::helpers::Point;

#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq)]
pub struct PointU32 {
    x: u32,
    y: u32,
}

impl PointU32 {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
    pub fn from_index(index: usize, size: &PointU32) -> Self {
        Self::new((index % (size.x as usize)) as u32, (index / (size.x as usize)) as u32)
    }

    pub fn multiply(&self, factor: u32) -> Self {
        Self::new(self.x * factor, self.y * factor)
    }

    pub fn in_bounds(&self, size: &PointU32) -> bool {
        self.x < size.x && self.y < size.y
    }
    pub fn in_bounds_xy(&self, x_bound: usize, y_bound: usize) -> bool {
        self.x < x_bound as u32 && self.y < y_bound as u32
    }

    pub fn to_index(&self, size: &PointU32) -> usize {
        ((self.x) + (self.y) * (size.x)) as usize
    }

    pub fn try_index(&self, size: &PointU32) -> Option<usize> {
        if self.in_bounds(size) { Some(self.to_index(size)) } else { None }
    }

    pub fn get_neighbours(&self, size: &PointU32, diagonal: bool) -> Vec<PointU32> {
        let directions=  if diagonal {
            let mut temp = Vec::with_capacity(8);
            for x in [-1, 1] {
                for y in [-1, 1] {
                    if x == 0 && y == 0 { continue; }
                    temp.push(Point::new(x, y));
                }
            }
            temp
        } else {
            Vec::from([Point::new(-1, 0), Point::new(1, 0), Point::new(0, -1), Point::new(0, 1)])
        };

        let mut neighbours = Vec::new();
        for direction in directions {
            let new_point = self.checked_add_signed(&direction);
            // If it is inbounds and it is one step higher
            if new_point.is_some() && new_point.unwrap().in_bounds(size) {
                neighbours.push(new_point.unwrap());
            }
        }
        neighbours
    }

    pub fn checked_add(&self, other: &PointU32) -> Option<PointU32> {
        let x = self.x.checked_add(other.x)?;
        let y = self.y.checked_add(other.y)?;
        Some(PointU32::new(x, y))
    }
    pub fn checked_add_signed(&self, other: &Point) -> Option<PointU32> {
        let x = self.x.checked_add_signed(other.x())?;
        let y = self.y.checked_add_signed(other.y())?;
        Some(PointU32::new(x, y))
    }

    pub fn checked_add_bounded_xy(&self, other: &Point, bound_x: usize, bound_y: usize) -> Option<PointU32> {
        let x = self.x.checked_add_signed(other.x())?;
        let y = self.y.checked_add_signed(other.y())?;
        if x >= bound_x as u32 || y >= bound_y as u32 {return None;}
        Some(PointU32::new(x, y))
    }

    pub fn add_signed(&self, other: &Point) -> Self {
        let x = self.x as i32 + other.x();
        let y = self.y as i32 + other.y();
        PointU32::new(x as u32, y as u32)
    }

    pub fn x(&self) -> u32 {
        self.x
    }

    pub fn y(&self) -> u32 {
        self.y
    }

    pub fn x_usize(&self) -> usize {
        self.x as usize
    }

    pub fn y_usize(&self) -> usize {
        self.y as usize
    }

    pub fn modulo(&self, factor: u32) -> Self {
        Self::new(self.x % factor, self.y % factor)
    }
}

impl Sub for PointU32 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
impl Add for PointU32 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Rem for PointU32 {
    type Output = Self;
    fn rem(self, other: Self) -> Self::Output {
        Self {
            x: self.x % other.x,
            y: self.y % other.y,
        }
    }
}

impl Display for PointU32 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl From<(usize, usize)> for PointU32 {
    fn from(value: (usize, usize)) -> Self {
        Self::new(value.0 as u32, value.1 as u32)
    }
}
