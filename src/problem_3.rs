use std::{error::Error, fs};

pub fn problem_3() -> Result<(), Box<dyn Error>> {
    let file_path = "./res/03/input";
    let contents = fs::read_to_string(file_path)?;
    let grid = create_engine_grid(contents);

    Ok(())
}

fn create_engine_grid(input: String) -> Vec<Vec<char>> {
    let lines = input.lines();
    let mut rows: Vec<Vec<char>> = Vec::new();
    for line in lines {
        let chars: Vec<char> = line.chars().collect();
        rows.push(chars);
    }

    rows
}

fn part_number_accumulator(grid: Vec<Vec<char>>) {
    if grid.len() <= 0 {
        panic!("Grid too short");
    }

    let mut part_number_sum = 0;

    let height = grid.len();
    let width = grid[0].len();

    for row in 0..height {
        for col in 0..width {
            part_number(row, col, height, width, grid);
        }
    }
}

fn part_number(
    row: usize,
    col: usize,
    grid_height: usize,
    grid_width: usize,
    grid: Vec<Vec<char>>,
) -> Option<i32> {
    let character = grid[row][col];
    if character.is_ascii_digit() {
        let length = length_of_number(row, col);
        if has_symbol_neighbour(row, col, length, grid_height, grid_width, grid) {
            let number_chars = &grid[row][col..col + length];
            let number_string: String = number_chars.iter().collect();
            let number: i32 = number_string.parse().unwrap();
            return Some(number);
        }
    }

    None
}

fn length_of_number(row: usize, col: usize) -> usize {}

fn has_symbol_neighbour(
    row: usize,
    col: usize,
    length: usize,
    grid_height: usize,
    grid_width: usize,
    grid: Vec<Vec<char>>,
) -> bool {
    for check_row in row - 1..row + 1 {
        for check_col in col - 1..col + length + 1 {
            if within_bounds(check_row, check_col, grid_height, grid_width) {
                let neighbour = grid[check_row][check_col];
                if is_symbol(neighbour) {
                    return true;
                }
            }
        }
    }

    false
}

fn within_bounds(row: usize, col: usize, grid_height: usize, grid_width: usize) -> bool {
    row >= 0 && row < grid_height && col >= 0 && col < grid_width
}

fn is_symbol(c: char) -> bool {}
