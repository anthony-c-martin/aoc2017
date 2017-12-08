use std::str;
use std::iter;
use std::collections::HashSet;
use std::collections::HashMap;

pub fn execute(input: &str) {
    let result_a = challenge_a(input);
    println!("Challenge 7a: {}", result_a);
    let result_b = challenge_b(input);
    println!("Challenge 7b: {}", result_b);
}

fn challenge_a(input: &str) -> String {
    let programs = input.lines().map(get_program_data).collect();
    find_root_node_name(&programs).unwrap()
}

fn challenge_b(input: &str) -> i32 {
    let programs = input.lines().map(get_program_data).collect();
    get_missing_weight_diff(&programs).unwrap()
}

fn find_root_node_name(programs: &Vec<ProgramData>) -> Option<String> {
    let mut all_children = HashSet::new();
    for program in programs.iter() {
        for child in program.children.iter() {
            all_children.insert(child);
        }
    }

    for program in programs.iter() {
        if !all_children.contains(&program.name) {
            return Some(program.name.clone());
        }
    }

    None
}

fn get_missing_weight_diff(programs: &Vec<ProgramData>) -> Option<i32> {
    let root_node_name = find_root_node_name(programs).unwrap();

    let mut program_map = HashMap::new();
    for program in programs {
        program_map.insert(program.name.clone(), program);
    }

    let mut weights_map = HashMap::new();
    get_missing_weight_diff_recurse(&program_map, &mut weights_map, &root_node_name, 0)
}

fn get_missing_weight_diff_recurse(programs: &HashMap<String, &ProgramData>, weights: &mut HashMap<String, i32>, program_name: &String, depth: i32) -> Option<i32> {
    let program = programs.get(program_name).unwrap();

    if program.children.is_empty() {
        weights.insert(program.name.clone(), program.weight);
        debug!("{}{}: {}", "  ".repeat(depth as usize), program.name, program.weight);
        return None;
    }

    let mut idx = 0;
    let mut weight1_idx = 0;
    let mut weight1 = 0;
    let mut weight1_count = 0;
    let mut weight2_idx = 0;
    let mut weight2 = 0;
    let mut weight2_count = 0;
    let mut weights_sum = program.weight;
    for child in &program.children {
        if !weights.contains_key(child) {
            match get_missing_weight_diff_recurse(programs, weights, child, depth + 1) {
                Some(weight) => return Some(weight),
                None => { ; },
            }
        }

        let &cur_weight = weights.get(child).unwrap();

        if weight1_count == 0 {
            weight1 = cur_weight;
            weight1_count += 1;
            weight1_idx = 0;
        } else if cur_weight == weight1 {
            weight1_count += 1;
        } else if weight2_count == 0 {
            weight2 = cur_weight;
            weight2_count += 1;
            weight2_idx = idx;
        } else if cur_weight == weight2 {
            weight2_count += 1;
        } else {
            panic!("Unable to find odd one out");
        }

        weights_sum += cur_weight;
        idx += 1;
    }

    weights.insert(program.name.clone(), weights_sum);
    debug!("{}{}({}): {}", "  ".repeat(depth as usize), program.name, program.weight, weights_sum);

    if weight1_count > 0 && weight2_count > 0 {
        if weight1_count == 1 && weight2_count > 1 {
            let bad_child = &program.children[weight1_idx];
            return Some(programs.get(bad_child).unwrap().weight + weight2 - weight1);
        }

        if weight2_count == 1 && weight1_count > 1 {
            let bad_child = &program.children[weight2_idx];
            return Some(programs.get(bad_child).unwrap().weight + weight1 - weight2);
        }

        panic!("Unable to find odd one out");
    }

    None
}

#[derive(Debug)]
enum Token {
    Arrow,
    OpenParen,
    CloseParen,
    Comma,
    Number(i32),
    Text(String)
}

#[derive(Debug)]
struct ProgramData {
    name: String,
    weight: i32,
    children: Vec<String>,
}

fn get_program_data(input: &str) -> ProgramData {
    let mut tokenizer = Tokenizer::new(input).peekable();
    let mut name = None;
    let mut weight = None;
    let mut children = Vec::new();

    loop {
        if name == None {
            match (tokenizer.next(), tokenizer.peek()) {
                (Some(Token::Text(text)), Some(&Token::OpenParen)) => {
                    name = Some(text);
                }
                (a, b) => panic!("{:?}, {:?}", a.unwrap(), b.unwrap()),
            }
        }

        match (tokenizer.next(), tokenizer.peek()) {
            (None, _) => break,
            (Some(Token::OpenParen), Some(&Token::Number(_))) => { ; },
            (Some(Token::CloseParen), Some(&Token::Arrow)) => { ; },
            (Some(Token::CloseParen), None) => break,
            (Some(Token::Text(text)), Some(&Token::Comma)) => {
                children.push(text);
            }
            (Some(Token::Text(text)), None) => {
                children.push(text);
                break;
            }
            (Some(Token::Number(number)), Some(&Token::CloseParen)) => {
                weight = Some(number);
            },
            (Some(Token::Comma), _) => { ; },
            (Some(Token::Arrow), _) => { ; },
            (a, None) => panic!("Unexpected {:?} followed by None", a.unwrap()),
            (a, b) => panic!("Unexpected {:?} followed by {:?}", a.unwrap(), b.unwrap()),
        }
    }

    ProgramData {
        name: name.unwrap(),
        weight: weight.unwrap(),
        children
    }
}

fn is_digit(input: char) -> bool {
    input.is_digit(10)
}

struct Tokenizer<'a> {
    input: iter::Peekable<str::Chars<'a>>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &str) -> Tokenizer {
        Tokenizer{
            input: input.chars().peekable(),
        }
    }

    fn read_number(&mut self, first_char: char) -> i32 {
        let int_string = self.read_string_with_cond(first_char, &is_digit);
        int_string.parse().unwrap()
    }

    fn read_string(&mut self, first_char: char) -> String {
        self.read_string_with_cond(first_char, &char::is_alphanumeric)
    }

    fn read_string_with_cond(&mut self, first_char: char, include_char: &Fn(char) -> bool) -> String {
        let mut output = first_char.to_string();
        loop {
            match self.input.peek() {
                Some(&c) if include_char(c) => output.push(c),
                _ => break,
            }

            self.input.next();
        }

        output.parse().unwrap()
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        loop {
            let cur_char = match self.input.peek() {
                Some(c) => *c,
                None => return None,
            };
            self.input.next();

            let token = match cur_char {
                c if c.is_whitespace() => continue,
                '(' => Some(Token::OpenParen),
                ')' => Some(Token::CloseParen),
                ',' => Some(Token::Comma),
                '-' => match self.input.peek() {
                    Some(&'>') => {
                        self.input.next();
                        Some(Token::Arrow)
                    }
                    _ => panic!("Expected >"),
                },
                c if is_digit(c) => Some(Token::Number(self.read_number(cur_char))),
                c if c.is_alphanumeric() => Some(Token::Text(self.read_string(cur_char))),
                _ => panic!("Got unexpected char"),
            };

            return token;
        }
    }
}