use std::collections::{HashMap, HashSet};
use advent_of_code::helpers::Point;
use heapq::PriorityQueue;
use rayon::prelude::*;

advent_of_code::solution!(18);

pub fn part_one(input: &str) -> Option<u32> {
    let (size, corrupted) = process_input(input, false);
    let corrupted = HashSet::from_iter(corrupted);

    // Return the result from a_star
    if let Some((path_len, _)) = a_star(&size, corrupted, 1) { Some(path_len) } else { None }
}

pub fn part_two(input: &str) -> Option<String> {
    let (size, corrupted) = process_input(input, true);
    let amount = if input.lines().count() <= 25 { 12 } else { 1024 };

    // Test every option in batches of ranges in parallel
    // This is faster than testing every path in parallel, because you can skip half of them
    let last_corrupted = (amount..corrupted.len()).into_par_iter().chunks(50).find_map_first(|range| {
        let mut path: Option<HashSet<Point>> = None;  // Keep track of the previous path

        // Test if every path exists, from the start amount till the max length
        for len in range {
            // Get the new set of corrupted positions for this test
            let new_corrupted = corrupted[len - 1];
            let corrupted_set = HashSet::from_iter(corrupted[..len].iter().cloned());

            // If the old path doesn't contain the new corrupted bit, we do not need to retest the path
            if let Some(ref path) = path {
                if !path.contains(&new_corrupted) { continue; }
            }

            // Retest the path whether there still exists one
            let result = a_star(&size, corrupted_set, 100);  // We do not need the optimal path, just a path
            if result.is_none() {
                // If not then we found the last corrupted bit of this batch
                return Some(new_corrupted);
            } else {
                // If there does exist a path, save it for the next cycle
                path = Some(result.unwrap().1);
            }
        }
        None  // All paths where valid
    });
    if let Some(corrupted) = last_corrupted { Some(format!("{},{}", corrupted.x(), corrupted.y())) } else { None }
}

/// ## A*
/// The A_star star path finding algorithm finds the most optimal path to the target.
/// But finding the most optimal path is slower than finding just a path,
/// so this function allows you to pass a greedy factor which will make it behave more like a greedy search.
/// This in turn speeds up the search, but it might not give the optimal answer
fn a_star(size: &Point, corrupted: HashSet<Point>, greedy_factor: u32) -> Option<(u32, HashSet<Point>)> {
    let score_fn = |entry: &(u32, u32, Point, _)| u32::MAX - (entry.0 * greedy_factor + entry.1);
    let mut queue = PriorityQueue::new(score_fn);
    let mut visited = HashSet::new();
    let mut connections: HashMap<Point, Point> = HashMap::new();  // child: &parent
    let bounds = Point::new(size.x() + 1, size.y() + 1);

    // add the start
    queue.push((0u32, 0u32, Point::new(0, 0), Point::new(0, 0)));

    while queue.peek().is_some() {
        // Get the item with the highest score
        let (_, path_len, pos, prev) = queue.pop().unwrap();

        // Ensure we didn't already visit them
        if visited.contains(&pos) { continue; }
        visited.insert(pos);
        connections.insert(pos, prev);  // Keep track of the paths

        // Check if we reached the target
        if pos.eq(size) {
            let path = generate_path(pos, &connections);
            return Some((path_len, path));
        }

        // get neighbours and filter them
        let neighbours = pos.get_neighbours(&bounds, false)
            .into_iter().filter(|next| !corrupted.contains(next) && !visited.contains(next)).collect::<Vec<_>>();

        // Add each valid neighbour
        for next in neighbours {
            // Calculate score and add them to the queue
            let manhattan = next.manhattan(size);
            queue.push((manhattan, path_len + 1, next.clone(), pos));
        }
    }

    None
}

/// Combine the collected connection into a path
fn generate_path(target: Point, connections: &HashMap<Point, Point>) -> HashSet<Point> {
    let mut path = HashSet::new();
    let mut current = &target;
    path.insert(current.clone());
    let start = Point::new(0, 0);

    while !start.eq(current) {
        current = &connections[current];
        path.insert(current.clone());
    }

    path
}

fn process_input(input: &str, part2: bool) -> (Point, Vec<Point>) {
    let input_len = input.lines().count();

    // The size and amount depends on whether it is the example or the real input
    let size = if input_len <= 25 { Point::new(6, 6) } else { Point::new(70, 70) };
    let amount = if input_len <= 25 { 12 } else { 1024 };
    // For part 2 load all the corrupted data
    let corrupted_amount = if part2 { input_len } else { amount };

    // Read the corrupted data for the given length
    let mut corrupted: Vec<Point> = Vec::with_capacity(corrupted_amount);
    let mut lines = input.lines();
    for _ in 0..corrupted_amount {
        let line = lines.next().unwrap();
        let (x, y) = line.trim().split_once(',').unwrap();
        corrupted.push(Point::new(x.parse().unwrap(), y.parse().unwrap()));
    }

    (size, corrupted)
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
        assert_eq!(result, Some("6,1".into()));
    }
}
