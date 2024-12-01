use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    // Parse the input
    let lines = input.trim().lines();
    let (mut list1, mut list2) = (Vec::new(), Vec::new());
    for line in lines {
        let line = line.trim();
        let (part1, part2) = line.split_once("   ").unwrap();
        list1.push(part1.parse::<u32>().unwrap());
        list2.push(part2.parse::<u32>().unwrap());
    }

    // Sort the lists
    list1.sort_unstable();
    list2.sort_unstable();

    // Sum up the differences
    let mut total = 0;
    for (part1, part2) in list1.iter().zip(list2.iter()) {
        total += part1.abs_diff(*part2);
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    // Parse the input
    let lines = input.trim().lines();
    let mut map1: HashMap<u32, u32> = HashMap::new();
    let mut map2: HashMap<u32, u32> = HashMap::new();
    for line in lines {
        let line = line.trim();
        let (part1, part2) = line.split_once("   ").unwrap();
        let (num1, num2) = (part1.parse::<u32>().unwrap(), part2.parse::<u32>().unwrap());

        *map1.entry(num1).or_default() += 1u32;
        *map2.entry(num2).or_default() += 1u32;
    }

    let mut total = 0;
    for (num, count) in map1.iter() {
        if let Some(count2) = map2.get(num) {
            total += num * count * count2;
        }
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
