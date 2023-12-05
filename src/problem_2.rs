use std::{error::Error, fs};

pub fn problem_2() -> Result<(), Box<dyn Error>> {
    let file_path = "./res/02/input";
    let contents = fs::read_to_string(file_path)?;
    let mut calibration_sum = 0;

    for line in contents.lines() {
        if let Some(game) = interpret_game(line) {
            let min_cubes = min_cube_count(&game);

            if min_cubes.red <= 12 && min_cubes.green <= 13 && min_cubes.blue <= 14 {
                // println!("{line}");
                calibration_sum += game.id;
            }
        }
    }

    println!("\nThe total calibration sum is: {}", calibration_sum);

    Ok(())
}

struct Game {
    id: u32,
    draws: Vec<BagDraw>,
}

struct BagDraw {
    red: u32,
    green: u32,
    blue: u32,
}

fn interpret_game(input: &str) -> Option<Game> {
    let id_and_draws: Vec<&str> = input.split(":").collect();
    if id_and_draws.len() == 2 {
        // TODO: Correct the error handling
        let id_string = id_and_draws[0].strip_prefix("Game ").unwrap();
        let id: u32 = id_string.parse().unwrap();

        let draw_strings: Vec<&str> = id_and_draws[1].split(";").collect();
        let mut draws: Vec<BagDraw> = Vec::new();

        for draw_string in draw_strings {
            if let Some(draw) = interpret_draw(draw_string) {
                draws.push(draw);
            }
        }

        return Some(Game { id, draws });
    }

    None
}

fn interpret_draw(input: &str) -> Option<BagDraw> {
    let colours: Vec<&str> = input.split(",").collect();
    if colours.len() > 3 {
        return None;
    }

    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;

    let mut colour_count = colours.len();

    for colour in colours {
        if let Some(count) = colour.strip_suffix("red") {
            red = count.trim().parse().unwrap();
            colour_count -= 1;
        }
        if let Some(count) = colour.strip_suffix("green") {
            green = count.trim().parse().unwrap();
            colour_count -= 1;
        }
        if let Some(count) = colour.strip_suffix("blue") {
            blue = count.trim().parse().unwrap();
            colour_count -= 1;
        }
    }

    if colour_count == 0 {
        Some(BagDraw { red, green, blue })
    } else {
        None
    }
}

fn min_cube_count(game: &Game) -> BagDraw {
    let draws = &game.draws;
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;

    for draw in draws {
        if draw.red > red {
            red = draw.red;
        }
        if draw.green > green {
            green = draw.green;
        }
        if draw.blue > blue {
            blue = draw.blue;
        }
    }

    BagDraw { red, green, blue }
}
