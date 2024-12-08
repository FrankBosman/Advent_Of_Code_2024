use std::collections::{HashMap, HashSet};
use advent_of_code::helpers::Point;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let (size, antennas) = process_input(input);

    // Calculate interference
    let mut anti_nodes: HashSet<Point> = HashSet::new();
    for (_, value) in antennas.iter() {
        calculate_anti_nodes(value, &mut anti_nodes, &size);
    }
    Some(anti_nodes.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (size, antennas) = process_input(input);

    // Calculate interference
    let mut anti_nodes: HashSet<Point> = HashSet::new();
    for (_, value) in antennas.iter() {
        calculate_anti_nodes_part2(value, &mut anti_nodes, &size);
    }
    Some(anti_nodes.len() as u32)
}

fn calculate_anti_nodes(antennas: &Vec<Point>, anti_nodes: &mut HashSet<Point>, size: &Point) {
    for antenna in antennas.iter() {
        for nearby in antennas.iter() {
            if antenna.eq(nearby) { continue; }

            // calculate the anti-node point
            let distance = *antenna - *nearby;
            // eq: nearby + distance * z where z: [-1, 2]
            let anti_node1 = *nearby + distance.multiply(-1);
            let anti_node2 = *nearby + distance.multiply(2);
            if anti_node1.in_bounds(size) { anti_nodes.insert(anti_node1); }
            if anti_node2.in_bounds(size) { anti_nodes.insert(anti_node2); }
        }
    }
}

fn process_input(input: &str) -> (Point, HashMap<char, Vec<Point>>) {
    let (mut width, mut height) = (0, 0);
    let mut attennas: HashMap<char, Vec<Point>> = HashMap::new();
    for line in input.lines() {
        if width == 0 { width = line.len() as i32 }
        let chars = line.chars();

        for (x, char) in chars.enumerate() {
            if char != '.' {
                if !attennas.contains_key(&char) { attennas.insert(char, Vec::new()); }
                attennas.get_mut(&char).unwrap().push(Point::new(x as i32, height));
            }
        }

        height += 1;
    }

    (Point::new(width, height), attennas)
}

fn calculate_anti_nodes_part2(antennas: &Vec<Point>, anti_nodes: &mut HashSet<Point>, size: &Point) {
    for antenna in antennas.iter() {
        for nearby in antennas.iter() {
            if antenna.eq(nearby) { continue; }

            // calculate the anti-node point
            let distance = *antenna - *nearby;
            // eq: nearby + distance * z, First go down the line
            let mut z = 0;
            loop {
                // Calculate the new anti-node
                let anti_node = *nearby + distance.multiply(z);

                // If in bounds, add and continue. if not stop looking further
                if anti_node.in_bounds(size) { anti_nodes.insert(anti_node); }
                else { break; }
                z -= 1;
            }
            // Then go up
            z = 1;
            loop {
                let anti_node = *nearby + distance.multiply(z);
                if anti_node.in_bounds(size) { anti_nodes.insert(anti_node); }
                else { break; }
                z += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
