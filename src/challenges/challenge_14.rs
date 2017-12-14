pub fn execute(input: &str) {
    let result_a = challenge_a(input);
    println!("Challenge 14a: {}", result_a);
    let result_b = challenge_b(input);
    println!("Challenge 14b: {}", result_b);
}

fn challenge_b(input: &str) -> i32 {
    let mut visited_data = [[false; 128]; 128];
    let disk_data = get_disk_data(input);

    let mut count = 0;
    for i in 0..128 {
        for j in 0..128 {
            if visited_data[i][j] {
                continue;
            }
            visited_data[i][j] = true;

            if !disk_data[i][j] {
                continue;
            }

            count += 1;

            let mut adjacents = vec![(i, j)];
            while let Some((x, y)) = adjacents.pop() {
                let mut next_adjacents = vec![];
                if x > 0 {
                    next_adjacents.push((x - 1, y));
                }
                if x < 127 {
                    next_adjacents.push((x + 1, y));
                }
                if y > 0 {
                    next_adjacents.push((x, y - 1));
                }
                if y < 127 {
                    next_adjacents.push((x, y + 1));
                }

                while let Some((n, m)) = next_adjacents.pop() {
                    if !visited_data[n][m] {
                        visited_data[n][m] = true;
                        if disk_data[n][m] {
                            adjacents.push((n, m));
                        }
                    }
                }
            }
        }
    }

    count
}

fn challenge_a(input: &str) -> i32 {
    let disk_data = get_disk_data(input);

    let mut count = 0;
    for i in 0..128 {
        for j in 0..128 {
            if disk_data[i][j] {
                count += 1;
            }
        }
    }

    count
}

fn get_disk_data(input: &str) -> [[bool; 128]; 128] {
    let mut disk_data = [[false; 128]; 128];
    for i in 0..128 {
        let lengths = get_length_seq(format!("{}-{}", input, i).as_str());
        disk_data[i] = get_knot_hash(&lengths);
    }

    disk_data
}

const CHAR_SIZE: usize = 256;
fn get_initial_data() -> [u8; CHAR_SIZE] {
    let mut data: [u8; CHAR_SIZE] = [0; CHAR_SIZE];
    for i in 0..CHAR_SIZE {
        data[i] = i as u8;
    }

    data
}

fn get_knot_hash(lengths: &[usize]) -> [bool; 128] {
    let mut data = get_initial_data();
    let mut skip_size = 0;
    let mut cur_pos = 0;
    for _ in 0..64 {
        do_knot_round(lengths, &mut data, &mut skip_size, &mut cur_pos);
    }

    let mut output = [0; 16];
    for bucket in 0..16 {
        let mut element = data[bucket * 16];
        for pos in 1..16 {
            element ^= data[(bucket * 16) + pos];
        }

        output[bucket] = element as u8;
    }

    to_bit_array(&output)
}

fn to_bit_array(input: &[u8; 16]) -> [bool; 128] {
    let mut output: [bool; 128] = [false; 128];
    for i in 0..16 {
        let offset = i * 8;
        output[offset + 0] = input[i] & 0b10000000 == 0b10000000;
        output[offset + 1] = input[i] & 0b01000000 == 0b01000000;
        output[offset + 2] = input[i] & 0b00100000 == 0b00100000;
        output[offset + 3] = input[i] & 0b00010000 == 0b00010000;
        output[offset + 4] = input[i] & 0b00001000 == 0b00001000;
        output[offset + 5] = input[i] & 0b00000100 == 0b00000100;
        output[offset + 6] = input[i] & 0b00000010 == 0b00000010;
        output[offset + 7] = input[i] & 0b00000001 == 0b00000001;
    }

    output
}

fn do_knot_round(lengths: &[usize], data: &mut [u8; CHAR_SIZE], skip_size: &mut usize, cur_pos: &mut usize) {
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

fn get_length_seq(input: &str) -> Vec<usize> {
    let mut lengths: Vec<usize> = input.chars().map(|x| x as usize).collect();
    lengths.push(17);
    lengths.push(31);
    lengths.push(73);
    lengths.push(47);
    lengths.push(23);
    lengths
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(8108, challenge_a("flqrgnkx"));
        assert_eq!(1242, challenge_b("flqrgnkx"));
    }
}