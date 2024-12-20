use std::collections::HashSet;
use std::ops::SubAssign;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<usize> {
    // Only look at uneven string, because even string would just have an empty space at the back
    let input = if input.len() % 2 == 0 { &input.trim()[..input.len() - 1] } else { input.trim() };
    let data = input.chars().map(|char| char.to_digit(10).unwrap()).collect::<Vec<_>>();

    let mut last_id = (input.len() - 1) / 2;
    let mut end_index = data.len() - 1;
    let mut last_size = data[end_index];

    // let mut result = Vec::new();
    let mut answer = 0usize;
    let mut pos = 0usize;
    for (i, block) in data.iter().enumerate() {
        // Test if we reached the end, Do not fill the last empty space
        if i >= end_index - 1 {
            // Add the remaining block
            // result.extend(vec![last_id; last_size as usize]);
            let size = last_size as usize;
            answer += (size * (pos * 2 + (size - 1)) / 2) * last_id;
            break;
        }
        // Filled block
        if i % 2 == 0 {
            let id = i / 2;
            // result.extend(vec![id; *block as usize]);

            let size = *block as usize;
            answer += (size * (pos * 2 + (size - 1)) / 2) * id;
        }
        // Block of empty space
        else if block > &0 {
            // Fill the empty spot
            let filler = get_filler(block, &mut last_id, &mut end_index, &mut last_size, &data);

            for (i, id) in filler.iter().enumerate() {
                answer += (pos + i) * id;
            }
            // result.extend(filler);
        }
        pos += *block as usize;
    }

    // let answer = result.iter().enumerate().fold(0, |acc, (i, val)| acc + i * val);
    Some(answer)
}

fn get_filler(size: &u32, last_id: &mut usize, last_index: &mut usize, last_size: &mut u32, data: &Vec<u32>) -> Vec<usize> {
    // Create a filler from the last item
    let last_size_immutable = last_size.to_owned();
    let filler_size = size.min(&last_size_immutable);
    let mut filler = vec![*last_id; *filler_size as usize];
    last_size.sub_assign(filler_size);
    let remainder = size - filler_size;

    // If the last item is empty move it over to the second last
    if 0u32.gt(last_size) { panic!("Last size should never be smaller than 0, it is: {last_size}") }
    if 0u32.eq(last_size) {
        last_index.sub_assign(2);  // Lower the index by 2
        last_id.sub_assign(1);     // Lower the id by 1
        *last_size = data[*last_index];  // Set the new size
    }

    // If the filler doesn't fill it completely, increase it recursively
    if remainder > 0 {
        filler.extend(get_filler(&remainder, last_id, last_index, last_size, data));
    }

    filler
}

pub fn part_two(input: &str) -> Option<usize> {
    /// A faster, but less straightforward approach, instead of looking at the filled blocks it will look at the empty spots
    /// First group everything into a readable disk-map containing the blocks and their sizes.
    /// Then go over each block,
    ///  - If it filled then directly calculate its score and add to the answer
    ///  - If it is empty then find filled blocks to fill it. and make sure we only look at a filled block once using a set.
    ///    Then, if the moved block is found later, thread it as an empty space.
    ///     (Nothing will ever be moved there so no need to merge with surrounding empty blocks)
    ///
    /// Left the commented result array in it which makes it easier to see how the code works. (\\\\)

    // parse the data into a vector containing the blocks with their id and sizes.
    let data = input.trim().chars().map(|char| char.to_digit(10).unwrap()).collect::<Vec<_>>();
    let disk_map = data.iter().enumerate().map(|(i, size)| {
        (
            if i % 2 == 0 { Some(i / 2) } else { None },
            *size as usize,
        )
    }).collect::<Vec<_>>();

    // Keep track of the processed items and the empty block sizes which can't be filled anymore
    let mut processed: HashSet<usize> = HashSet::new();
    let mut not_available_sizes = HashSet::new();

    //// let mut result = Vec::new();
    let mut answer = 0usize;
    let mut pos = 0usize;
    for (i, (id, size)) in disk_map.iter().enumerate() {

        // filled block
        if id.is_some() && !processed.contains(&id.unwrap()) {
            //// result.extend(vec![id.unwrap(); *size]);
            answer += (size * (pos * 2 + (size - 1)) / 2) * id.unwrap();
        }
        // empty block
        else if size > &0 {
            // Try to keep filling the void
            let mut remainder = *size;
            // Only continue if there is space left and something can fit in that space
            while remainder > 0 && !not_available_sizes.contains(&remainder) {
                let filler = get_filler_2(&remainder, i, &disk_map, &processed);
                // Fill the space if something fits
                if let Some((filler_id, filler_size)) = filler {
                    processed.insert(filler_id);                      // Note it as processed

                    //// result.extend(vec![filler_id; filler_size]);  // add it to the results
                    answer += (filler_size * ((pos+(*size - remainder)) * 2 + (filler_size - 1)) / 2) * filler_id;

                    remainder -= filler_size;
                }
                // If nothing fits in that space, remember it for later
                else {
                    not_available_sizes.insert(remainder);
                    break;
                }
            }

            //// Fill the remainder with zeros, not needed when directly calculating the answer
            //// if remainder > 0 {
            ////     result.extend(vec![0; remainder]);
            //// }
        }
        pos += size;
    }

    //// calculate the answer, not needed when directly calculating the answer
    //// let answer = result.iter().enumerate().fold(0, |acc, (i, val)| acc + i * val);
    Some(answer)
}

fn get_filler_2(size: &usize, index: usize, data: &Vec<(Option<usize>, usize)>, processed: &HashSet<usize>) -> Option<(usize, usize)> {
    for (i, (id, block_size)) in data.iter().enumerate().rev() {
        if i <= index { return None; }  // Only look to the right
        // Check if we are looking at a filled block, the size fits, and make sure to only process everything once
        if id.is_none() || block_size > size || processed.contains(&id.unwrap()) { continue; }
        return Some((id.unwrap(), *block_size));
    }
    None
}

pub fn naive_2(input: &str) -> Option<usize> {
    /// Slower but more straightforward approach to solve part 2.
    /// Converts everything to the vector.
    /// Then go over each filled block from end to start, try to find a spot where the filled block can move to.
    /// If has been found, move the block over and continue, just like the example on the site.

    let input = input.trim();
    // Put everything in a vector
    let disk_map = input.chars().map(|char| char.to_digit(10).unwrap()).collect::<Vec<_>>();
    let first_group_size = disk_map[0];
    let mut disk = Vec::with_capacity(disk_map.iter().sum::<u32>() as usize);
    for (i, block) in disk_map.iter().enumerate() {
        if i % 2 == 0 { disk.extend(vec![Some(i / 2); *block as usize]); } else { disk.extend(vec![None; *block as usize]); }
    }

    // Go from right to left to modify it
    let mut index = disk.len() - 1;
    loop {
        let group_len = get_group_len(&disk, index);
        let id = disk[index];
        if let Some(id) = id {
            let new_space = find_space(&disk, group_len, index);

            // Found new space, so move
            if let Some(new_space) = new_space {
                // Move over
                for i in new_space..(new_space + group_len) { disk[i] = Some(id); }
                // Clear old space
                for i in (index - group_len + 1)..=index { disk[i] = None; }
            }
        }

        // Move to the next group
        index -= group_len;
        if index <= first_group_size as usize { break; }
    }

    let answer = disk.iter().enumerate().fold(0, |acc, (i, val)| acc + i * val.unwrap_or_default());
    Some(answer)
}

fn find_space(disk: &Vec<Option<usize>>, size: usize, cur_index: usize) -> Option<usize> {
    let mut local_size = 0usize;
    for (i, id) in disk.iter().enumerate() {
        if id.is_some() {
            local_size = 0;
            continue;
        }
        if i >= cur_index { break; }
        local_size += 1;

        if local_size >= size { return Some(i - size + 1); }
    }
    None
}

fn get_group_len(disk: &Vec<Option<usize>>, index: usize) -> usize {
    let val = disk[index];
    let mut offset = 0;
    loop {
        offset += 1;
        if val.ne(&disk[index - offset]) { break; }
    }
    offset
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
