use utils;

pub fn execute(input: &str) {
    let result_a = challenge_a(input);
    println!("Challenge 1a: {}", result_a);
    let result_b = challenge_b(input);
    println!("Challenge 1b: {}", result_b);
}

fn challenge_a(input: &str) -> u32 {
    let mut sum = 0;
    let int_arr = utils::parse_int(input);
    let length = int_arr.len();

    for cur_index in 0..length {
        let prev_index = (cur_index + length - 1) % length;

        if int_arr[cur_index] == int_arr[prev_index] {
            sum += int_arr[cur_index];
        }
    }

    sum
}

fn challenge_b(input: &str) -> u32 {
    let mut sum = 0;
    let int_arr = utils::parse_int(input);
    let half_length = int_arr.len() / 2;

    for cur_index in 0..half_length {
        let prev_index = half_length + cur_index;

        if int_arr[cur_index] == int_arr[prev_index] {
            sum += 2 * int_arr[cur_index];
        }
    }

    sum
}