use std::collections::{HashMap,HashSet};

pub fn execute(input: &str) {
    let result_a = challenge_a(input);
    println!("Challenge 20a: {}", result_a);
    let result_b = challenge_b(input);
    println!("Challenge 20b: {}", result_b);
}

fn challenge_a(input: &str) -> usize {
    let particles: Vec<_> = input.lines().map(parse_line).collect();

    let distances: Vec<_> = (0..particles.len())
        .into_iter()
        .map(|i| (i, get_dist(&get_coord(&particles[i], 1000))))
        .collect();

    let (mut min_i, mut min_dist) = distances[0];
    for (i, dist) in distances {
        if dist < min_dist {
            min_i = i;
            min_dist = dist;
        }
    }

    min_i
}

fn challenge_b(input: &str) -> usize {
    let lines: Vec<_> = input.lines().collect();
    let mut particles: HashMap<usize, Particle> = (0..lines.len()).map(|i| (i, parse_line(lines[i]))).collect();

    for time in 0..1000 {
        let mut positions = HashMap::new();
        let mut collisions = HashSet::new();

        for (&i, particle) in particles.iter() {
            let coord = get_coord(particle, time);
            let position_key = (coord.x, coord.y, coord.z);
            if positions.contains_key(&position_key) {
                collisions.insert(i);
                collisions.insert(positions[&position_key]);
            } else {
                positions.insert(position_key, i);
            }
        }

        for i in collisions {
            particles.remove(&i);
        }
    }

    particles.len()
}

fn get_coord(particle: &Particle, time: usize) -> Coord {
    Coord {
        x: calc_pos(particle, &|c| c.x, time),
        y: calc_pos(particle, &|c| c.y, time),
        z: calc_pos(particle, &|c| c.z, time),
    }
}

fn calc_pos(particle: &Particle, dimension: &Fn(&Coord) -> i32, time: usize) -> i32 {
    let time = time as i32;
    let position = dimension(&particle.position);
    let velocity = dimension(&particle.velocity);
    let acceleration = dimension(&particle.acceleration);
    position + (time * velocity) + (((time + 1) * time * acceleration) / 2)
}

fn get_dist(coord: &Coord) -> i32 {
    coord.x.abs() + coord.y.abs() + coord.z.abs()
}

fn parse_vector(chars: &[char], index: &mut usize) -> Coord {
    let mut start_index = 0;
    let mut end_index = 0;
    for i in *index..chars.len() {
        match chars[i] {
            '<' => start_index = i + 1,
            '>' => {
                end_index = i;
                break;
            },
            _ => { ; },
        }
    }
    *index = end_index + 1;
    let vals: Vec<_> = chars[start_index..end_index]
        .into_iter().collect::<String>().trim()
        .split(',').map(|s| s.parse::<i32>().unwrap()).collect();

    Coord {
        x: vals[0],
        y: vals[1],
        z: vals[2],
    }
}

fn parse_line(line: &str) -> Particle {
    let mut position = None;
    let mut velocity = None;
    let mut acceleration = None;
    
    let chars: Vec<_> = line.chars().collect();
    for mut i in 0..chars.len() {
        match chars[i] {
            'p' => position = Some(parse_vector(&chars, &mut i)),
            'v' => velocity = Some(parse_vector(&chars, &mut i)),
            'a' => acceleration = Some(parse_vector(&chars, &mut i)),
            _ => { ; }
        }
    }

    Particle {
        position: position.expect("Failed to find position"),
        velocity: velocity.expect("Failed to find velocity"),
        acceleration: acceleration.expect("Failed to find acceleration"),
    }
}

#[derive(Debug)]
struct Particle {
    position: Coord,
    velocity: Coord,
    acceleration: Coord,
}

#[derive(Debug)]
struct Coord {
    x: i32,
    y: i32,
    z: i32,
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(0, challenge_a("p=< 3,0,0>, v=< 2,0,0>, a=<-1,0,0>\np=< 4,0,0>, v=< 0,0,0>, a=<-2,0,0>"));
    }

    #[test]
    fn test_2() {
        assert_eq!(1, challenge_b("p=<-6,0,0>, v=< 3,0,0>, a=< 0,0,0>\np=<-4,0,0>, v=< 2,0,0>, a=< 0,0,0>\np=<-2,0,0>, v=< 1,0,0>, a=< 0,0,0>\np=< 3,0,0>, v=<-1,0,0>, a=< 0,0,0>"));
    }
}