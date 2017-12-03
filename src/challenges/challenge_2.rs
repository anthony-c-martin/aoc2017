use utils;

pub fn execute(input: &str) {
    let result_a = challenge_a(input);
    println!("Challenge 2a: {}", result_a);
    let result_b = challenge_b(input);
    println!("Challenge 2b: {}", result_b);
}

fn challenge_a(input: &str) -> i32 {
    let int_arr = utils::parse_array_input(input);
    int_arr.iter().map(calc_checksum).fold(0, |acc, x| acc + x)
}

fn challenge_b(input: &str) -> i32 {
    let int_arr = utils::parse_array_input(input);
    int_arr.iter().map(|line| calc_divisibles(line).unwrap()).fold(0, |acc, x| acc + x)
}

fn calc_checksum(line: &Vec<i32>) -> i32 {
    if line.len() == 0 {
        return 0;
    }

    let mut min_val = line[0];
    let mut max_val = line[0];

    for i in 1..line.len() {
        if line[i] < min_val {
            min_val = line[i];
        }
        if line[i] > max_val {
            max_val = line[i];
        }
    }

    return max_val - min_val;
}

fn calc_divisibles(line: &Vec<i32>) -> Result<i32, String> {
    if line.len() == 0 {
        return Ok(0);
    }

    for i in 0..line.len() {
        for j in i + 1..line.len() {
            let larger = if line[i] > line[j] { line[i] } else { line[j] };
            let smaller = if line[i] > line[j] { line[j] } else { line[i] };

            let self_result = if smaller > 0 { (larger / smaller) * smaller } else { 0 };

            if self_result == larger {
                return Ok(larger / smaller);
            }
        }
    }

    return Err(String::from("Did not find result"));
}