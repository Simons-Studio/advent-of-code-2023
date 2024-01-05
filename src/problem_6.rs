use std::{error::Error, fs};

use crate::utils::{common_ops, interval::Interval};

pub fn problem_5() -> Result<(), Box<dyn Error>> {
    let file_path = "./res/06/input";
    let contents = fs::read_to_string(file_path)?;

    Ok(())
}
