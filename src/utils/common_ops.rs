pub fn get_numbers(number_list_string: &str) -> Vec<i64> {
    let number_strings = number_list_string.split_ascii_whitespace();
    let mut numbers = Vec::new();
    for number_str in number_strings {
        let number: i64 = number_str.parse().unwrap();
        numbers.push(number);
    }
    numbers
}

#[cfg(test)]
mod tests {
    use crate::common_ops::get_numbers;

    #[test]
    fn get_numbers_test() {
        let numbers_list_string = "12 23  2 12 32  6 34 97 67";
        let numbers_list = vec![12, 23, 2, 12, 32, 6, 34, 97, 67];
        assert_eq!(get_numbers(numbers_list_string), numbers_list);
    }
}
