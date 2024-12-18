use std::collections::HashSet;
use advent_of_code::helpers::Point;
use heapq::PriorityQueue;

advent_of_code::solution!(18);

pub fn part_one(input: &str) -> Option<u32> {
    let (size, corrupted) = process_input(input, false);
    let corrupted = HashSet::from_iter(corrupted);
    a_star(&size, corrupted)
}

fn a_star(size: &Point, corrupted: HashSet<Point>) -> Option<u32> {
    let score_fn = |entry: &(u32, u32, Point)| u32::MAX - (entry.0 + entry.1);
    let mut queue = PriorityQueue::new(score_fn);
    let mut visited = HashSet::new();
    let bounds = Point::new(size.x() + 1, size.y() + 1);

    // at the start
    queue.push((0u32, 0u32, Point::new(0, 0)));

    while queue.peek().is_some() {
        // Get the item with the highest score
        let (_, path_len, pos) = queue.pop().unwrap();
        visited.insert(pos);

        // get neighbours
        let neighbours = pos.get_neighbours(&bounds, false)
            .into_iter().filter(|next| !corrupted.contains(next) && !visited.contains(next)).collect::<Vec<_>>();
        for next in neighbours {
            // Check if we are done
            if next.x() == size.x() && next.y() == size.y() {
                return Some(path_len + 1);
            }

            // Calculate score
            let manhattan = next.manhattan(size);
            let next_len = path_len + 1;
            queue.push((manhattan, next_len, next.clone()));
        }
    }

    None
}

pub fn part_two(input: &str) -> Option<String> {
    let (size, corrupted) = process_input(input, true);
    let amount = if input.lines().count() <= 25 {12} else {1024};

    for len in amount..corrupted.len() {
        let corrupted_set = HashSet::from_iter(corrupted[..len].iter().cloned());
        let result = a_star(&size, corrupted_set);
        if result.is_none() {
            let last_corrupt = corrupted[len - 1];
            return Some(format!("{},{}", last_corrupt.x(), last_corrupt.y()));
        }
    }
    None
}

fn process_input(input: &str, part2: bool) -> (Point, Vec<Point>) {
    let input_len = input.lines().count();
    let size = if input_len <= 25 {Point::new(6, 6)} else {Point::new(70, 70)};
    let amount = if input_len <= 25 {12} else {1024};

    let corrupted = get_corrupted(input, if part2 {input_len} else { amount });
    (size, corrupted)
}

fn get_corrupted(input: &str, amount: usize) -> Vec<Point> {
    let mut corrupted: Vec<Point> = Vec::with_capacity(amount);
    let mut lines = input.lines();
    for _ in 0..amount {
        let line = lines.next().unwrap();
        let (x, y) = line.trim().split_once(',').unwrap();
        corrupted.push(Point::new(x.parse().unwrap(), y.parse().unwrap()));
    }
    corrupted
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
