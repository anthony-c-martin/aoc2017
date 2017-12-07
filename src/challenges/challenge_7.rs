use std::str;
use std::iter;

pub fn execute(input: &str) {
    let result_a = challenge_a(input);
    println!("Challenge 6a: {}", result_a);
}

fn challenge_a(input: &str) -> i32 {
    let mut programs = input.lines().map(get_program);
    for program in programs {
        info!("Got Program {:?}", program);
    }
    0
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
struct Program {
    name: String,
    weight: i32,
    programs: Vec<String>,
}

fn get_program(input: &str) -> Program {
    let mut tokenizer = Tokenizer::new(input).peekable();
    let mut name = None;
    let mut weight = None;
    let mut programs = Vec::new();

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
                programs.push(text);
            }
            (Some(Token::Text(text)), None) => {
                programs.push(text);
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

    Program {
        name: name.unwrap(),
        weight: weight.unwrap(),
        programs
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