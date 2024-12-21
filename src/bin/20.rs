use advent_of_code::helpers::{parse_to, Point};
use owo_colors::OwoColorize;
use rayon::prelude::*;
use std::collections::HashMap;
use std::ops::{Add, Sub};

advent_of_code::solution!(20);

/// Solve part 1
pub fn part_one(input: &str) -> Option<u32> {
    // Parse the input
    let (racetrack, size) = parse_to::grid_2d(input, None);
    let size = Point::new(size.1 as i32, size.0 as i32);
    let sanitized = input.trim().replace("\n", "");
    let start = Point::from_index(sanitized.find('S').unwrap(), &size);

    // Calculate the length of the path of each position
    let path = walk_path(&start, &racetrack, &size);
    if racetrack.len() <= 15 { print_track(&racetrack, &size, &path); }

    // Count all shortcuts that are larger than the minimum shortcut
    let minimum_shortcut = if racetrack.len() <= 15 { 0 } else { 100 };
    let directions = [Point::new(-1, 0), Point::new(1, 0), Point::new(0, -1), Point::new(0, 1)];
    let mut shortcuts = HashMap::new();
    for (pos, path_len) in path.iter() {
        // Check every direction
        for direction in directions.iter() {
            let offset_1steps = pos.add(*direction);
            let offset_2steps = pos.add(direction.multiply(2));

            // If one step in the direction is a wall (not in path) and the second step is in the path
            if !path.contains_key(&offset_1steps) && path.contains_key(&offset_2steps) {
                let shortcut_length_option = path.get(&offset_2steps).unwrap().checked_sub(*path_len);
                if let Some(shortcut_length) = shortcut_length_option {
                    let shortcut = shortcut_length - 2;
                    if shortcut >= minimum_shortcut {
                        if !shortcuts.contains_key(&shortcut) { shortcuts.insert(shortcut, Vec::new()); }
                        shortcuts.get_mut(&shortcut).unwrap().push(pos);
                    }
                }
            }
        }
    }

    // Sum up the total
    let mut total = 0u32;
    for (_, points) in shortcuts {
        total += points.len() as u32;
    }

    Some(total)
}


/// Solve part 2
pub fn part_two(input: &str) -> Option<u32> {
    // Parse the input
    let (racetrack, size) = parse_to::grid_2d(input, None);
    let size = Point::new(size.1 as i32, size.0 as i32);
    let sanitized = input.trim().replace("\n", "");
    let start = Point::from_index(sanitized.find('S').unwrap(), &size);

    // Calculate the length of the path of each position
    let path = walk_path(&start, &racetrack, &size);
    if racetrack.len() <= 15 { print_track(&racetrack, &size, &path); }

    // Count all shortcuts that are larger than the minimum shortcut, in parallel
    let minimum_shortcut = if racetrack.len() <= 15 { 50 } else { 100 };
    let offsets = calc_offsets();  // calculate the offsets only once
    let total = path.par_iter().map(|(pos, path_len)| {
        cheat_20steps(pos, path_len, &racetrack, &path, &offsets, minimum_shortcut)
    }).sum();

    Some(total)
}

/// Goes through the path in the racetrack and keeps track of the path and the length of each node
fn walk_path(start: &Point, racetrack: &Vec<Vec<String>>, size: &Point) -> HashMap<Point, u32> {
    let mut path: HashMap<Point, u32> = HashMap::new();
    let mut next = Some(*start);
    let mut prev = *start;
    while next.is_some() {
        let current = next.unwrap();

        // Get all the neighbours and filter it so it only contains the next step
        let neighbours = current.get_neighbours(size, false).into_iter().filter(|pos| {
            pos.ne(&prev) && racetrack[pos.y_usize()][pos.x_usize()] != "#"
        }).collect::<Vec<Point>>();

        // There shouldn't be any junction in the track, spoiler there isn't.
        if neighbours.len() > 1 {
            println!("{:?}", path);
            panic!("Can't have more than one valid option, error at: {current}");
        }

        // Keep track of the previous, and add the new one to the path
        prev = current.clone();
        path.insert(current, path.len() as u32);

        // Look at the new one next
        let new = neighbours.first();
        if let Some(new) = new { next = Some(*new); } else { next = None; }
    }

    path
}

/// count all the viable cheats that save at least the minimum provided steps
fn cheat_20steps(pos: &Point, path_len: &u32, racetrack: &Vec<Vec<String>>, path: &HashMap<Point, u32>, offsets: &[Point; 840], min_len: u32) -> u32 {
    // Go through each offset, calculated before, and check if it is a viable cheat.
    let mut reachable = 0u32;
    for offset in offsets {
        // Get the end spot of this cheat
        let point = pos.add(*offset);

        // Ensure we don't end outside the map or on a wall
        let char = point.checked_get_2d(racetrack);
        if char.is_none() || char.unwrap() == "#" { continue; }

        // Get the path length of the tile where we end up after taking this cheat
        let offset_path_len = path.get(&point);
        if offset_path_len.is_none() { continue; }
        let offset_path_len = offset_path_len.unwrap();

        // Check if it is even usefully
        if offset_path_len <= path_len { continue; }
        let shortcut = offset_path_len.sub(path_len) - offset.x().abs() as u32 - offset.y().abs() as u32;

        // It has to be at least 50 or 100 steps faster to include it
        if shortcut >= min_len { reachable += 1; }
    }

    reachable
}

/// Calculate all offsets the 20 picoseconds cheat can lead to
fn calc_offsets() -> [Point; 840] {
    let mut offsets: [Point; 840] = [Point::new(0, 0); 840];
    let mut n = 0;
    for y in -20..=20i32 {
        for x in -20..=20i32 {
            if x.abs() + y.abs() <= 20 && !(x == 0 && y == 0) {
                offsets[n] = Point::new(x, y);
                n += 1;
            }
        }
    }
    // Check if the offset calculation works, spoiler it does.
    if n != 840 { panic!("Offsets calculation was wrong, n: {n}") }
    offsets
}

/// Prints the track and highlight the path length of each step
fn print_track(racetrack: &Vec<Vec<String>>, size: &Point, path: &HashMap<Point, u32>) {
    for y in 0..=size.y_usize() {
        for x in 0..=size.x_usize() {
            if y == 0 {
                if x != 0 { print!("{:^4}", x - 1); } else { print!("    "); }
                continue;
            } else if x == 0 {
                print!("{:^4}", y - 1);
                continue;
            }
            let (y, x) = (y - 1, x - 1);
            let el = &racetrack[y][x];
            let val = path.get(&Point::new(x as i32, y as i32));
            if let Some(val) = val {
                if el == "S" || el == "E" {
                    print!("{:^4}", val.bright_yellow().bold());
                } else {
                    print!("{:^4}", val.yellow());
                }
            } else {
                print!("{:^4}", "[==]".white());
            }
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(44));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(285));
    }
}
