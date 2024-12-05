use std::collections::{HashMap, HashSet};

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let (updates, before_map) = parse_input(input);

    // Validate all the strings
    let mut answer = 0u32;
    'main: for line in updates.lines() {
        let pages = line.split(",").collect::<Vec<&str>>();
        let mut past = HashSet::with_capacity(pages.len());

        // Go through every page and ensure that it is not in front of any page that should be behind it
        for &page in &pages {
            let has_to_before = before_map.get(&page.to_string());
            if has_to_before.is_some() && !has_to_before.unwrap().is_disjoint(&past) { continue 'main; }
            past.insert(page.to_string());
        }

        // Add the middle page to the counter
        let middle_page = pages.get(pages.len() / 2).unwrap();
        answer += middle_page.parse::<u32>().unwrap();
    }

    Some(answer)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (updates_part, before_map) = parse_input(input);

    // Map it to a vector which contains all the parsed pages
    let updates = updates_part.lines().map(|line| line.split(",").map(|str| str.to_string()).collect::<Vec<String>>()).collect::<Vec<Vec<String>>>();
    // Filter them and only keep the unordered pages
    let unordered = updates.iter().filter(|&pages| !correctly_ordered(&before_map, pages)).collect::<Vec<&Vec<String>>>();

    let mut answer = 0u32;
    for pages in unordered {
        // Create the order map, which counts all the occurrences that other pages are after it
        let pages_set: HashSet<String> = HashSet::from_iter(pages.to_vec());
        let middle = pages.len() / 2;
        let mut order_map = vec![0; pages.len()];
        for page in pages {
            // Count the amount of pages after this one, Way faster than using regex (was 1.6s)
            let page_set = before_map.get(&page.to_string());
            let mut count = 0;
            if page_set.is_some() { count = page_set.unwrap().intersection(&pages_set).count(); }
            order_map[count] = page.parse::<u32>().unwrap();
        }

        // Add the middle to the counter
        let result = order_map.get(middle);
        if let Some(value) = result {
            answer += value;
        }
    }

    Some(answer)
}

fn correctly_ordered(before_map: &HashMap<String, HashSet<String>>, pages: &Vec<String>) -> bool {
    let mut past = HashSet::with_capacity(pages.len());

    // Go through every page and ensure that it is not in front of any page that should be behind it
    for page in pages {
        let has_to_before = before_map.get(&page.to_string());
        if has_to_before.is_some() && !has_to_before.unwrap().is_disjoint(&past) { return false; }
        past.insert(page.to_string());
    }
    true
}

fn parse_input(input: &str) -> (&str, HashMap<String, HashSet<String>>) {
    // Split the data into two parts
    let (part1, part2) = input.split_once("\n\n").unwrap();

    // Create the order map
    let mut before_map: HashMap<String, HashSet<String>> = HashMap::new();
    for line in part1.lines() {
        let (before, after) = line.split_once("|").unwrap();

        // Way faster than one lining it with map.entry
        if !before_map.contains_key(before) { before_map.insert(before.to_string(), HashSet::new()); }
        before_map.get_mut(before).unwrap().insert(after.to_string());
    }
    (part2, before_map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
