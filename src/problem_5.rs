/* // TODO: Create the functionality to create map rules
 * 1. Create a struct to hold the contents of a "map"
 * 2. Create a function that can read the contents of the files into the struct
 */

use std::{error::Error, fs};

use crate::common_ops;

pub fn problem_5() -> Result<(), Box<dyn Error>> {
    let file_path = "./res/05/input";
    let contents = fs::read_to_string(file_path)?;

    top_level_function(contents);

    Ok(())
}

// TODO: Rename this function
fn top_level_function(input: String) {
    let mut sections = input.split("\n\n");
    let Some(seeds_str) = sections.next() else {
        return;
    };
    let Some(seeds) = collect_seeds(seeds_str) else {
        return;
    };
    let categories = create_categories(sections.collect());
}

fn category_transform(seeds: Vec<i32>, category: CategoryMap) -> Vec<i32> {
    seeds.iter().map(|x| category.transform(*x)).collect()
}

fn collect_seeds(seeds_str: &str) -> Option<Vec<i32>> {
    if let Some(numbers_str) = seeds_str.strip_prefix("seeds: ") {
        Some(common_ops::get_numbers(numbers_str))
    } else {
        None
    }
}

fn create_categories(sections: Vec<&str>) -> Vec<CategoryMap> {
    let mut category_maps: Vec<CategoryMap> = Vec::new();
    for section in sections {
        if let Some(category) = create_category_map(section) {
            category_maps.push(category);
        }
    }
    category_maps
}

fn create_category_map(category_str: &str) -> Option<CategoryMap> {
    let mut lines = category_str.lines();
    let Some(name) = lines.next() else {
        return None;
    };
    let name = sanitize_name(name);

    let maps = create_map_element_list(lines.collect());

    Some(CategoryMap { name, maps })
}

fn sanitize_name(dirty_name: &str) -> String {
    let name = if let Some(name) = dirty_name.strip_suffix(" map:") {
        name
    } else {
        dirty_name
    };
    String::from(name)
}

fn create_map_element_list(numbers_string_list: Vec<&str>) -> Vec<MapElement> {
    let mut maps: Vec<MapElement> = Vec::new();
    for line in numbers_string_list {
        if let Some(map) = create_map_element(line) {
            maps.push(map);
        }
    }
    maps
}

fn create_map_element(numbers_str: &str) -> Option<MapElement> {
    let numbers = common_ops::get_numbers(numbers_str);
    if numbers.len() == 3 {
        let destination_range_start = numbers[0];
        let source_range_start = numbers[1];
        let range = numbers[2];
        let map = MapElement::new(destination_range_start, source_range_start, range);
        Some(map)
    } else {
        None
    }
}

#[derive(Debug, PartialEq, Eq)]
struct CategoryMap {
    name: String,
    maps: Vec<MapElement>,
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

#[derive(Debug, PartialEq, Eq)]
struct MapElement {
    destination_range_start: i32,
    source_range_start: i32,
    range: i32,
    source_to_destination_difference: i32,
}
impl MapElement {
    fn new(destination_range_start: i32, source_range_start: i32, range: i32) -> MapElement {
        let source_to_destination_difference = destination_range_start - source_range_start;
        MapElement {
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

#[cfg(test)]
mod tests {
    use crate::problem_5::{category_transform, create_category_map};

    use super::{collect_seeds, create_map_element, CategoryMap, MapElement};

    #[test]
    fn test_create_category_map() {
        let example = "seed-to-soil map:
    50 98 2
    52 50 48";
        let map_element_1 = MapElement::new(50, 98, 2);
        let map_element_2 = MapElement::new(52, 50, 48);
        let category = CategoryMap {
            name: String::from("seed-to-soil"),
            maps: vec![map_element_1, map_element_2],
        };
        assert_eq!(Some(category), create_category_map(example));
    }

    #[test]
    fn test_category_transform() {
        let seeds = vec![79, 14, 55, 13];
        let map_element_1 = MapElement::new(50, 98, 2);
        let map_element_2 = MapElement::new(52, 50, 48);
        let category = CategoryMap {
            name: String::from("seed-to-soil"),
            maps: vec![map_element_1, map_element_2],
        };

        let soils = vec![81, 14, 57, 13];
        assert_eq!(soils, category_transform(seeds, category));
    }
}
