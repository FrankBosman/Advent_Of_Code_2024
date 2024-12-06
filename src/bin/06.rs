use std::collections::HashSet;

advent_of_code::solution!(6);
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}
impl Direction {
    fn turn_90(&self) -> Self {
        match self {
            Direction::UP => Direction::RIGHT,
            Direction::RIGHT => Direction::DOWN,
            Direction::DOWN => Direction::LEFT,
            Direction::LEFT => Direction::UP,
        }
    }
}
pub fn part_one(input: &str) -> Option<u32> {
    // Parse input
    let lines = input.lines();
    let size = (input.find("\n")?, input.lines().count());
    let map = lines.fold(Vec::new(), |mut acc, line| {
        acc.extend(line.trim().chars());
        acc
    });

    // Set init parameters
    let mut pos = map.iter().position(|char| '^'.eq(char))?;
    let mut direction = Direction::UP;
    let mut visited = HashSet::new();
    visited.insert(pos);

    // Simulate the steps of the guard
    loop {
        let result = step(pos, direction, &map, &size, None);
        if result.is_none() { break; }
        (pos, direction) = result.unwrap();
        visited.insert(pos);
    }

    // print_map(&map, &visited, &size);
    Some(visited.len() as u32)
}

fn step(pos: usize, direction: Direction, map: &Vec<char>, size: &(usize, usize), obstacle: Option<usize>) -> Option<(usize, Direction)> {
    // Check if next position is inbound, if not then we are done
    let next_opt = forward(pos, &direction, size);
    if next_opt.is_none() { return None; }
    let next_pos = next_opt.unwrap();

    // Test if next pos is blocked
    let next_blocked = '#'.eq(&map[next_pos]) || (obstacle.is_some() && obstacle.unwrap() == next_pos);
    if !next_blocked { return Some((next_pos, direction)); }  // Move forward

    // Turn 90 degree and try to step again
    step(pos, direction.turn_90(), map, size, obstacle)
}

pub fn part_two(input: &str) -> Option<u32> {
    // Parse the input
    let lines = input.lines();
    let size = (input.find("\n")?, input.lines().count());
    let map = lines.fold(Vec::new(), |mut acc, line| { acc.extend(line.trim().chars()); acc });

    // Get the init parameters
    let mut direction = Direction::UP;
    let mut pos = map.iter().position(|char| '^'.eq(char))?;
    let mut visited = HashSet::new();
    visited.insert((pos, direction));

    let mut count = 0;  // possible obj locations
    // Same loop as part 1, but now test the obj at every step
    loop {
        // Perform one step and stop if moved out of the map
        let result = step(pos, direction, &map, &size, None);
        if result.is_none() { break; }  // Moved out of the map

        // Try to place an obstacle here
        let (new_pos, new_direction) = result.unwrap();
        let obstacle = test_obstacle(new_pos, pos, direction, &visited,  &map, &size);
        if obstacle.is_some() { count += 1u32 }

        // Update the position, direction and save them.
        (pos, direction) = (new_pos, new_direction);
        visited.insert((pos, direction));
    }

    Some(count)
}

fn test_obstacle(obstacle: usize, start_pos: usize, direction: Direction, already_visited: &HashSet<(usize, Direction)>, map: &Vec<char>, size: &(usize, usize)) -> Option<usize> {
    let mut visited: HashSet<(usize, Direction)> = already_visited.clone();
    let mut pos = start_pos;
    let mut direction = direction;

    loop {

        let result = step(pos, direction, &map, &size, Some(obstacle));
        if result.is_none() { return None; }
        (pos, direction) = result.unwrap();

        if visited.contains(&(pos, direction)) { return Some(obstacle); }
        visited.insert((pos, direction));
    }
}

fn forward(pos: usize, direction: &Direction, size: &(usize, usize)) -> Option<usize> {
    let (x, y) = (pos % size.0, pos / size.0);
    match direction {
        Direction::UP => {
            if y <= 0 { None } else { Some(x + (y - 1) * size.0) }
        }
        Direction::DOWN => {
            if y + 1 >= size.1 { None } else { Some(x + (y + 1) * size.0) }
        }
        Direction::LEFT => {
            if x <= 0 { None } else { Some(x - 1 + y * size.0) }
        }
        Direction::RIGHT => {
            if x + 1 >= size.0 { None } else { Some(x + 1 + y * size.0) }
        }
    }
}


fn _print_map(map: &Vec<char>, visited: &HashSet<usize>, size: &(usize, usize)) {
    for i in 0..map.len() {
        if (i + 1) % size.0 == 0 { println!() }
        if visited.contains(&i) { print!("X"); } else { print!("{}", map[i]); }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
