/// ##### Parse grid 2D
/// Parses the input into a 2-dimensional vector of &str. <br>
/// **Params:**
/// - Input: &str, the input string.
/// - Delimiter: Option<&str>, the delimiter, if None it will be split on chars.<br>
///
/// **Returns:**
/// - 2D grid with grid\[y]\[x]
/// - Size with (x, y)
pub fn grid_2d(input: &str, delimiter: Option<&str>) -> (Vec<Vec<String>>, (usize, usize)) {
    let lines = input.lines();
    let mut result = Vec::with_capacity(lines.size_hint().0);
    let mut size = (0usize, 0usize);
    for line in lines {
        let row: Vec<String>;
        if let Some(del) = delimiter {
            row = line.split(del).map(|char| char.to_string()).collect();
        } else {
            row = line.chars().map(|char| char.to_string()).collect();
        }
        if size.0 == 0 { size.0 = row.len(); }
        result.push(row);
        size.1 += 1;
    }
    (result, size)
}

/// Extracts all the integers from a string to a i32 vector.<br>
/// Decimal numbers are not supported
pub fn extract_numbers(input: &str) -> Vec<i32> {
    let mut result = Vec::new();

    let mut current = 0;
    let mut is_negative = false;
    let mut modified = false;
    for char in input.chars() {
        if char == '-' {
            is_negative = true;
        } else if char.is_ascii_digit() {
            current = current * 10 + char.to_digit(10).unwrap() as i32;
            modified = true;
        } else if modified {
            result.push(current * (if is_negative { -1 } else { 1 }));
            current = 0;
            is_negative = false;
            modified = false;
        }
    }
    result.push(current * (if is_negative { -1 } else { 1 }));
    result
}

/// Extracts all the integers from a string to an u32 vector.<br>
/// Decimal numbers are not supported
pub fn extract_u32(input: &str) -> Vec<u32> {
    let mut result = Vec::new();

    let mut current = 0;
    let mut modified = false;
    for char in input.chars() {
       if char.is_ascii_digit() {
            current = current * 10 + char.to_digit(10).unwrap();
            modified = true;
        } else if modified {
            result.push(current);
            current = 0;
            modified = false;
        }
    }
    result.push(current);
    result
}
