pub fn execute(input: &str) {
    let result_a = challenge_a(input, "abcdefghijklmnop");
    println!("Challenge 16a: {}", result_a);
    let result_b = challenge_b(input, "abcdefghijklmnop");
    println!("Challenge 16b: {}", result_b);
}

fn challenge_a(input: &str, programs: &str) -> String {
    let moves: Vec<_> = input.split(',').map(parse_move).collect();
    let mut names: Vec<_> = programs.chars().collect();
    for dancemove in &moves {
        do_move(&mut names, dancemove);
    }
    
    names.iter().collect()
}

fn challenge_b(input: &str, programs: &str) -> String {
    let moves: Vec<_> = input.split(',').map(parse_move).collect();
    let input_vec: Vec<_> = programs.chars().collect();
    let mut names: Vec<_> = programs.chars().collect();
    
    let mut vals = vec![input_vec.clone()];
    loop {
        for dancemove in &moves {
            do_move(&mut names, dancemove);
        }

        if names == vals[0] {
            break;
        }

        vals.push(names.clone());
    }

    vals[1000000000 % vals.len()].iter().collect()
}

#[derive(Debug)]
enum DanceMove {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

fn do_move(input: &mut [char], dancemove: &DanceMove) {
    match dancemove {
        &DanceMove::Spin(spin) => {
            rotate(input);
            rotate(&mut input[..spin]);
            rotate(&mut input[spin..]);
        },
        &DanceMove::Exchange(pos1, pos2) => {
            let val1 = input[pos1];
            input[pos1] = input[pos2];
            input[pos2] = val1;
        },
        &DanceMove::Partner(name1, name2) => {
            let pos1 = input.iter().position(|&x| x == name1).unwrap();
            let pos2 = input.iter().position(|&x| x == name2).unwrap();

            let val1 = input[pos1];
            input[pos1] = input[pos2];
            input[pos2] = val1;
        }
    }
}

fn rotate(input: &mut [char]) {
    let length = input.len();
    for i in 0..(length / 2) {
        let temp = input[i];
        input[i] = input[length - i - 1];
        input[length - i - 1] = temp;
    }
}

fn parse_move(input: &str) -> DanceMove {
    let first_char = input.chars().nth(0);
    let after_first = &input[1..];

    match first_char.unwrap() {
        's' => DanceMove::Spin(after_first.parse().unwrap()),
        'x' => {
            let split: Vec<_> = after_first.split('/').collect();
            DanceMove::Exchange(split[0].parse().unwrap(), split[1].parse().unwrap())
        },
        'p' => {
            let split: Vec<_> = after_first.split('/').collect();
            DanceMove::Partner(split[0].parse().unwrap(), split[1].parse().unwrap())
        },
        _ => panic!("Unexpected input {}", input),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!("eabcd", challenge_a("s1", "abcde"));
        assert_eq!("eabdc", challenge_a("s1,x3/4", "abcde"));
        assert_eq!("baedc", challenge_a("s1,x3/4,pe/b", "abcde"));
    }

    #[test]
    fn test_2() {
        assert_eq!("pabcdefghijklmno", challenge_a("s1", "abcdefghijklmnop"));
        assert_eq!("bcdefghijklmnopa", challenge_a("s15", "abcdefghijklmnop"));
        assert_eq!("jklmnopabcdefghi", challenge_a("s7", "abcdefghijklmnop"));
        assert_eq!("mnopabcdefghijkl", challenge_a("s4", "abcdefghijklmnop"));
        assert_eq!("efghijklmnopabcd", challenge_a("s12", "abcdefghijklmnop"));
    }
}