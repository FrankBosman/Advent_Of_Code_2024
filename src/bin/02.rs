advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let reports = parse_input(input);

    let mut safe = 0u32;
    for report in reports {
        if is_safe(report, false) { safe += 1; }
    }
    Some(safe)
}


pub fn part_two(input: &str) -> Option<u32> {
    let reports = parse_input(input);

    let mut safe = 0u32;
    for report in reports {
        if is_safe(report, true) { safe += 1; }
    }
    Some(safe)
}

fn is_safe(report: Vec<i32>, safety_damper: bool) -> bool {
    // Determine if the array will be increasing or decreasing
    let increasing = (report.get(0).unwrap() - report.get(1).unwrap()) < 0;

    // Loop over 2 levels at once using a sliding window
    let mut index = 0;  // keep track of the current index, for the safety damper
    for window in report.windows(2) {
        // Determine the difference and if it is ascending or descending
        let increase_window = (window[0] - window[1]) < 0;
        let differ = window[0].abs_diff(window[1]);

        // Test if it is invalid
        if increase_window != increasing || differ < 1 || differ > 3 {

            // If it is invalid and the safety damper is still active
            if safety_damper {
                // Try to remove one level, only the first and the current are useful.
                for index in [0, index, index + 1] {
                    if is_safe(remove_index(&report, index), false) {
                        return true;
                    }
                }
            }
            return false;
        }
        index += 1;
    }

    true
}

// Split the input from a grid of numbers into a double vector
fn parse_input(input: &str) -> Vec<Vec<i32>> {
    let lines = input.trim().lines();
    let reports = lines.map(|line| line.split_whitespace().map(|str| str.parse::<i32>().unwrap())
        .collect::<Vec<_>>()).collect::<Vec<_>>();
    reports
}

// Clone and remove an index from an array
fn remove_index(report: &Vec<i32>, index: usize) -> Vec<i32> {
    let mut new_report = report.clone();
    new_report.remove(index);
    new_report
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
