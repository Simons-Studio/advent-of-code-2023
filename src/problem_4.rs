use std::{error::Error, fs};

pub fn problem_4() -> Result<(), Box<dyn Error>> {
    let file_path = "./res/04/input";
    let contents = fs::read_to_string(file_path)?;

    Ok(())
}

fn winning_points_accumulator(input: String) -> i32 {
    /* // TODO
     * Remove card number
     * Split winning numbers from received numbers
     * Check how many are contained
     */
    let mut game_point_sum = 0;

    for line in input.lines() {
        let card_no_and_lists = line.split_once(':');
        let wins_and_recieved = if let Some((_, numbers)) = card_no_and_lists {
            numbers.split_once('|')
        } else {
            None
        };
        let game_points = if let Some((wins_str, recieved_str)) = wins_and_recieved {
            let wins = get_numbers(wins_str);
            let recieved = get_numbers(recieved_str);
            point_score(wins, recieved)
        } else {
            0
        };
        game_point_sum += game_points;
    }

    game_point_sum
}

fn point_score(wins: Vec<i32>, recieved: Vec<i32>) -> i32 {}

fn get_numbers(number_list_string: &str) -> Vec<i32> {
    let number_strings = number_list_string.split_ascii_whitespace();
    let mut numbers = Vec::new();
    for number_str in number_strings {
        let number: i32 = number_str.parse().unwrap();
        numbers.push(number);
    }
    numbers
}

#[cfg(test)]
mod tests {
    use crate::problem_4::point_score;

    use super::{get_numbers, winning_points_accumulator};

    #[test]
    fn winning_points_accumulator_test() {
        let example = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let points = 13;
        assert_eq!(points, winning_points_accumulator(String::from(example)));
    }

    #[test]
    fn point_score_test() {
        let wins = vec![41, 48, 83, 86, 17];
        let recieved = vec![83, 86, 6, 31, 17, 9, 48, 53];
        let points = 8;
        assert_eq!(points, point_score(wins, recieved));
    }

    #[test]
    fn get_numbers_test() {
        let numbers_list_string = "12 23  2 12 32  6 34 97 67";
        let numbers_list = vec![12, 23, 2, 12, 32, 6, 34, 97, 67];
        assert_eq!(get_numbers(numbers_list_string), numbers_list);
    }
}
