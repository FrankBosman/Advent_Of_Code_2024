use std::fmt::Display;

/// ##### Print grid 2d
/// Print a 2D grid of any type that implements std::fmt::Display
pub fn print_grid_2d<T: Display>(grid: &Vec<Vec<T>>) {
    for row in grid {
        for cell in row {
            print!("{}", cell);
        }
        println!();
    }
}

/// ##### Print if grid 2d
/// Print a 2D grid depending on another boolean 2D grid
pub fn print_if_grid_2d<T: Display>(grid: &Vec<Vec<T>>, grid_if: &Vec<Vec<bool>>) {
    for (row, row_if) in grid.iter().zip(grid_if.iter()) {
        for (cell, cell_if) in row.into_iter().zip(row_if.iter()) {
            if *cell_if { print!("{}", cell); }
            else { print!("."); }
        }
        println!();
    }
}
