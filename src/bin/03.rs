use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    // Create a regex that finds mult(num, num)
    let regex = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();

    // Process the slice, part1 was just one slice
    let answer = process_slice(input, &regex);
    Some(answer)
}

pub fn part_two(input: &str) -> Option<u32> {
    // Part 2 is made using a scanner approach, it adds everything it reads into a buffer and then checks if it contains do or don't

    // Create a regex that finds mult(num, num)
    let regex = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();

    let mut last_index = 0usize;
    let mut disabled = false;
    let mut answer = 0u32;

    for i in 4..input.len() {
        // get the buffer / last slice
        let slice = &input[last_index..i];

        // if it contains do() or don't(), process it
        if slice.contains("do()") {
            // Process the last slide depending on if it was disabled or not
            if disabled { disabled = false; }
            else {
                answer += process_slice(slice, &regex);
            }
            // update the lower bound of the buffer
            last_index = i;
        } else if slice.contains("don't()") {
            if !disabled {
                answer += process_slice(slice, &regex);
                disabled = true;
            }
            last_index = i;
        }
    }
    // Process the last remaining data if it isn't disabled
    if !disabled {
        answer += process_slice(&input[last_index..], &regex);
    }

    Some(answer)
}

fn process_slice(slice: &str, regex: &Regex) -> u32 {
    // Loop over all found instructions and sum them
    let mut sum = 0u32;
    for test in regex.find_iter(slice) {
        // Get the instruction and remove the mult(*) part
        let instruction = test.as_str();
        let instruction = instruction.replace("mul(", "");
        let instruction = instruction.replace(")", "");

        // Split into numbers, multiply them, and add to the sum
        let (part1, part2) = instruction.split_once(",").unwrap();
        sum += part1.parse::<u32>().unwrap() * part2.parse::<u32>().unwrap();
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
