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
