pub fn execute((a_start, b_start): (i32, i32)) {
    let result_a = challenge_a(a_start, b_start);
    println!("Challenge 15a: {}", result_a);
    let result_b = challenge_b(a_start, b_start);
    println!("Challenge 15b: {}", result_b);
}

fn challenge_a(a_start: i32, b_start: i32) -> i32 {
    let mut count = 0;
    let mut a_val = a_start as i64;
    let mut b_val = b_start as i64;
    for _ in 0..40000000 {
        a_val = (a_val * 16807) % 2147483647;
        b_val = (b_val * 48271) % 2147483647;
        if (a_val & 0x0000ffff) == (b_val & 0x0000ffff) {
            count += 1;
        }
    }

    count
}

fn challenge_b(a_start: i32, b_start: i32) -> i32 {
    let mut count = 0;
    let mut a_val = a_start as i64;
    let mut b_val = b_start as i64;
    for _ in 0..5000000 {
        a_val = (a_val * 16807) % 2147483647;
        b_val = (b_val * 48271) % 2147483647;

        while a_val % 4 != 0 {
            a_val = (a_val * 16807) % 2147483647;
        }
        while b_val % 8 != 0 {
            b_val = (b_val * 48271) % 2147483647;
        }

        if (a_val & 0x0000ffff) == (b_val & 0x0000ffff) {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(588, challenge_a(65, 8921));
        assert_eq!(309, challenge_b(65, 8921));
    }
}