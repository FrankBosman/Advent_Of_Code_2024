use std::fmt::{Display, Formatter};
use std::ops::{Add, Rem, Sub};

#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
    pub fn from_index(index: usize, size: &Point) -> Self {
        Self::new((index % (size.x as usize)) as i32, (index / (size.x as usize)) as i32)
    }

    pub fn multiply(&self, factor: i32) -> Self {
        Self::new(self.x * factor, self.y * factor)
    }

    pub fn in_bounds(&self, size: &Point) -> bool {
        self.x >= 0 && self.y >= 0 && self.x < size.x && self.y < size.y
    }

    pub fn to_index(&self, size: &Point) -> usize {
        (self.x as usize) + (self.y as usize) * (size.x as usize)
    }

    pub fn try_index(&self, size: &Point) -> Option<usize> {
        if self.in_bounds(size) { Some(self.to_index(size)) } else { None }
    }

    pub fn get_neighbours(&self, size: &Point, diagonal: bool) -> Vec<Point> {
        let directions = if diagonal {
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
            let new_point = self.add(direction);
            // If it is inbounds and it is one step higher
            if new_point.in_bounds(size) {
                neighbours.push(new_point);
            }
        }
        neighbours
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }

    pub fn x_usize(&self) -> usize {
        self.x as usize
    }

    pub fn y_usize(&self) -> usize {
        self.y as usize
    }

    pub fn modulo(&self, factor: i32) -> Self {
        Self::new(self.x % factor, self.y % factor)
    }

    pub fn manhattan(&self, size: &Point) -> u32 {
        self.x.abs_diff(size.x) + self.y.abs_diff(size.y)
    }

    pub fn checked_get_2d<'a, T>(&self, array_2d: &'a Vec<Vec<T>>) -> Option<&'a T> {
        let bounds = Self::new(array_2d[0].len() as i32, array_2d.len() as i32);
        if self.in_bounds(&bounds) { Some(&array_2d[self.y as usize][self.x as usize]) } else { None }
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

impl Rem for Point {
    type Output = Self;
    fn rem(self, other: Self) -> Self::Output {
        Self {
            x: self.x % other.x,
            y: self.y % other.y,
        }
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
