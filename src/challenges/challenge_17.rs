pub fn execute(input: i32) {
    let result_a = challenge_a(input);
    println!("Challenge 17a: {}", result_a);
    let result_b = challenge_b(input);
    println!("Challenge 17b: {}", result_b);
}

fn challenge_a(input: i32) -> i32 {
    let mut buffer: Vec<i32> = vec![0];
    let mut next_pos = 0;

    for i in 0..2017 {
        next_pos = ((next_pos + input) % (i + 1)) + 1;
        buffer.insert(next_pos as usize, (i + 1));
    }

    buffer[(next_pos + 1) as usize % buffer.len()]
}

fn challenge_b(input: i32) -> i32 {
    let mut next_pos = 0;
    let mut val_after_zero = 0;
    for i in 0..50_000_000 {
        next_pos = ((next_pos + input) % (i + 1)) + 1;
        
        if next_pos == 1 {
            val_after_zero = i + 1;
        }
    }

    val_after_zero
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(638, challenge_a(3));
    }
}