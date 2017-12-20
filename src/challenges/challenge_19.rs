pub fn execute(input: &str) {
    let result_a = challenge_a(input);
    println!("Challenge 19a: {}", result_a);
    let result_b = challenge_b(input);
    println!("Challenge 19b: {}", result_b);
}

fn challenge_a(input: &str) -> String {
    let grid = LineGrid::new(input);
    grid.filter_map(|e| e.letter).collect()
}

fn challenge_b(input: &str) -> usize {
    let grid = LineGrid::new(input);
    grid.count()
}

impl LineGrid {
    pub fn new(input: &str) -> LineGrid {
        let grid_array: Vec<_> = input.lines().map(|l| {
                l.chars().collect::<Vec<_>>()
            }).collect();
        let x_start = grid_array[0].iter().position(|&v| v == '|').unwrap();
        
        LineGrid {
            iter_started: false,
            grid_array,
            cur_element: LineGridElement {
                x: x_start,
                y: 0,
                direction: Direction::Down,
                letter: None,
            },
        }
    }

    fn next_elem(&self, (x, y): (usize, usize), direction: Direction) -> Option<(usize, usize)> {
        match direction {
            Direction::Down if y + 1 < self.grid_array.len() => Some((x, y + 1)),
            Direction::Up if y > 0 => Some((x, y - 1)),
            Direction::Left if x > 0 => Some((x - 1, y)),
            Direction::Right if x + 1 < self.grid_array[y].len() => Some((x + 1, y)),
            _ => None,
        }
    }

    fn find_direction(&self, coords: (usize, usize), direction: Direction) -> Direction {
        let possible_directions = match direction {
            Direction::Right => &[Direction::Right, Direction::Down, Direction::Up],
            Direction::Down => &[Direction::Right, Direction::Down, Direction::Left],
            Direction::Left => &[Direction::Down, Direction::Left, Direction::Up],
            Direction::Up => &[Direction::Right, Direction::Left, Direction::Up],
        };
        
        for &direction in possible_directions.into_iter() {
            if let Some((x, y)) = self.next_elem(coords, direction) {
                return match self.grid_array[y][x] {
                    ' ' =>  continue,
                    _ => direction.clone(),
                };
            }
        }

        panic!("Failed to find direction at ({}, {}) going {:?}", coords.0, coords.1, direction);
    }

    fn get_next_elem(&self) -> Option<LineGridElement> {
        let (x, y, direction) = (self.cur_element.x, self.cur_element.y, self.cur_element.direction);

        let (x, y) = match self.next_elem((x, y), direction) {
            None => return None,
            Some(coords) => coords,
        };

        let direction = match (self.grid_array[y][x], direction) {
            ('|', _) => direction,
            ('-', _) => direction,
            ('+', _) => self.find_direction((x, y), direction),
            (v, _)  if char::is_alphanumeric(v) => direction,
            (' ', _) => return None,
            (c, _) => panic!("Unexpected element '{}' at ({}, {}) going {:?}", c, x, y, direction),
        };

        let letter = match self.grid_array[y][x] {
            c if char::is_alphanumeric(c) => Some(c),
            _ => None,
        };

        Some(LineGridElement {
            x,
            y,
            direction,
            letter,
        })
    }
}

impl Iterator for LineGrid {
    type Item = LineGridElement;

    fn next(&mut self) -> Option<LineGridElement> {
        if !self.iter_started {
            self.iter_started = true;
            return Some(self.cur_element);
        }

        while let Some(elem) = self.get_next_elem() {
            self.cur_element = elem;
            return Some(elem);
        }
        
        None
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

#[derive(Debug)]
struct LineGrid {
    iter_started: bool,
    cur_element: LineGridElement,
    grid_array: Vec<Vec<char>>,
}

#[derive(Debug, Clone, Copy)]
struct LineGridElement {
    x: usize,
    y: usize,
    direction: Direction,
    letter: Option<char>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!("ABCDEF", challenge_a("     |          \n     |  +--+    \n     A  |  C    \n F---|----E|--+ \n     |  |  |  D \n     +B-+  +--+ \n"));
    }
}