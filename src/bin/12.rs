use advent_of_code::helpers::Point;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::ops::{Sub};

advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<u32> {
    let (grid, size) = process(input);

    // Go over each region and determine their size
    let mut processed: HashSet<usize> = HashSet::new();
    let mut price = 0u32;
    for (i, plant) in grid.iter().enumerate() {
        if processed.contains(&i) { continue; }

        let (region, perimeter, _) = process_region(plant, &grid, &size);

        processed.extend(&region);
        price += perimeter * region.len() as u32;
    }
    Some(price)
}

fn process_region(plant: &Plant, grid: &Vec<Plant>, size: &Point) -> (HashSet<usize>, u32, HashSet<(Point, Point)>) {
    // Sum the region area and perimeter using flood fill
    let mut visited = HashSet::new();
    let mut frontier = Vec::new();
    frontier.push(plant.index(size));

    let mut perimeter = 0u32;
    let mut fences: HashSet<(Point, Point)> = HashSet::new();  // Fences are defined as their cell and a direction

    while !frontier.is_empty() {
        let current = grid.get(frontier.pop().unwrap()).unwrap();
        let current_index = current.index(size);

        // Check if we already visited it and add it to the set
        if visited.contains(&current_index) { continue; }
        visited.insert(current_index);
        perimeter += current.perimeter();

        // Loop through al directions (needed for Part 2)
        let directions = [Point::new(-1, 0), Point::new(1, 0), Point::new(0, -1), Point::new(0, 1)];
        let connected = &current.connected;
        for direction in directions {
            let new_index = (direction + current.pos).try_index(size);  // Get the new direction
            if new_index.is_some() && connected.contains(&new_index.unwrap()) {      // Check if it is not out of bounds and of the same region
                // Connected, so no fence. Add them to explore later
                if visited.contains(&new_index.unwrap()) { continue; }
                frontier.push(new_index.unwrap())
            } else {
                // No connection or out of bounds, which means a fence
                fences.insert((current.pos, direction));
            }
        }
    }

    (visited, perimeter, fences)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (grid, size) = process(input);

    // Go over each region and determine their size
    let mut processed: HashSet<usize> = HashSet::new();
    let mut price = 0u32;
    for (i, plant) in grid.iter().enumerate() {
        if processed.contains(&i) { continue; }

        let (region, perimeter, fences) = process_region(plant, &grid, &size);
        let mut sides = perimeter;

        // Remove all the sides which have a fence next to it in the same direction
        for (plant, direction) in fences.iter() {
            // check if a plant to the left or up has a fence in the same direction
            if fences.contains(&(plant.sub(Point::new(-1, 0)), *direction)) ||
                fences.contains(&(plant.sub(Point::new(0, -1)), *direction)) {
                sides -= 1
            }
        }

        processed.extend(&region);
        price += sides * region.len() as u32;
    }
    Some(price)
}


fn process(input: &str) -> (Vec<Plant>, Point) {
    // Get the size of the grid
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let size = Point::new(width as i32, height as i32);

    // Parse and transform the input grid into a 1D vector
    let raw_grid: Vec<char> = input.lines().flat_map(|line| line.chars()).collect();
    let grid = raw_grid.iter().enumerate().map(|(i, char)| {
        Plant::new(i, char, &raw_grid, &size)
    }).collect::<Vec<_>>();

    (grid, size)
}

#[derive(Eq, Hash, PartialEq)]
struct Plant {
    pos: Point,
    plant: char,
    connected: Vec<usize>,
}

impl Plant {
    fn new(index: usize, plant: &char, grid: &Vec<char>, size: &Point) -> Self {
        let pos = Point::from_index(index, size);

        // Get and compare the neighbours
        let neighbours = pos.get_neighbours(size, false).iter().filter_map(|&point| {
            if plant == grid.get(point.to_index(size)).unwrap() {
                Some(point.to_index(size))
            } else { None }
        }).collect::<Vec<_>>();

        // Create the Plant
        Self { pos, plant: *plant, connected: neighbours }
    }
    fn index(&self, size: &Point) -> usize {
        self.pos.to_index(size)
    }

    fn perimeter(&self) -> u32 {
        4 - self.connected.len() as u32
    }
}

impl Display for Plant {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.plant)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
