use std::fmt::Display;
use crate::helpers::Point;
use owo_colors::{OwoColorize, Style};

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
            if *cell_if { print!("{}", cell); } else { print!("."); }
        }
        println!();
    }
}

/// Print a 1D Vector in the shape of a grid of any type that implements std::fmt::Display
pub fn print_grid<T: Display>(grid: &Vec<T>, size: &Point) {
    for (i, value) in grid.iter().enumerate() {
        if i != 0 && i as i32 % size.x() == 0 { println!() }
        print!("{}", value);
    }
    println!()
}

/// Print a 1D Vector in the shape of a grid of any type that implements std::fmt::Display
/// Depending on the function passed in, if the function evaluates to False it prints '.'
pub fn print_grid_if<T: Display, F>(grid: &Vec<T>, size: &Point, func_if: F)
where
    F: Fn(&T) -> bool,
{
    for (i, value) in grid.iter().enumerate() {
        if i != 0 && i as i32 % size.x() == 0 { println!() }

        if func_if(value) {
            print!("{}", value);
        } else {
            print!(".");
        }
    }
    println!()
}

/// Print a 1D Vector in the shape of a grid of any type that implements std::fmt::Display
/// It prints the value with the given style
pub fn print_grid_color<T: Display, F>(grid: &Vec<T>, size: &Point, func_style: F)
where
    F: Fn(&T) -> Style,
{
    for (i, value) in grid.iter().enumerate() {
        if i != 0 && i as i32 % size.x() == 0 { println!() }

        print!("{}", value.style(func_style(value)))
    }
    println!()
}
