use std::collections::HashMap;
use std::fmt;
use std::cmp;

pub fn execute(input: &str) {
    let result_a = challenge_a(input);
    println!("Challenge 21a: {}", result_a);
    let result_b = challenge_b(input);
    println!("Challenge 21b: {}", result_b);
}

fn challenge_a(input: &str) -> usize {
    get_pixel_count(input, 5)
}

fn challenge_b(input: &str) -> usize {
    get_pixel_count(input, 18)
}

fn get_pixel_count(input: &str, iterations: usize) -> usize {
    let rules: Vec<_> = input.lines().map(|line| {
        let split: Vec<_> = line.split(" => ").collect();
        let from = Image::new(split[0]);
        let to = Image::new(split[1]);
        (from, to)
    }).collect();

    let mut rule_map = HashMap::new();
    for (from, to) in rules {
        for from in from.get_permutations() {
            if !rule_map.contains_key(&from) {
                rule_map.insert(from, to.clone());
            }
        }
    }

    let mut image = Image::new(".#./..#/###");

    for _ in 0..iterations {
        image.apply_rules(&rule_map);
    }

    image.count_lit_pixels()
}

#[derive(Clone, Eq, Hash)]
struct Image {
    data: Vec<Vec<bool>>,
}

impl Image {
    fn new(data: &str) -> Image {
        let data = data.split('/').map(|l| l.chars().map(|c| c == '#').collect()).collect();

        Image {
            data,
        }
    }

    fn get_cells(&self) -> Vec<Vec<Image>> {
        match self.data.len() {
            width if width % 2 == 0 => {
                (0..(width / 2)).map(|x| {
                    (0..(width / 2)).map(|y| {
                        Image { data: vec![
                            vec![
                                self.data[x * 2][y * 2],
                                self.data[(x * 2) + 1][y * 2],
                            ],
                            vec![
                                self.data[x * 2][(y * 2) + 1],
                                self.data[(x * 2) + 1][(y * 2) + 1],
                            ],
                        ]}
                    }).collect()
                }).collect()
            },
            width if width % 3 == 0 => {
                (0..(width / 3)).map(|x| {
                    (0..(width / 3)).map(|y| {
                        Image { data: vec![
                            vec![
                                self.data[x * 3][y * 3],
                                self.data[(x * 3) + 1][y * 3],
                                self.data[(x * 3) + 2][y * 3],
                            ],
                            vec![
                                self.data[x * 3][(y * 3) + 1],
                                self.data[(x * 3) + 1][(y * 3) + 1],
                                self.data[(x * 3) + 2][(y * 3) + 1],
                            ],
                            vec![
                                self.data[x * 3][(y * 3) + 2],
                                self.data[(x * 3) + 1][(y * 3) + 2],
                                self.data[(x * 3) + 2][(y * 3) + 2],
                            ],
                        ]}
                    }).collect()
                }).collect()
            },
            _ => panic!("oh no!!!"),
        }
    }

    fn apply_rules(&mut self, rule_map: &HashMap<Image, Image>) {
        let cells = self.get_cells();
        let width = cells.len();

        let subimages: Vec<Vec<_>> = (0..width).map(|x| {
            (0..width).map(|y| {
                rule_map[&cells[x][y]].clone()
            }).collect()
        }).collect();

        let cell_width = subimages[0][0].data.len();

        self.data = vec![vec![false; width * cell_width]; width * cell_width];
        for x in 0..(width * cell_width) {
            for y in 0..(width * cell_width) {
                self.data[x][y] = subimages[x / cell_width][y / cell_width].data[x % cell_width][y % cell_width];
            }
        }
    }

    fn count_lit_pixels(&self) -> usize {
        self.data.iter().fold(0, |acc, a| {
            acc + a.iter().fold(0, |acc, &b| acc + if b { 1 } else { 0 })
        })
    }

    fn get_permutations(&self) -> Vec<Image> {
        let mut permutations = vec![];
        let mut image = self.clone();

        for _ in 0..4 {
            permutations.push(image.clone());
            image.rotate_90();
        }

        image.flip();
        for _ in 0..4 {
            permutations.push(image.clone());
            image.rotate_90();
        }

        image.rotate_90();
        image.flip();
        for _ in 0..4 {
            permutations.push(image.clone());
            image.rotate_90();
        }

        permutations
    }

    fn rotate_90(&mut self) {
        let width = self.data.len();
        let mut data = vec![vec![false; width]; width];
        for i in 0..width {
            for j in 0..width {
                data[i][j] = self.data[width - j - 1][i];
            }
        }

        self.data = data;
    }

    fn flip(&mut self) {
        let width = self.data.len();
        let mut data = vec![vec![false; width]; width];
        for i in 0..width {
            for j in 0..width {
                data[i][j] = self.data[i][width - j - 1];
            }
        }

        self.data = data;
    }
}

impl cmp::PartialEq for Image {
    fn eq(&self, other: &Image) -> bool {
        self.data == other.data
    }

    fn ne(&self, other: &Image) -> bool {
        !self.eq(other)
    }
}

impl fmt::Debug for Image {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let output = self.data.iter().map(|line| {
                line.iter().map(|&c| if c { '#' } else { '.' }).collect::<String>()
            })
            .collect::<Vec<String>>().join("\n");

        write!(f, "{}", output)
    }
}