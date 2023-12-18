use std::{error::Error, fs};

pub fn problem_4() -> Result<(), Box<dyn Error>> {
    let file_path = "./res/04/input";
    let contents = fs::read_to_string(file_path)?;

    Ok(())
}

fn winning_points_accumulator(input: String) {
    /* TODO
     * Remove card number
     * Split winning numbers from received numbers
     * Check how many are contained
     */
    for line in input.lines() {}
}

fn get_numbers(number_list_string: &str) -> Vec<i32> {
    let number_strings = number_list_string.split_ascii_whitespace();
    for number_str in number_strings {
        let number: i32 = number_str.parse().unwrap();
    }

    vec![0]
}

#[cfg(test)]
mod tests {
    use super::get_numbers;

    #[test]
    fn get_numbers_test() {
        let numbers_list_string = "12 23  2 12 32  6 34 97 67";
        let numbers_list = vec![12, 23, 2, 12, 32, 6, 34, 97, 67];
        assert_eq!(get_numbers(numbers_list_string), numbers_list);
    }
}
