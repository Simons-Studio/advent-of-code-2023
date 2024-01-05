use std::{error::Error, fs};

use crate::utils::common_ops;

pub fn problem_6() -> Result<(), Box<dyn Error>> {
    let file_path = "../res/06/input";
    let contents = fs::read_to_string(file_path)?;

    let possible_wins = number_of_solutions(&contents);
    println!("The number of possible wins is: {}", possible_wins);

    if let Some(one_solution) = one_big_solution(&contents) {
        println!("The single solution is: {}", one_solution);
    }

    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
struct Race {
    time: i64,
    distance: i64,
}

fn one_big_solution(input: &String) -> Option<i64> {
    let race = create_one_race(input);

    if let Some((lower, upper)) = zeros(race) {
        Some(upper - lower + 1)
    } else {
        None
    }
}

fn number_of_solutions(input: &String) -> i64 {
    let mut possible_wins = 1;

    let races = create_races(input);

    for race in races {
        if let Some((lower, upper)) = zeros(race) {
            let diff = upper - lower + 1;
            possible_wins *= diff;
        }
    }

    possible_wins
}

fn create_one_race(input: &String) -> Race {
    let mut lines = input.lines();

    let time_segments: Vec<&str> = lines
        .next()
        .unwrap()
        .strip_prefix("Time:")
        .unwrap()
        .split_whitespace()
        .collect();

    let dist_segments: Vec<&str> = lines
        .next()
        .unwrap()
        .strip_prefix("Distance:")
        .unwrap()
        .split_whitespace()
        .collect();

    let time: i64 = time_segments.concat().parse().unwrap();
    let distance: i64 = dist_segments.concat().parse().unwrap();

    Race { time, distance }
}

fn create_races(input: &String) -> Vec<Race> {
    let mut lines = input.lines();
    let mut races = Vec::new();

    let times = common_ops::get_numbers(lines.next().unwrap().strip_prefix("Time:").unwrap());
    let dists = common_ops::get_numbers(lines.next().unwrap().strip_prefix("Distance:").unwrap());

    let time_iter = times.iter();
    let dist_iter = dists.iter();

    for (&time, &distance) in time_iter.zip(dist_iter) {
        let race = Race { time, distance };
        races.push(race);
    }

    races
}

fn zeros(race: Race) -> Option<(i64, i64)> {
    let f_time = race.time as f64;
    let f_distance = race.distance as f64;

    let discriminant = f_time * f_time - 4.0 * f_distance;

    if discriminant >= 0.0 {
        let discriminant_sqrt = discriminant.sqrt();
        let f_solution_1 = (f_time + discriminant_sqrt) / 2.0;
        let f_solution_2 = (f_time - discriminant_sqrt) / 2.0;

        let min_solution = f_solution_1.min(f_solution_2).floor() as i64 + 1;
        let max_solution = f_solution_1.max(f_solution_2).ceil() as i64 - 1;

        Some((min_solution, max_solution))
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use crate::problem_6::{number_of_solutions, zeros};

    use super::{create_one_race, Race};

    #[test]
    fn test_zeros() {
        let race_1 = Race {
            time: 7,
            distance: 9,
        };
        assert_eq!(Some((2, 5)), zeros(race_1));

        let race_2 = Race {
            time: 15,
            distance: 40,
        };
        assert_eq!(Some((4, 11)), zeros(race_2));

        let race_3 = Race {
            time: 30,
            distance: 200,
        };
        assert_eq!(Some((11, 19)), zeros(race_3));
    }

    #[test]
    fn test_num_solutions() {
        let input = String::from(
            "Time:      7  15   30
Distance:  9  40  200",
        );
        assert_eq!(number_of_solutions(&input), 288);
    }

    #[test]
    fn test_create_one_race() {
        let input = String::from(
            "Time:      7  15   30
Distance:  9  40  200",
        );
        let race = Race {
            time: 71530,
            distance: 940200,
        };
        assert_eq!(create_one_race(&input), race)
    }
}
