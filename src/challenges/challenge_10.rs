pub fn execute(input: &str) {
    let result_a = challenge_a(input);
    println!("Challenge 10a: {}", result_a);
    let result_b = challenge_b(input);
    println!("Challenge 10b: {}", result_b);
}

fn get_lengths_a(input: &str) -> Vec<usize> {
    input.split(',').map(|x| x.parse::<usize>().unwrap()).collect()
}

fn challenge_a(input: &str) -> i32 {
    let mut data = get_initial_data();
    do_knot_round(&get_lengths_a(input), &mut data, &mut 0, &mut 0);
    data[0] * data[1]
}

fn challenge_b(input: &str) -> String {
    let mut data = get_initial_data();
    let mut skip_size = 0;
    let mut cur_pos = 0;
    let lengths = get_lengths_b(input);
    for _ in 0..64 {
        do_knot_round(&lengths, &mut data, &mut skip_size, &mut cur_pos);
    }

    let mut output = Vec::new();
    for bucket in 0..16 {
        let mut element = data[bucket * 16];
        for pos in 1..16 {
            element ^= data[(bucket * 16) + pos];
        }

        output.push(element);
    }

    output.iter().map(|x| format!("{:02x}", x)).collect()
}

const CHAR_SIZE: usize = 256;

fn get_initial_data() -> [i32; CHAR_SIZE] {
    let mut data: [i32; CHAR_SIZE] = [0; CHAR_SIZE];
    for i in 0..CHAR_SIZE {
        data[i] = i as i32;
    }

    data
}

fn do_knot_round(lengths: &[usize], data: &mut [i32; CHAR_SIZE], skip_size: &mut usize, cur_pos: &mut usize) {
    for length in lengths {
        for i in 0..(length / 2) {
            let pos_from = (*cur_pos + i) % CHAR_SIZE;
            let pos_to = (*cur_pos + length - 1 - i) % CHAR_SIZE;

            let temp = data[pos_from];
            data[pos_from] = data[pos_to];
            data[pos_to] = temp;
        }

        *cur_pos = (*cur_pos + length + *skip_size) % CHAR_SIZE;
        *skip_size += 1;
    }
}

fn get_lengths_b(input: &str) -> Vec<usize> {
    let mut lengths: Vec<usize> = input.chars().map(|x| x as usize).collect();
    lengths.push(17);
    lengths.push(31);
    lengths.push(73);
    lengths.push(47);
    lengths.push(23);
    lengths
}