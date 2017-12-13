pub fn execute(input: &str) {
    let result_a = challenge_a(input);
    println!("Challenge 13a: {}", result_a);
    let result_b = challenge_b(input);
    println!("Challenge 12b: {}", result_b);
}

fn get_layer(line: &str) -> (i32, i32) {
    let mut values_it = line.split(": ").map(|val| val.parse::<i32>().unwrap());
    (values_it.next().unwrap(), values_it.next().unwrap())
}

fn challenge_a(input: &str) -> i32 {
    let layers: Vec<(i32, i32)> = input.lines().map(get_layer).collect();
    let (_, severity) = get_severity(&layers, 0);

    severity
}

fn get_severity(layers: &Vec<(i32, i32)>, start_time: i32) -> (bool, i32) {
    let mut severity = 0;
    let mut is_caught = false;
    for &(depth, range) in layers.iter() {
        if (start_time + depth) % (2 * (range - 1)) == 0 {
            severity += depth * range;
            is_caught = true;
        }
    }

    (is_caught, severity)
}

fn challenge_b(input: &str) -> i32 {
    let layers: Vec<(i32, i32)> = input.lines().map(get_layer).collect();
    let mut start_time = 0;

    loop {
        let (is_caught, _) = get_severity(&layers, start_time);
        if !is_caught {
            return start_time;
        }

        start_time += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(24, challenge_a("0: 3\n1: 2\n4: 4\n6: 4"));
    }
}