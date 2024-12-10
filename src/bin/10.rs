use std::collections::HashSet;
use rayon::prelude::*;
use advent_of_code::helpers::Point;

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u32> {
    // Pre-process the data in to a vector of u8
    let (grid, size, starting_points) = process(input);

    // Go over all start points and calculate their score, sequentially
    // let mut answer = 0u32;
    // for point in starting_points {
    //     answer += calc_score(point, &grid, &size);
    // }

    // Sum up all the scores in parallel
    let answer = starting_points.par_iter().fold(|| 0u32, |acc, point| {
        acc + calc_score(*point, &grid, &size)
    }).sum::<u32>();

    Some(answer)
}

fn calc_score(point: Point, grid: &Vec<u32>, size: &Point) -> u32 {
    // A simple breath first searcher for part 1
    let mut visited = HashSet::<Point>::new();
    let mut frontier = Vec::<Point>::new();
    frontier.push(point);
    let mut result = 0u32;

    while !frontier.is_empty() {
        let current = frontier.pop().unwrap();

        // Don't revisit old places
        if visited.contains(&current) { continue; }
        visited.insert(current);

        // Check if we found the piek, height of 9.
        let value = grid.get(current.to_index(size)).unwrap();
        if value.eq(&9) { result += 1; }

        // Add neighbours to the frontier
        frontier.extend(get_neighbours(current, grid, size, Some(&visited)));
    }

    result
}

pub fn part_two(input: &str) -> Option<u32> {
    // Pre-process the data in to a vector of u8
    let (grid, size, starting_points) = process(input);

    // Go over all start points and calculate their score, sequentially.
    // let mut answer = 0u32;
    // for point in starting_points {
    //     answer += calc_score_part2(point, &grid, &size);
    // }

    // Sum up all the scores in parallel
    let answer = starting_points.par_iter().fold(|| 0u32, |acc, point| {
        acc + calc_score_part2(*point, &grid, &size)
    }).sum::<u32>();
    Some(answer)
}

fn calc_score_part2(point: Point, grid: &Vec<u32>, size: &Point) -> u32 {
    // A simple searcher which uses depth first search to solve part 2
    let mut result = 0u32;

    // Detect if we found the target
    let value = grid.get(point.to_index(size)).unwrap();
    if *value == 9 { return 1u32; }

    for neighbour in get_neighbours(point, grid, size, None) {
        result += calc_score_part2(neighbour, grid, size);
    }

    result
}


fn get_neighbours(point: Point, grid: &Vec<u32>, size: &Point, visited: Option<&HashSet<Point>>) -> Vec<Point> {
    // The directions of possible neighbours
    let directions = [Point::new(-1, 0), Point::new(1, 0), Point::new(0, -1), Point::new(0, 1)];

    let mut neighbours = Vec::new();
    let value = grid.get(point.to_index(size)).unwrap();

    for direction in directions {
        let new_point = point + direction;
        // If it is inbounds and it is one step higher
        if new_point.in_bounds(size) && *grid.get(new_point.to_index(size)).unwrap() == value + 1 {
            // Check if we should keep track of the visited
            if let Some(visited) = visited {
                // Only add it if we haven't been there yet
                if !visited.contains(&new_point) { neighbours.push(new_point) }
            }
            // If we do not have to keep track of the visited, then just add it
            else { neighbours.push(new_point); }
        }
    }

    neighbours
}

fn process(input: &str) -> (Vec<u32>, Point, Vec<Point>) {
    // Get the size of the grid
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let size = Point::new(width as i32, height as i32);

    // Parse and transform the input grid into a 1D vector
    let grid: Vec<_> = input.lines().flat_map(|l| l.chars().map(|char| char.to_digit(10).unwrap())).collect();

    // Get all the positions where the height is 0
    let starting_points = grid.iter().enumerate()
        .filter_map(|(index, value)| if value.eq(&0) { Some(Point::from_index(index, &size)) } else { None })
        .collect::<Vec<_>>();

    (grid, size, starting_points)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
