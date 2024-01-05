use std::{cmp, error::Error, fmt::UpperExp, fs};

use crate::utils::common_ops;

pub fn problem_5() -> Result<(), Box<dyn Error>> {
    let file_path = "./res/06/input";
    let contents = fs::read_to_string(file_path)?;

    Ok(())
}

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

    let discriminant = f_time * f_time - f_distance;

    if discriminant >= 0.0 {
        let f_solution_1 = (f_time + discriminant.sqrt()) / 2.0;
        let f_solution_2 = (f_time - discriminant.sqrt()) / 2.0;

        let min_solution = f_solution_1.min(f_solution_1) as i64;
        let max_solution = f_solution_1.max(f_solution_2) as i64 + 1;

        Some((min_solution, max_solution))
    } else {
        None
    }
}
