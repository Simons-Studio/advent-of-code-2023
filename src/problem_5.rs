/* // TODO: Create the functionality to create map rules
 * 1. Create a struct to hold the contents of a "map"
 * 2. Create a function that can read the contents of the files into the struct
 */

use std::{error::Error, fs};

pub fn problem_5() -> Result<(), Box<dyn Error>> {
    let file_path = "./res/05/input";
    let contents = fs::read_to_string(file_path)?;
    Ok(())
}

#[derive(Debug)]
struct CategoryMap {
    maps: Vec<SourceToDestinationMap>,
}
impl CategoryMap {
    fn transform(&self, source: i32) -> i32 {
        for map in &self.maps {
            if let Some(result) = map.transform(source) {
                return result;
            }
        }
        source
    }
}

#[derive(Debug)]
struct SourceToDestinationMap {
    destination_range_start: i32,
    source_range_start: i32,
    range: i32,
    source_to_destination_difference: i32,
}
impl SourceToDestinationMap {
    fn new(
        destination_range_start: i32,
        source_range_start: i32,
        range: i32,
    ) -> SourceToDestinationMap {
        let source_to_destination_difference = destination_range_start - source_range_start;
        SourceToDestinationMap {
            destination_range_start,
            source_range_start,
            range,
            source_to_destination_difference,
        }
    }

    fn in_source_range(&self, source: i32) -> bool {
        source >= self.source_range_start && source < self.source_range_start + self.range
    }

    fn transform(&self, source: i32) -> Option<i32> {
        if self.in_source_range(source) {
            Some(source + self.source_to_destination_difference)
        } else {
            None
        }
    }
}
