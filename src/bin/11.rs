use std::collections::HashMap;
use std::sync::Mutex;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u64> {
    let solver = Day11::new();
    Some(solver.solve(input, 25))
}

pub fn part_two(input: &str) -> Option<u64> {
    let solver = Day11::new();
    Some(solver.solve(input, 75))
}

struct Day11 {
    memorized: Mutex<HashMap<(u64, u32), u64>>,
}
impl Day11 {
    fn new() -> Self {
        Self{ memorized: Mutex::new(HashMap::new())}
    }

    /// Simulates for the given steps
    fn solve(&self, input: &str, steps: u32) -> u64 {
        let mut answer = 0u64;
        for stone in input.split_whitespace() {
            let num = stone.parse::<u64>().unwrap();
            answer += self.simulate_stone(num, steps);
        }

        answer
    }

    fn simulate_stone(&self, num: u64, max_depth: u32) -> u64 {
        if max_depth == 0 { return 1; }

        // Check if we already processed it, memorization
        {
            let memorized = self.memorized.lock().unwrap();
            let checked = memorized.get(&(num, max_depth));
            if let Some(result) = checked {
                return *result;
            }
        }

        // If not follow the three rules like normal
        let result = match num {
            0 => self.simulate_stone(1, max_depth - 1),
            _ => {
                if let Some(factor) = get_factor(num) {
                    let (left, right) = (num / factor, num % factor);
                    self.simulate_stone(left, max_depth - 1) + self.simulate_stone(right, max_depth - 1)
                } else {
                    self.simulate_stone(num * 2024, max_depth - 1)
                }
            }
        };

        // Save the result for later
        let mut checked = self.memorized.lock().unwrap();
        checked.insert((num, max_depth), result.clone());
        result
    }
}


/// ##### Get factor
/// Calculate the factor to split the stones on.
/// If the magnitude is even it returns a factor, otherwise None is returned
fn get_factor(num: u64) -> Option<u64> {
    // The log aproach is around 2.3 micro seconds, but the loop is ~500-800 nano seconds
    // (num.ilog10() + 1) % 2 == 0

    // calculate the magnitude
    let mut magnitude = 1;
    while num >= 10u64.pow(magnitude) {
        magnitude += 1;
    }

    // return the factor if it has an even number of digits
    if magnitude % 2 == 0 { Some(10u64.pow(magnitude / 2)) } else { None }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
