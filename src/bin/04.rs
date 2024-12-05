use advent_of_code::helpers::parse_to;

advent_of_code::solution!(4);
const XMAS: [&str; 4] = ["X", "M", "A", "S"];

pub fn part_one(input: &str) -> Option<u32> {
    part_one_threads(input)
}
fn part_one_original(input: &str) -> Option<u32> {
    let (grid, size) = parse_to::grid_2d(input, None);

    // Setup checked grid
    let mut checked: Vec<Vec<bool>> = Vec::with_capacity(size.1);
    for _y in 0..size.1 {
        checked.push((0..size.0).map(|_| false).collect::<Vec<bool>>());
    }

    let mut count = 0u32;

    // Check all chars
    for y in 0..size.1 {
        for x in 0..size.0 {
            if checked[y][x] { continue; }

            // let val = &grid[y][x];
            let valid = check_xmas(x, y, &size, &grid);
            if !valid.is_empty() {
                count += ((valid.len() - 1) / (XMAS.len() - 1)) as u32;
                for (checked_x, checked_y) in valid {
                    checked[checked_y][checked_x] = true;
                }
            }
        }
    }

    // print::print_if_grid_2d(&grid, &checked);
    Some(count)
}

fn part_one_threads(input: &str) -> Option<u32> {
    let (grid, size) = parse_to::grid_2d(input, None);

    // Split the traverse of the grid into 2
    // using more than two thread is not a lot more efficient
    let mut answer = 0u32;
    let (part1, part2) = rayon::join(|| loop_grid(0, size.1/2, &size, &grid), || loop_grid(size.1/2, size.1, &size, &grid));
    answer += part1 + part2;

    Some(answer)
}

fn loop_grid(start: usize, end: usize, size: &(usize, usize), grid: &Vec<Vec<String>>) -> u32 {
    let mut count = 0u32;
    for y in start..end {
        for x in 0..size.0 {
            let valid = check_xmas(x, y, &size, &grid);
            if !valid.is_empty() {
                count += ((valid.len() - 1) / (XMAS.len() - 1)) as u32;
            }
        }
    }
    count
}

fn check_xmas(x: usize, y: usize, size: &(usize, usize), grid: &Vec<Vec<String>>) -> Vec<(usize, usize)> {
    // Retrieve teh value and verify if it is the start
    let val = &grid[y][x];
    if val.ne(XMAS[0]) { return vec![]; }

    // Check all directions
    let mut result = Vec::new();
    for direction in neighbours(x, y, size) {
        result.extend(check_continue(x, y, 1, direction, size, grid));
    }
    if !result.is_empty() { result.push((x, y)); }
    result
}

fn check_continue(x: usize, y: usize, index: usize, direction: (i32, i32), size: &(usize, usize), grid: &Vec<Vec<String>>) -> Vec<(usize, usize)> {
    // Calculate the next position and verify it
    let (next_x, next_y) = (x as i32 + direction.0, y as i32 + direction.1);
    if !is_bound(next_x, next_y, size) { return vec![]; }
    let (next_x, next_y) = (next_x as usize, next_y as usize);

    // Get the next value and verify it
    let next = &grid[next_y][next_x];
    if next.ne(XMAS[index]) { return vec![]; }

    // Found Successfully last part of the word
    if index + 1 >= XMAS.len() { return vec![(x, y)]; }

    // Continue in the same direction
    let mut result = check_continue(next_x, next_y, index + 1, direction, size, grid);
    if !result.is_empty() { result.push((x, y)); }
    result
}

pub fn part_two(input: &str) -> Option<u32> {
    // Transform to a 2D grid
    let (grid, size) = parse_to::grid_2d(input, None);

    // Loop and  M S
    // Find all   A
    //  X-MAS    M S
    let mut count = 0u32;
    for y in 0..size.1 {
        for x in 0..size.0 {
            // Continue if it isn't an A
            if grid[y][x].ne("A") || x < 1 || x >= size.0 - 1 || y < 1 || y >= size.1 - 1 { continue; }

            // Check if the top left is an S or M and not the same the positive diagonal
            let (top_left, bottom_right) = (&grid[y - 1][x - 1], &grid[y + 1][x + 1]);
            if top_left.eq(bottom_right) || !"MS".contains(bottom_right) || !"MS".contains(top_left) { continue; }
            // Check if the top left is an S or M and not the same in the negative diagonal
            let (bottom_left, top_right) = (&grid[y + 1][x - 1], &grid[y - 1][x + 1]);
            if bottom_left.eq(top_right) || !"MS".contains(bottom_left) || !"MS".contains(top_right) { continue; }
            count += 1;
        }
    }

    Some(count)
}

fn neighbours(x: usize, y: usize, size: &(usize, usize)) -> Vec<(i32, i32)> {
    let mut neighbours: Vec<(i32, i32)> = Vec::new();
    for y_offset in -1..=1 {
        for x_offset in -1..=1 {
            // Don't take a direction if the later letters are out of bound
            let (new_y, new_x) = (y as i32 + y_offset * (XMAS.len() - 1) as i32, x as i32 + x_offset * (XMAS.len() - 1) as i32);
            if is_bound(new_x, new_y, size) {
                neighbours.push((x_offset, y_offset));
            }
        }
    }
    neighbours
}

fn is_bound(x: i32, y: i32, size: &(usize, usize)) -> bool {
    y >= 0 && y < size.1 as i32 && x >= 0 && x < size.0 as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
