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

use std::process;

fn main() {
    if let Err(e) = problem_1::problem_1() {
        println!("Application Error: {e}");
        process::exit(1);
    }

    if let Err(e) = problem_2::problem_2() {
        println!("Application Error: {e}");
        process::exit(1);
    }

    if let Err(e) = problem_3::problem_3() {
        println!("Application Error: {e}");
        process::exit(1);
    }

    if let Err(e) = problem_4::problem_4() {
        println!("Application Error: {e}");
        process::exit(1);
    }
}
