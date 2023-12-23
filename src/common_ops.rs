pub fn get_numbers(number_list_string: &str) -> Vec<i32> {
    let number_strings = number_list_string.split_ascii_whitespace();
    let mut numbers = Vec::new();
    for number_str in number_strings {
        let number: i32 = number_str.parse().unwrap();
        numbers.push(number);
    }
    numbers
}
