pub fn execute(input: i32) {
    let result_a = challenge_a(input);
    println!("Challenge 3a: {}", result_a);
    let result_b = challenge_b(input);
    println!("Challenge 3b: {}", result_b);
}

fn challenge_a(input: i32) -> i32 {
    let coords = get_coords(input);
    coords.x.abs() + coords.y.abs()
}

fn challenge_b(input: i32) -> i32 {
    let size = 9;
    let offset = size / 2;
    let mut xs: [[i32; 9]; 9] = [[0; 9]; 9];

    xs[offset as usize][offset as usize] = 1;
    for i in 2..(size * size) {
        let coords = get_coords(i);
        let x_coord = (offset + coords.x) as usize;
        let y_coord = (offset + coords.y) as usize;

        xs[x_coord][y_coord] = get_adjacent(&xs, x_coord, y_coord).iter().fold(0, |acc, x| acc + x);

        if xs[x_coord][y_coord] > input {
            return xs[x_coord][y_coord];
        }
    }

    panic!("Failed to find value");
}

fn get_adjacent(data: &[[i32; 9]; 9], x: usize, y: usize) -> Vec<i32> {
    let mut vec: Vec<i32> = Vec::new();
    if x > 0 { vec.push(data[x - 1][y]); }
    if x < 8 { vec.push(data[x + 1][y]); }
    if y > 0 { vec.push(data[x][y - 1]); }
    if y < 8 { vec.push(data[x][y + 1]); }
    if x > 0 && y > 0 { vec.push(data[x - 1][y - 1]); }
    if x > 0 && y < 8 { vec.push(data[x - 1][y + 1]); }
    if x < 8 && y > 0 { vec.push(data[x + 1][y - 1]); }
    if x < 8 && y < 8 { vec.push(data[x + 1][y + 1]); }

    vec
}

struct Coords {
    x: i32,
    y: i32,
}

fn get_coords_in_square(square_size: i32, relative_val: i32) -> Coords {
    if square_size == 0 {
        return Coords { x: 0, y: 0 }
    }

    let side = (relative_val / square_size) % 4;
    let val_offset = relative_val % square_size;
    let edge_offset = square_size / 2;

    match side {
        0 => Coords { x: edge_offset, y: val_offset - edge_offset },
        1 => Coords { x: edge_offset - val_offset, y: edge_offset },
        2 => Coords { x: -edge_offset, y: edge_offset - val_offset },
        3 => Coords { x: val_offset - edge_offset, y: -edge_offset },
        _ => panic!("Invalid value for side")
    }
}

fn get_coords(val: i32) -> Coords {
    let mut relative_val = val;
    let mut square_size = 0;
    let mut max_val = 1;
    while relative_val > max_val {
        relative_val -= max_val;
        square_size += 2;
        max_val = square_size * 4;
    }

    get_coords_in_square(square_size, relative_val)
}