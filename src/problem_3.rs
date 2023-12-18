use std::{error::Error, fs};

pub fn problem_3() -> Result<(), Box<dyn Error>> {
    let file_path = "./res/03/input";
    let contents = fs::read_to_string(file_path)?;
    let grid = create_engine_grid(contents);
    let part_number_sum = part_number_accumulator(&grid);
    let gear_ratio_sum = gear_ratio_accumulator(&grid);

    println!("The sum of all the part numbers is: {}", part_number_sum);
    println!("The sum of all the gear ratios is: {}", gear_ratio_sum);

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

fn part_number_accumulator(grid: &Vec<Vec<char>>) -> i32 {
    if grid.len() <= 0 {
        panic!("Grid too short");
    }

    let mut part_number_sum = 0;

    let grid_height = grid.len();
    let grid_width = grid[0].len();

    for row in 0..grid_height {
        let mut col = 0;
        while col < grid_width {
            if let Some(part_number) = get_part_number(row, col, grid_height, grid_width, &grid) {
                col += part_number.number_string.len();
                let part_number: i32 = part_number.number_string.parse().unwrap();
                part_number_sum += part_number;
            } else {
                col += 1;
            }
        }
    }

    part_number_sum
}

fn get_part_number(
    row: usize,
    col: usize,
    grid_height: usize,
    grid_width: usize,
    grid: &Vec<Vec<char>>,
) -> Option<PartNumber> {
    let character = grid[row][col];
    if character.is_ascii_digit() {
        let length = length_of_number(row, col, grid_width, &grid);
        if has_symbol_neighbour(row, col, length, grid_height, grid_width, &grid) {
            let number_chars = &grid[row][col..col + length];
            let number_string: String = number_chars.iter().collect();
            let position = Position { row, col };
            return Some(PartNumber {
                number_string,
                position,
            });
        } else {
            None
        }
    } else {
        None
    }
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
    let max_row = if row + 2 >= grid_height {
        grid_height
    } else {
        row + 2
    };
    let max_col = if col + length + 1 >= grid_width {
        grid_width
    } else {
        col + length + 1
    };

    for check_row in min_row..max_row {
        for check_col in min_col..max_col {
            let neighbour = grid[check_row][check_col];
            if is_symbol(neighbour) {
                return true;
            }
        }
    }

    false
}

fn is_symbol(c: char) -> bool {
    let symbols = "!@#$%^&*()<>?/|-=+_[]{}";
    symbols.contains(c)
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct PartNumber {
    number_string: String,
    position: Position,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Position {
    row: usize,
    col: usize,
}

// PART 2
fn gear_ratio_accumulator(grid: &Vec<Vec<char>>) -> i32 {
    if grid.len() <= 0 {
        panic!("Grid too short");
    }

    let mut gear_ratio_sum = 0;

    let grid_height = grid.len();
    let grid_width = grid[0].len();

    for row in 0..grid_height {
        for col in 0..grid_width {
            let c = grid[row][col];
            if is_gear(c) {
                gear_ratio_sum += get_gear_ratio(row, col, grid_height, grid_width, grid);
            }
        }
    }

    gear_ratio_sum
}

fn get_gear_ratio(
    row: usize,
    col: usize,
    grid_height: usize,
    grid_width: usize,
    grid: &Vec<Vec<char>>,
) -> i32 {
    let numbers = numbers_adjacent_to_gear(row, col, grid_height, grid_width, grid);
    if numbers.len() == 2 {
        let mut ratio = 1;
        for n in numbers {
            ratio *= n;
        }
        ratio
    } else {
        0
    }
}

fn is_gear(c: char) -> bool {
    c == '*'
}

fn numbers_adjacent_to_gear(
    row: usize,
    col: usize,
    grid_height: usize,
    grid_width: usize,
    grid: &Vec<Vec<char>>,
) -> Vec<i32> {
    // Set bounds
    let min_row = if row == 0 { 0 } else { row - 1 };
    let min_col = if col == 0 { 0 } else { col - 1 };
    let max_row = if row + 2 >= grid_height {
        grid_height
    } else {
        row + 2
    };
    let max_col = if col + 2 >= grid_width {
        grid_width
    } else {
        col + 2
    };

    let mut gear_numbers: Vec<i32> = Vec::new();

    for check_row in min_row..max_row {
        let mut check_col = min_col;
        while check_col >= min_col && check_col < max_col {
            if let Some(gear_number) = get_number(check_row, check_col, grid_width, grid) {
                let number: i32 = gear_number.number_string.parse().unwrap();
                gear_numbers.push(number);

                check_col = gear_number.position.col + gear_number.number_string.len();
            } else {
                check_col += 1;
            }
        }
    }

    gear_numbers
}

fn get_number(
    row: usize,
    col: usize,
    grid_width: usize,
    grid: &Vec<Vec<char>>,
) -> Option<PartNumber> {
    let character = grid[row][col];
    if character.is_ascii_digit() {
        let mut index = col;
        let mut length = 0;
        let mut within_bound = index > 0;

        while within_bound && grid[row][index - 1].is_ascii_digit() {
            index -= 1;
            length += 1;
            within_bound = index > 0;
        }

        within_bound = index + length < grid_width;

        while within_bound && grid[row][index + length].is_ascii_digit() {
            length += 1;
            within_bound = index + length < grid_width;
        }

        let position = Position { row, col: index };

        let number_chars = &grid[row][index..index + length];
        let number_string: String = number_chars.iter().collect();

        Some(PartNumber {
            number_string,
            position,
        })
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_3::{
        gear_ratio_accumulator, get_number, has_symbol_neighbour, numbers_adjacent_to_gear,
        PartNumber, Position,
    };

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

        let row = 2;
        let col = 0;
        let length = 3;
        let grid_height = 3;
        let grid_width = 4;
        let grid = vec![
            vec!['.', '.', '.', '.'],
            vec!['.', '.', '.', '.'],
            vec!['1', '2', '3', '#'],
        ];
        assert!(has_symbol_neighbour(
            row,
            col,
            length,
            grid_height,
            grid_width,
            &grid
        ));
    }

    #[test]
    fn numbers_adjacent_to_gear_test() {
        let row = 1;
        let col = 1;
        let grid_height = 3;
        let grid_width = 4;
        let grid = vec![
            vec!['1', '2', '3', '.'],
            vec!['.', '*', '.', '.'],
            vec!['.', '4', '5', '6'],
        ];
        let gears = vec![123, 456];
        assert_eq!(
            gears,
            numbers_adjacent_to_gear(row, col, grid_height, grid_width, &grid)
        );
    }

    #[test]
    fn get_number_test() {
        let row = 0;
        let col = 1;
        let grid_width = 4;
        let grid = vec![
            vec!['1', '2', '3', '.'],
            vec!['.', '*', '.', '.'],
            vec!['.', '4', '5', '6'],
        ];
        assert_eq!(
            Some(PartNumber {
                number_string: String::from("123"),
                position: Position { row: 0, col: 0 }
            }),
            get_number(row, col, grid_width, &grid)
        );

        let row = 2;
        let col = 3;
        let grid_width = 4;
        let grid = vec![
            vec!['1', '2', '3', '.'],
            vec!['.', '*', '.', '.'],
            vec!['.', '4', '5', '6'],
        ];
        assert_eq!(
            Some(PartNumber {
                number_string: String::from("456"),
                position: Position { row: 2, col: 1 }
            }),
            get_number(row, col, grid_width, &grid)
        );
    }

    #[test]
    fn gear_ratio_accumulator_test() {
        let grid = vec![
            vec!['1', '2', '3', '.'],
            vec!['.', '*', '.', '.'],
            vec!['.', '4', '5', '6'],
        ];
        assert_eq!(56088, gear_ratio_accumulator(&grid));

        let grid = vec![
            vec!['1', '2', '3', '.'],
            vec!['.', '*', '.', '.'],
            vec!['.', '4', '5', '6'],
            vec!['.', '.', '.', '.'],
            vec!['*', '4', '5', '6'],
        ];
        assert_eq!(56088, gear_ratio_accumulator(&grid));
    }
}
