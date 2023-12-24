// TODO: Tasks to complete this puzzle
// 1. Take in a text file
// 2. Seperate out a line
// 3. Look through line for digits
// 4. Produce number with first and last digit
// 5. Sum all the created numbers

mod problem_1;
mod problem_2;
mod problem_3;
mod problem_4;
mod problem_5;
mod utils;

use std::process;

fn main() {
    let selector = 5;

    match selector {
        1 => {
            if let Err(e) = problem_1::problem_1() {
                println!("Application Error: {e}");
                process::exit(1);
            }
        }

        2 => {
            if let Err(e) = problem_2::problem_2() {
                println!("Application Error: {e}");
                process::exit(1);
            }
        }

        3 => {
            if let Err(e) = problem_3::problem_3() {
                println!("Application Error: {e}");
                process::exit(1);
            }
        }

        4 => {
            if let Err(e) = problem_4::problem_4() {
                println!("Application Error: {e}");
                process::exit(1);
            }
        }

        5 => {
            if let Err(e) = problem_5::problem_5() {
                println!("Application Error: {e}");
                process::exit(1);
            }
        }

        _ => {}
    }
}
