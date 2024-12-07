use rayon::prelude::*;
advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let mut answer = 0u64;

    // Test every equation
    for line in input.lines() {
        let result = process_line(line);
        if result.is_some() {
            answer += result.unwrap();
        }
    }
    Some(answer)
}

fn process_line(line: &str) -> Option<u64> {
    // Parse the data
    let (test_val, equation) = line.split_once(": ").unwrap();
    let test_val = test_val.parse::<u64>().ok().unwrap();
    let equation = equation.trim().split_whitespace().map(|val| val.parse::<u64>().unwrap()).collect::<Vec<_>>();

    // Validate the equation, Loop over every possibility
    for x in 0..2u32.pow((equation.len() - 1) as u32) {
        // Evaluate the equation
        let mut result = *equation.get(0).unwrap();
        for (i, value) in equation.iter().enumerate().skip(1) {
            // Use the bits of the current cycle to determine the operators
            let operator = (x >> (i - 1)) & 1;
            match operator {
                0 => result += value,
                1 => result *= value,
                _ => panic!("Invalid operator {}", operator)
            }
        }

        // If it is correct directly return the result
        if test_val.eq(&result) {
            return Some(result);
        }
    }

    None
}

pub fn part_two(input: &str) -> Option<u64> {
    // Part 1, but keep track of the lines that fail.
    let mut failed_1 = Vec::new();
    let mut answer = 0u64;
    for line in input.lines() {
        let result = process_line(line);
        if result.is_some() {
            answer += result.unwrap();
        } else {
            failed_1.push(line);
        }
    }

    // Part 2, three operators. but then in parallel
    answer += failed_1.par_iter()
        .fold(|| 0u64, |acc, line| acc + process_line_part2(line).unwrap_or_default())
        .sum::<u64>();

    Some(answer)
}

fn process_line_part2(line: &str) -> Option<u64> {
    // Parse the data
    let (test_val, equation) = line.split_once(": ").unwrap();
    let test_val = test_val.parse::<u64>().ok().unwrap();
    let equation = equation.trim().split_whitespace().map(|val| val.parse::<u64>().unwrap()).collect::<Vec<_>>();

    // Validate the equation, Loop over every possibility
    for step in 0..3u32.pow((equation.len() - 1) as u32) {
        // Evaluate the equation
        let mut x = step;
        let mut result = *equation.get(0).unwrap();
        for (_i, value) in equation.iter().enumerate().skip(1) {
            // Transform the cycle step into base three and use it as the operators, one at a time
            let m = x % 3;
            x = x / 3;
            let operator = std::char::from_digit(m, 3).unwrap();

            // Use the operator to update the result
            match operator {
                '0' => result += value,
                '1' => result *= value,
                '2' => {
                    // Concat ||
                    // Multiply the left side by the magnitude of the right, faster than using log.
                    let mut pow: u64 = 10;
                    while pow.lt(value) { pow *= 10; }  // Determine the magnitude
                    result = result * pow + value;      // Concat them
                }
                _ => panic!("Invalid operator {}", operator)
            }
        }

        // If it is correct directly return the result
        if test_val.eq(&result) {
            return Some(result);
        }
    }

    None
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
