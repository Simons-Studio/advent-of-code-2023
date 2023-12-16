use std::{error::Error, fs};

pub fn problem_3() -> Result<(), Box<dyn Error>> {
    let file_path = "./res/03/input";
    let contents = fs::read_to_string(file_path)?;
    let grid = create_engine_grid(contents);
    let part_number_sum = part_number_accumulator(grid);

    println!("The sum of all the part numbers is: {}", part_number_sum);

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

fn part_number_accumulator(grid: Vec<Vec<char>>) -> i32 {
    if grid.len() <= 0 {
        panic!("Grid too short");
    }

    let mut part_number_sum = 0;

    let grid_height = grid.len();
    let grid_width = grid[0].len();

    for row in 0..grid_height {
        let mut col = 0;
        while col < grid_width {
            if let Some(number_string) = part_number(row, col, grid_height, grid_width, &grid) {
                col += number_string.len();
                let part_number: i32 = number_string.parse().unwrap();
                part_number_sum += part_number;
            } else {
                col += 1;
            }
        }
    }

    part_number_sum
}

fn part_number(
    row: usize,
    col: usize,
    grid_height: usize,
    grid_width: usize,
    grid: &Vec<Vec<char>>,
) -> Option<String> {
    let character = grid[row][col];
    if character.is_ascii_digit() {
        let length = length_of_number(row, col, grid_width, &grid);
        if has_symbol_neighbour(row, col, length, grid_height, grid_width, &grid) {
            let number_chars = &grid[row][col..col + length];
            let number_string: String = number_chars.iter().collect();
            return Some(number_string);
        }
    }

    None
}

fn length_of_number(row: usize, col: usize, grid_width: usize, grid: &Vec<Vec<char>>) -> usize {
    let mut length = 0;
    let mut within_bound = col + length < grid_width;

    while within_bound && grid[row][col + length].is_ascii_digit() {
        length += 1;
        within_bound = col + length < grid_width;
    }

    length
}

fn has_symbol_neighbour(
    row: usize,
    col: usize,
    length: usize,
    grid_height: usize,
    grid_width: usize,
    grid: &Vec<Vec<char>>,
) -> bool {
    // Set bounds
    let min_row = if row == 0 { 0 } else { row - 1 };
    let min_col = if col == 0 { 0 } else { col - 1 };
    let max_row = if row >= grid_height {
        grid_height
    } else {
        row + 2
    };
    let max_col = if col + length >= grid_width {
        grid_width
    } else {
        col + length + 1
    };

    println!("");
    println!("min row: {min_row}, min col: {min_col}, max row: {max_row}, max col: {max_col}");

    for check_row in min_row..max_row {
        for check_col in min_col..max_col {
            let neighbour = grid[check_row][check_col];
            print!("{neighbour}");
            if is_symbol(neighbour) {
                return true;
            }
        }
        println!("");
    }
    println!("");

    false
}

fn is_symbol(c: char) -> bool {
    let symbols = "!@#$%^&*()<>?/|-=+_[]{}";
    symbols.contains(c)
}

#[cfg(test)]
mod tests {
    use crate::problem_3::has_symbol_neighbour;

    #[test]
    fn has_symbol_neighbour_test() {
        let row = 0;
        let col = 0;
        let length = 3;
        let grid_height = 3;
        let grid_width = 4;
        let grid = vec![
            vec!['1', '2', '3', '.'],
            vec!['.', '.', '.', '#'],
            vec!['.', '.', '.', '.'],
        ];
        assert!(has_symbol_neighbour(
            row,
            col,
            length,
            grid_height,
            grid_width,
            &grid
        ));

        let row = 2;
        let col = 0;
        let length = 3;
        let grid_height = 5;
        let grid_width = 5;
        let grid = vec![
            vec!['#', '#', '#', '#', '#'],
            vec!['.', '.', '.', '.', '#'],
            vec!['1', '2', '3', '.', '#'],
            vec!['.', '.', '.', '.', '#'],
            vec!['#', '#', '#', '#', '#'],
        ];
        assert!(!has_symbol_neighbour(
            row,
            col,
            length,
            grid_height,
            grid_width,
            &grid
        ));
    }
}
