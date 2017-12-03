fn to_digit(c: char) -> u32 {
    c.to_digit(10).unwrap()
}

fn to_integer(input: &str) -> i32 {
    input.parse::<i32>().unwrap()
}

pub fn parse_int(input: &str) -> Vec<u32> {
    input.chars().map(to_digit).collect()
}

pub fn parse_tabbed_data(input: &str) -> Vec<i32> {
    input.split_whitespace().map(to_integer).collect()
}

pub fn parse_array_input(input: &str) -> Vec<Vec<i32>> {
    input.lines().map(parse_tabbed_data).collect()
}