/* // TODO: Create the functionality to create map rules
 * 1. Create a struct to hold the contents of a "map"
 * 2. Create a function that can read the contents of the files into the struct
 */

use std::{error::Error, fs};

use crate::common_ops;

pub fn problem_5() -> Result<(), Box<dyn Error>> {
    let file_path = "./res/05/input";
    let contents = fs::read_to_string(file_path)?;
    Ok(())
}

fn create_category(category_str: String) -> Option<CategoryMap<'static>> {
    let mut maps: Vec<SourceToDestinationMap> = Vec::new();

    let mut lines = category_str.lines();
    let Some(name) = lines.next() else {
        return None;
    };

    let name = sanitize_name(name);

    for line in lines {
        let numbers = common_ops::get_numbers(line);
        if numbers.len() == 3 {
            let destination_range_start = numbers[0];
            let source_range_start = numbers[1];
            let range = numbers[2];
            let map =
                SourceToDestinationMap::new(destination_range_start, source_range_start, range);
            maps.push(map);
        }
    }

    Some(CategoryMap { name, maps })
}

fn sanitize_name(dirty_name: &str) -> &str {
    if let Some(name) = dirty_name.strip_suffix(" map:") {
        name
    } else {
        dirty_name
    }
}

#[derive(Debug)]
struct CategoryMap<'a> {
    name: &'a str,
    maps: Vec<SourceToDestinationMap>,
}
impl CategoryMap<'_> {
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
