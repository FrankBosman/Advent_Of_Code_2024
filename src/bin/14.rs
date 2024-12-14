use advent_of_code::helpers::parse_to::extract_numbers;
use advent_of_code::helpers::Point;

advent_of_code::solution!(14);

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines().collect::<Vec<_>>();
    // The size is different for the real input and the example
    let size = if lines.len() <= 12 { Point::new(11, 7) } else { Point::new(101, 103) };
    let mut robots = lines.into_iter().map(|line| Robot::new(line)).collect::<Vec<_>>();

    // Take 100 steps forwards
    robots.iter_mut().for_each(|robot| robot.step(100, &size));

    // Sum the up the quadrants
    let mut quadrants = [0u32; 4];
    for robot in robots {
        if robot.position.y() < size.y() / 2 {
            if robot.position.x() < size.x() / 2 { quadrants[0] += 1; }
            if robot.position.x() > size.x() / 2 { quadrants[1] += 1; }
        }
        if robot.position.y() > size.y() / 2 {
            if robot.position.x() < size.x() / 2 { quadrants[2] += 1; }
            if robot.position.x() > size.x() / 2 { quadrants[3] += 1; }
        }
    }

    // Multiply together
    Some(quadrants.iter().fold(1, |acc, x| acc * x))
}

pub fn part_two(input: &str) -> Option<u32> {
    // Parse the input and create the robots
    let lines = input.lines().collect::<Vec<_>>();
    let size = if lines.len() <= 12 { Point::new(11, 7) } else { Point::new(101, 103) };
    let mut robots = lines.into_iter().map(|line| Robot::new(line)).collect::<Vec<_>>();

    // Loop until it finds the Christmas tree
    for i in 1..20000 {
        robots.iter_mut().for_each(|robot| robot.step(1, &size));
        if check_tree(&robots, &size) {
            // print_field(&robots, &size);
            return Some(i);
        }
    }
    None
}

/// Test if the robots from a Christmas tree, by searching for a ^
fn check_tree(robots: &Vec<Robot>, size: &Point) -> bool {
    // Setup field
    let mut field = vec![vec![0; size.x() as usize]; size.y() as usize];
    for robot in robots.iter() {
        field[robot.y()][robot.x()] += 1;
    }

    // Check for every robot if they are the top of the tree
    for robot in robots.iter() {
        // Position below it
        let left_below = robot.position + Point::new(-1, 1);
        let right_below = robot.position + Point::new(1, 1);

        // Check if they are inbound and the next position is a robot
        if !left_below.in_bounds(size) || !right_below.in_bounds(size) { continue; }
        if field[left_below.y() as usize][left_below.x() as usize] == 0 ||
            field[right_below.y() as usize][right_below.x() as usize] == 0 { continue; }

        // Recursively check the diagonal down
        if check_diagonal(left_below, -1, 4, &field, size) &&
            check_diagonal(right_below, 1, 4, &field, size) {
            return true;
        }
    }

    false
}

/// Recursively check if there exist a diagonal line down from the current point
fn check_diagonal(from: Point, direction: i32, remaining: u32, field: &Vec<Vec<i32>>, size: &Point) -> bool {
    if remaining == 0 { return true; }
    let new_point = from + Point::new(direction, 1);
    if !new_point.in_bounds(size) { return false };
    if field[new_point.y() as usize][new_point.x() as usize] == 0 { return false}

    check_diagonal(new_point, direction, remaining - 1, field, size)
}


#[derive(Debug)]
struct Robot {
    position: Point,
    velocity: Point,
}

impl Robot {
    pub fn new(line: &str) -> Self {
        let vars = extract_numbers(line);
        Self { position: Point::new(vars[0], vars[1]), velocity: Point::new(vars[2], vars[3]) }
    }

    fn step(&mut self, steps: i32, size: &Point) {
        // Move to the new spot and ensure it remains within the field
        let new_pos = self.position + self.velocity.multiply(steps);
        let mut new_pos = new_pos % *size;

        // If they are on the negative side, move them over
        if new_pos.x() < 0 { new_pos = new_pos + Point::new(size.x(), 0) }
        if new_pos.y() < 0 { new_pos = new_pos + Point::new(0, size.y()) }

        self.position = new_pos;
    }

    fn x(&self) -> usize { self.position.x() as usize }

    fn y(&self) -> usize { self.position.y() as usize }
}


fn _print_field(robots: &Vec<Robot>, size: &Point) {
    // build field
    let mut field = vec![vec![0; size.x() as usize]; size.y() as usize];
    for robot in robots.iter() {
        field[robot.y()][robot.x()] += 1;
    }

    // print field, Debug
    for y in 0..size.y() {
        for x in 0..size.x() {
            let count = field[y as usize][x as usize];
            let result = if count == 0 {String::from(".")} else if count > 9 {String::from("+")} else {count.to_string()};
            print!("{}", result);
        }
        println!();
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
