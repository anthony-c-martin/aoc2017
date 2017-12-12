#[derive(Debug)]
struct HexCoords {
    n: i32,
    ne: i32,
}

pub fn execute(input: &str) {
    let result_a = challenge_a(input);
    println!("Challenge 11a: {}", result_a);
    let result_b = challenge_b(input);
    println!("Challenge 11b: {}", result_b);
}

fn challenge_a(input: &str) -> i32 {
    let mut coords = HexCoords { n: 0, ne: 0 };
    for direction in input.split(',') {
        update_coords(direction, &mut coords);
    }
    get_distance(&coords)
}

fn challenge_b(input: &str) -> i32 {
    let mut coords = HexCoords { n: 0, ne: 0 };
    let mut max_dist = 0;
    for direction in input.split(',') {
        update_coords(direction, &mut coords);
        max_dist = i32::max(max_dist, get_distance(&coords));
    }

    max_dist
}

fn update_coords(direction: &str, coords: &mut HexCoords) -> i32 {
    match direction {
        "n" => { coords.n += 1; },
        "s" => { coords.n -= 1; },
        "ne" => { coords.ne += 1; },
        "se" => { coords.ne += 1; coords.n -= 1; },
        "nw" => { coords.ne -= 1; coords.n += 1; },
        "sw" => { coords.ne -= 1; },
        _ => panic!("Unexpected instruction")
    }

    get_distance(coords)
}

fn get_distance(coords: &HexCoords) -> i32 {
    let naive_dist = (coords.n + coords.ne).abs();
    if naive_dist > coords.n.abs() && naive_dist > coords.ne.abs() {
        return naive_dist;
    }

    if coords.n.abs() > coords.ne.abs() {
        return coords.n.abs();
    }

    coords.ne.abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(3, challenge_a("ne,ne,ne"));
    }

    #[test]
    fn test_2() {
        assert_eq!(0, challenge_a("ne,ne,sw,sw"));
    }

    #[test]
    fn test_3() {
        assert_eq!(2, challenge_a("ne,ne,s,s"));
    }

    #[test]
    fn test_4() {
        assert_eq!(3, challenge_a("se,sw,se,sw,sw"));
    }
}