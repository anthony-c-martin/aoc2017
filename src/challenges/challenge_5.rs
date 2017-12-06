pub fn execute(input: &str) {
    let result_a = challenge_a(input);
    println!("Challenge 5a: {}", result_a);
    let result_b = challenge_b(input);
    println!("Challenge 5b: {}", result_b);
}

fn challenge_a(input: &str) -> i32 {
    let instructions = parse_input(input);
    count_jumps(instructions, &inc_jump_data_a)
}

fn challenge_b(input: &str) -> i32 {
    let instructions = parse_input(input);
    count_jumps(instructions, &inc_jump_data_b)
}

fn parse_input(input: &str) -> Vec<i32> {
    input.lines().map(|line| line.parse::<i32>().unwrap()).collect()
}

fn inc_jump_data_a(jump_data: &mut [i32], index: usize) {
    jump_data[index] += 1;
}

fn inc_jump_data_b(jump_data: &mut [i32], index: usize) {
    if jump_data[index] >= 3 {
        jump_data[index] -= 1;
    } else {
        jump_data[index] += 1;
    }
}

fn count_jumps(data: Vec<i32>, inc_jump_data: &Fn(&mut [i32], usize)) -> i32 {
    let jump_data_len = data.len() as i32;
    let mut jump_data = data.to_owned();
    let mut index = 0;
    let mut jump_count = 0;

    loop {
        let jump = jump_data[index];
        inc_jump_data(&mut jump_data[..], index);
        jump_count += 1;

        let new_index = index as i32 + jump;
        if new_index < 0 {
            break;
        }

        if new_index >= jump_data_len {
            break;
        }

        index = new_index as usize;
    }

    jump_count
}