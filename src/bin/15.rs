use std::cmp::PartialEq;
use std::fmt::{Display, Formatter};
use owo_colors::OwoColorize;
use advent_of_code::helpers::{Point, PointU32, Direction};
use advent_of_code::helpers::print::print_grid_2d;

advent_of_code::solution!(15);

pub fn part_one(input: &str) -> Option<u32> {
    let (field_str, moves) = input.split_once("\n\n").unwrap();
    let (mut field, mut pos) = parse_field(field_str, false);
    let moves = moves.replace("\n", "");

    // Move the robot according to its programming
    for dir in moves.chars() {
        move_robot(&mut pos, Direction::new(dir), &mut field);
    }
    // print_grid_2d(&field);

    // Calculate the score
    let mut score = 0usize;
    for (y, row) in field.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if cell.eq(&Spot::Box) {
                score += y * 100usize + x;
            }
        }
    }

    Some(score as u32)
}

fn move_robot(pos: &mut PointU32, direction: Direction, field: &mut Vec<Vec<Spot>>) {
    let can_push = can_push(&pos, &direction, &field);
    if let Some(last_push) = can_push {
        field[pos.y_usize()][pos.x_usize()] = Spot::Empty;                      // Move the robot out of the current spot
        *pos = (&pos).checked_add_signed(&direction.to_direction()).unwrap();   // Move the robot
        field[last_push.y_usize()][last_push.x_usize()] = Spot::Box;            // Move the Crates over
        field[pos.y_usize()][pos.x_usize()] = Spot::Robot;                      // Move the robot in to the new spot
    }
}

fn can_push(pos: &PointU32, direction: &Direction, field: &Vec<Vec<Spot>>) -> Option<PointU32> {
    // Get the new position and ensure it is inbounds
    let new_pos = pos.checked_add_signed(&direction.to_direction())?;
    if !new_pos.in_bounds_xy(field[0].len(), field.len()) { return None; }

    // See what is on the next spot
    let new_obj = &field[new_pos.y_usize()][new_pos.x_usize()];
    if new_obj.eq(&Spot::Empty) { return Some(new_pos); }  // If it is empty then that is the final pos
    if new_obj.ne(&Spot::Box) { return None; }  // You can't push against a wall or other robot

    // If it is a box recursively check further
    can_push(&new_pos, direction, field)
}

// +----------+----------+ Part 2 +----------+----------+ \\
pub fn part_two(input: &str) -> Option<u32> {
    let (field_str, moves) = input.split_once("\n\n").unwrap();
    let (mut field, mut pos) = parse_field(field_str, true);
    let moves = moves.replace("\n", "");

    for dir in moves.chars() {
        move_robot_2(&mut pos, Direction::new(dir), &mut field);
    }

    // print_grid_2d(&field);

    // Count the score
    let mut score = 0usize;
    for (y, row) in field.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if cell.eq(&Spot::BoxLeft) {
                score += y * 100usize + x;
            }
        }
    }

    Some(score as u32)
}

fn move_robot_2(pos: &mut PointU32, direction: Direction, field: &mut Vec<Vec<Spot>>) {
    let can_push = push_obj_2(&pos, &direction, field);
    if can_push.is_some() {
        // Move every box when moving in the vertical direction
        for (left, right) in can_push.unwrap() {
            field[left.y_usize()][left.x_usize()] = Spot::Empty;
            field[right.y_usize()][right.x_usize()] = Spot::Empty;
            let (new_left, new_right) = (left.add_signed(&direction.to_direction()), right.add_signed(&direction.to_direction()));
            field[new_left.y_usize()][new_left.x_usize()] = Spot::BoxLeft;
            field[new_right.y_usize()][new_right.x_usize()] = Spot::BoxRight;
        }

        // Move the robot
        field[pos.y_usize()][pos.x_usize()] = Spot::Empty;                      // Move the robot out of the current spot
        *pos = (&pos).checked_add_signed(&direction.to_direction()).unwrap();   // Move the robot
        field[pos.y_usize()][pos.x_usize()] = Spot::Robot;                      // Move the robot in to the new spot
    }
}

fn push_obj_2(pos: &PointU32, direction: &Direction, field: &mut Vec<Vec<Spot>>) -> Option<Vec<(PointU32, PointU32)>> {
    // Get the new position and ensure it is inbounds
    let new_pos = pos.checked_add_bounded_xy(&direction.to_direction(), field[0].len(), field.len());
    if new_pos.is_none() { return None; }
    let new_pos = new_pos.unwrap();

    // See what is on the next spot
    let new_obj = &field[new_pos.y_usize()][new_pos.x_usize()];
    if new_obj.eq(&Spot::Empty) { return Some(Vec::new()); }  // Can move to an empty spot
    if new_obj.eq(&Spot::Wall) { return None; }  // You can't push against a wall
    if new_obj.eq(&Spot::Robot) { panic!("Hit a robot!!")}

    // We know we hit an object, now determine if we hit it horizontally or vertically
    if direction.is_horizontal() {
        // Move over to the other end of the box and test if we can push further
        let outer_pos = new_pos.add_signed(&direction.to_direction());
        if push_obj_2(&outer_pos, direction, field).is_some() {
            // Move over this box
            let new_outer_pos = outer_pos.checked_add_signed(&direction.to_direction()).unwrap();
            field[new_pos.y_usize()][new_pos.x_usize()] = Spot::Empty;
            let left = direction.eq(&Direction::Left);
            field[outer_pos.y_usize()][outer_pos.x_usize()] = if left { Spot::BoxRight } else { Spot::BoxLeft };
            field[new_outer_pos.y_usize()][new_outer_pos.x_usize()] = if left { Spot::BoxLeft } else { Spot::BoxRight };

            return Some(Vec::new());
        }
    } else {
        // Vertical movement, do not move them directly, because one branch could be unhindered whiles the other isn't
        let left;
        let right;
        if new_obj.eq(&Spot::BoxLeft) {
            left = new_pos;
            right = new_pos.add_signed(&Point::new(1, 0));
        } else {
            left = new_pos.add_signed(&Point::new(-1, 0));
            right = new_pos;
        }

        // Calculate both branches
        let left_result = push_obj_2(&left, direction, field);
        let right_result = push_obj_2(&right, direction, field);

        // If both are fine, then combine them
        if left_result.is_none() || right_result.is_none() { return None }
        let mut movable = left_result.unwrap();
        for val in right_result.unwrap() {
            if !movable.contains(&val) {
                movable.push(val);
            }
        }
        movable.push((left, right));

        return Some(movable);
    }
    None
}

fn parse_field(input: &str, part2: bool) -> (Vec<Vec<Spot>>, PointU32) {
    let mut field = Vec::with_capacity(input.lines().count());
    let mut robot_pos = PointU32::new(0, 0);

    for (y, line) in input.lines().enumerate() {
        let mut row = Vec::with_capacity(line.len());
        for (x, char) in line.chars().enumerate() {
            if part2 {
                let objs = Spot::from_2(char);
                if objs.contains(&Spot::Robot) { robot_pos = PointU32::new(x as u32 * 2, y as u32) }
                row.extend(objs);
            } else {
                let obj = Spot::from(char);
                if obj == Spot::Robot { robot_pos = PointU32::new(x as u32, y as u32) }
                row.push(obj)
            }
        }
        field.push(row);
    }

    (field, robot_pos)
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Spot {
    Robot,
    Box,
    Empty,
    Wall,
    BoxLeft,
    BoxRight,
}

impl Spot {
    fn from_2(c: char) -> [Spot; 2] {
        match c {
            '@' => [Spot::Robot, Spot::Empty],
            'O' => [Spot::BoxLeft, Spot::BoxRight],
            '.' => [Spot::Empty; 2],
            '#' => [Spot::Wall; 2],
            _ => panic!("invalid field type, {c}"),
        }
    }
}
impl From<char> for Spot {
    fn from(c: char) -> Self {
        match c {
            '@' => Spot::Robot,
            'O' => Spot::Box,
            '.' => Spot::Empty,
            '#' => Spot::Wall,
            _ => panic!("invalid field type, {c}"),
        }
    }
}
impl Display for Spot {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Spot::Robot => write!(f, "{}", "@".yellow()),
            Spot::Box => write!(f, "{}", "O".cyan()),
            Spot::Empty => write!(f, "{}", ".".white()),
            Spot::Wall => write!(f, "{}", "#".white()),
            Spot::BoxLeft => write!(f, "{}", "[".cyan()),
            Spot::BoxRight => write!(f, "{}", "]".cyan()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9021));
    }
}
