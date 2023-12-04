use std::{error::Error, fs};

pub fn problem_1() -> Result<(), Box<dyn Error>> {
    let file_path = "./res/01/input";
    let contents = fs::read_to_string(file_path)?; //.expect("Should have been able to read the file");

    let mut calibration_sum = 0;

    for mut line in contents.lines() {
        // let mut index = 0;
        let mut calibration_value = 0;
        let mut most_recent_digit_option: Option<u32> = None;

        while line != "" {
            if let Some(digit) = starts_with_digit(line) {
                if most_recent_digit_option == None {
                    calibration_value = 10 * digit;
                }
                most_recent_digit_option = Some(digit);
            }
            line = &line[1..];
        }

        if let Some(digit) = most_recent_digit_option {
            calibration_value += digit;
        }

        print!("{}, ", calibration_value);

        calibration_sum += calibration_value;
    }

    println!("\nThe total calibration sum is: {}", calibration_sum);

    Ok(())
}

fn starts_with_digit(input: &str) -> Option<u32> {
    let digit_numerals = [
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("0", 0),
    ];

    for digit_numeral in digit_numerals {
        if input.starts_with(digit_numeral.0) {
            let digit = digit_numeral.1;
            return Some(digit);
        }
    }

    let digit_strings = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    for digit_string in digit_strings {
        if input.starts_with(digit_string.0) {
            let digit = digit_string.1;
            return Some(digit);
        }
    }

    None
}
