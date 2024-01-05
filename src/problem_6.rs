use std::{error::Error, fs};

use crate::utils::common_ops;

pub fn problem_5() -> Result<(), Box<dyn Error>> {
    let file_path = "./res/06/input";
    let contents = fs::read_to_string(file_path)?;

    let mut possible_wins = 1;

    let races = create_races(contents);

    for race in races {
        if let Some((lower, upper)) = zeros(race) {
            let diff = upper - lower;
            possible_wins *= diff;
        }
    }

    println!("The number of possible wins is: {}", possible_wins);

    Ok(())
}

#[derive(Debug)]
struct Race {
    time: i64,
    distance: i64,
}

fn create_races(input: String) -> Vec<Race> {
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

        let min_solution = f_solution_1.min(f_solution_2) as i64 + 1;
        let max_solution = f_solution_1.max(f_solution_2) as i64;

        Some((min_solution, max_solution))
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use crate::problem_6::zeros;

    use super::Race;

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
    }
}
