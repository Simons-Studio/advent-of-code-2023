// TODO: Tasks to complete this puzzle
// 1. Take in a text file
// 2. Seperate out a line
// 3. Look through line for digits
// 4. Produce number with first and last digit
// 5. Sum all the created numbers

use std::fs;

fn main() {
    let file_path = "./res/01/input";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("Contents of the file:\n{contents}");

    let mut calibration_sum = 0;

    for line in contents.lines() {
        let mut calibration_value = 0;
        let mut most_recent_digit_option: Option<u32> = None;

        for character in line.chars() {
            if character.is_ascii_digit() {
                if let Some(digit) = character.to_digit(10) {
                    if most_recent_digit_option == None {
                        calibration_value = 10 * digit;
                    }

                    most_recent_digit_option = Some(digit);
                }
            }
        }

        if let Some(digit) = most_recent_digit_option {
            calibration_value += digit;
        }

        println!("{}", calibration_value);

        calibration_sum += calibration_value;
    }

    println!("The total calibration sum is: {}", calibration_sum);
}
