use std::{error::Error, fs};

pub fn problem_3() -> Result<(), Box<dyn Error>> {
    let file_path = "./res/03/input";
    let contents = fs::read_to_string(file_path)?;
    let grid = create_engine_grid(contents);

    // 1. Convert to grid

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

fn identify_neighbours(
    row: usize,
    col: usize,
    max_row: usize,
    max_col: usize,
    grid: Vec<Vec<char>>,
) -> Option<Vec<char>> {
    let mut check_row = row - 1;
    let mut check_col = col - 1;
    let mut number_length = 0;

    let x = grid[0][0];

    if within_bounds(check_row, check_col, max_row, max_col) {
        let neighbour = grid[check_row][check_col];
    }

    None
}

fn within_bounds(row: usize, col: usize, max_row: usize, max_col: usize) -> bool {
    row >= 0 && row < max_row && col >= 0 && col < max_col
}

fn is_symbol(c: char) -> bool {}
