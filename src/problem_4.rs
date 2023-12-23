use std::{collections::HashMap, error::Error, fs};

use crate::common_ops;

pub fn problem_4() -> Result<(), Box<dyn Error>> {
    let file_path = "./res/04/input";
    let contents = fs::read_to_string(file_path)?;
    let total_score = winning_points_accumulator(&contents);

    println!(
        "The total winning score for all the games is {}",
        total_score
    );

    let number_of_cards = number_of_cards(&contents);

    println!("The total number of cards is {}", number_of_cards);

    Ok(())
}

#[derive(PartialEq, Eq, Debug)]
struct Game {
    card_no: i64,
    wins: Vec<i64>,
    recieved: Vec<i64>,
}

fn winning_points_accumulator(input: &String) -> i64 {
    let mut game_point_sum = 0;

    for game_str in input.lines() {
        if let Some(game) = get_game_values(game_str) {
            game_point_sum += point_score(game);
        }
    }

    game_point_sum
}

fn get_game_values(game: &str) -> Option<Game> {
    let card_no_and_lists = game.split_once(':');
    if let Some((card_no_str, numbers_str)) = card_no_and_lists {
        let card_no_option = get_card_no(card_no_str);
        let wins_and_recieved_str = numbers_str.split_once('|');

        if let Some(card_no) = card_no_option {
            if let Some((wins_str, recieved_str)) = wins_and_recieved_str {
                let wins = common_ops::get_numbers(wins_str);
                let recieved = common_ops::get_numbers(recieved_str);
                Some(Game {
                    card_no,
                    wins,
                    recieved,
                })
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    }
}

fn get_card_no(card_no_str: &str) -> Option<i64> {
    if let Some(stripped) = card_no_str.strip_prefix("Card") {
        let card_no: i64 = stripped.trim().parse().unwrap();
        Some(card_no)
    } else {
        None
    }
}

fn point_score(game: Game) -> i64 {
    let mut score = 0;
    for w in game.wins {
        if game.recieved.contains(&w) {
            if score == 0 {
                score = 1;
            } else {
                score *= 2;
            }
        }
    }
    score
}

// PART 2
fn number_of_cards(input: &String) -> i64 {
    let card_copies = assign_number_of_copies(input);
    let mut number_of_cards = 0;
    for (_, copies) in card_copies {
        number_of_cards += copies;
    }
    number_of_cards
}

fn assign_number_of_copies(input: &String) -> HashMap<i64, i64> {
    let mut card_copies: HashMap<i64, i64> = HashMap::new();

    for line in input.lines() {
        let Some(game) = get_game_values(line) else {
            break;
        };

        let card_no = game.card_no;
        let copies = *card_copies.entry(card_no).or_insert(1);

        let number_of_cards_copied = number_of_wins(game);
        for card_no_offset in 1..number_of_cards_copied + 1 {
            let copied_card_number = card_no + card_no_offset;
            let copied_card_copies = card_copies.entry(copied_card_number).or_insert(1);
            *copied_card_copies += copies;

            // println!(
            //     "For card no: {card_no}, there are {copies} copies of card: {copied_card_number}"
            // );
        }
    }

    card_copies
}

fn number_of_wins(game: Game) -> i64 {
    let mut number_wins = 0;
    for w in game.wins {
        if game.recieved.contains(&w) {
            number_wins += 1;
        }
    }
    number_wins
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::problem_4::{
        assign_number_of_copies, get_game_values, number_of_cards, number_of_wins, point_score,
    };

    use super::{winning_points_accumulator, Game};

    #[test]
    fn winning_points_accumulator_test() {
        let example = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let points = 13;
        assert_eq!(points, winning_points_accumulator(&String::from(example)));
    }

    #[test]
    fn point_score_test() {
        let wins = vec![41, 48, 83, 86, 17];
        let recieved = vec![83, 86, 6, 31, 17, 9, 48, 53];
        let points = 8;
        assert_eq!(
            points,
            point_score(Game {
                card_no: 0,
                wins,
                recieved
            })
        );
    }

    #[test]
    fn get_game_values_test() {
        let game_str = "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83";
        let game = Game {
            card_no: 4,
            wins: vec![41, 92, 73, 84, 69],
            recieved: vec![59, 84, 76, 51, 58, 5, 54, 83],
        };
        assert_eq!(Some(game), get_game_values(game_str));
    }

    #[test]
    fn number_of_cards_test() {
        let example = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let copies = 30;
        assert_eq!(copies, number_of_cards(&String::from(example)));
    }

    #[test]
    fn assign_number_of_copies_test() {
        let example = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let copies = HashMap::from([(1, 1), (2, 2), (3, 4), (4, 8), (5, 14), (6, 1)]);
        assert_eq!(copies, assign_number_of_copies(&String::from(example)));
    }

    #[test]
    fn number_of_wins_test() {
        let game = Game {
            card_no: 1,
            wins: vec![41, 48, 83, 86, 17],
            recieved: vec![83, 86, 6, 31, 17, 9, 48, 53],
        };
        let matches = 4;
        assert_eq!(matches, number_of_wins(game));

        let game = Game {
            card_no: 4,
            wins: vec![41, 92, 73, 84, 69],
            recieved: vec![59, 84, 76, 51, 58, 5, 54, 83],
        };
        let matches = 1;
        assert_eq!(matches, number_of_wins(game));
    }
}
