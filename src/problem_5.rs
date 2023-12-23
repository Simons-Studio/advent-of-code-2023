use std::{error::Error, fs};

pub fn problem_5() -> Result<(), Box<dyn Error>> {
    let file_path = "./res/05/input";
    let contents = fs::read_to_string(file_path)?;
    Ok(())
}
