use utils::*;

pub fn execute(input: &str) {
    let result_a = challenge_a(input);
    println!("Challenge 6a: {}", result_a);
    let result_b = challenge_b(input);
    println!("Challenge 6b: {}", result_b);
}

fn challenge_a(input: &str) -> i32 {
    let mut instructions = parse_tabbed_data(input);
    do_memory_allocation(&mut instructions)
}

fn challenge_b(input: &str) -> i32 {
    let mut instructions = parse_tabbed_data(input);
    do_memory_allocation(&mut instructions);
    do_memory_allocation(&mut instructions)
}

fn do_memory_allocation(instructions: &mut Vec<i32>) -> i32 {
    let mut root_node = TrieNode::new();

    let mut count = 0;
    while !root_node.insert_check_dupe(&instructions) {
        count += 1;

        let mut cur_index = get_max_index(&instructions);

        let value = instructions[cur_index];
        instructions[cur_index] = 0;
        for _ in 0..value {
            cur_index = (cur_index + 1) % 16;
            instructions[cur_index] += 1;
        }
    }

    count
}

fn get_max_index(input: &[i32]) -> usize {
    let mut max_index = 0;
    for i in 1..input.len() {
        if input[i] > input[max_index] {
            max_index = i;
        }
    }

    max_index
}