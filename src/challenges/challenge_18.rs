use std::collections::{HashMap,VecDeque};
use std::cell::{Cell, RefCell};

pub fn execute(input: &str) {
    let result_a = challenge_a(input);
    println!("Challenge 18a: {}", result_a);
    let result_b = challenge_b(input);
    println!("Challenge 18b: {}", result_b);
}

#[derive(Debug, Copy, Clone)]
enum Value {
    Register(char),
    Constant(i32),
}

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Send(Value),
    Set(Value, Value),
    Add(Value, Value),
    Multiply(Value, Value),
    Modulo(Value, Value),
    Receive(Value),
    Jump(Value, Value),
}

#[derive(Debug)]
struct Program {
    instructions: Vec<Instruction>,
    state: ProgramState,
}

#[derive(Debug, Clone, PartialEq)]
struct ProgramState {
    id: i32,
    registers: HashMap<char, i64>,
    position: usize
}

impl Program {
    fn new(id: i32, instructions: &[Instruction]) -> Program {
        Program {
            instructions: instructions.to_vec(),
            state: ProgramState {
                id,
                registers: [('p', id as i64)].iter().cloned().collect(),
                position: 0,
            },
        }
    }

    fn advance(&mut self, read: &mut FnMut() -> Option<i64>, write: &mut FnMut(i64)) -> bool {
        if self.state.position >= self.instructions.len() {
            return false;
        }

        let mut advance_by = 1;
        match { self.instructions[self.state.position] } {
            Instruction::Set(ref reg_val, ref from_val) => self.update_val(reg_val, from_val, &|(_, f)| f),
            Instruction::Add(ref reg_val, ref from_val) => self.update_val(reg_val, from_val, &|(r, f)| r + f),
            Instruction::Multiply(ref reg_val, ref from_val) => self.update_val(reg_val, from_val, &|(r, f)| r * f),
            Instruction::Modulo(ref reg_val, ref from_val) => self.update_val(reg_val, from_val, &|(r, f)| r % f),
            Instruction::Send(ref reg_val) => write(self.get_val(reg_val)),
            Instruction::Receive(ref reg_val) => match read() {
                Some(val) => *self.get_val_mut(reg_val) = val,
                None => return false,
            },
            Instruction::Jump(ref reg_val, ref jump_val) => {
                if self.get_val(reg_val) > 0 {
                    advance_by = self.get_val(jump_val);
                }
            },
        }

        self.state.position = (self.state.position as i64 + advance_by) as usize;
        true
    }

    fn update_val(&mut self, to_val: &Value, from_val: &Value, mut_fn: &Fn((i64, i64)) -> i64) {
        let from = self.get_val(from_val);
        {
            let to = self.get_val_mut(to_val);
            *to = mut_fn((*to, from));
        }
    }

    fn get_val(&mut self, input: &Value) -> i64 {
        match *input {
            Value::Constant(val) => val as i64,
            Value::Register(name) => *self.state.registers.entry(name).or_insert(0),
        }
    }

    fn get_val_mut(&mut self, input: &Value) -> &mut i64 {
        match *input {
            Value::Register(name) => self.state.registers.entry(name).or_insert(0),
            _ => panic!("{:?} cannot be mutated", *input),
        }
    }
}

fn challenge_a(input: &str) -> i64 {
    let instructions: Vec<_> = input.lines().map(parse_line).collect();
    let mut program = Program::new(0, &instructions);

    let last_sound = Cell::new(0);
    let read_non_zero = Cell::new(false);

    let mut read = || {
        let sound = last_sound.get();
        if sound > 0 {
            read_non_zero.set(true);
        }
        Some(sound)
    };
    let mut write = |val| last_sound.set(val);
    while program.advance(&mut read, &mut write) {
        if read_non_zero.get() {
            break;
        }
    }
    
    last_sound.get()
}

fn challenge_b(input: &str) -> i32 {
    let instructions: Vec<_> = input.lines().map(parse_line).collect();
    let mut program_0 = Program::new(0, &instructions);
    let mut program_1 = Program::new(1, &instructions);
    let program_1_snd_count = Cell::new(0);

    let buf_0 = RefCell::new(VecDeque::new());
    let buf_1 = RefCell::new(VecDeque::new());

    let mut read_0 = || buf_0.borrow_mut().pop_front();
    let mut read_1 = || buf_1.borrow_mut().pop_front();
    let mut write_0 = |val| buf_1.borrow_mut().push_back(val);
    let mut write_1 = |val| {
        program_1_snd_count.set(program_1_snd_count.get() + 1);
        buf_0.borrow_mut().push_back(val);
    };

    loop {
        let prog_0_running = program_0.advance(&mut read_0, &mut write_0);
        let prog_1_running = program_1.advance(&mut read_1, &mut write_1);

        if !prog_0_running && !prog_1_running {
            break;
        }
    }

    program_1_snd_count.get()
}

fn parse_value(input: &str) -> Value {
    match input.parse::<i32>() {
        Ok(val) => Value::Constant(val),
        Err(_) => Value::Register(input.chars().nth(0).unwrap()),
    }
}

fn parse_line(line: &str) -> Instruction {
    let split: Vec<_> = line.split(' ').collect();
    match split[0] {
        "snd" => Instruction::Send(parse_value(split[1])),
        "set" => Instruction::Set(parse_value(split[1]), parse_value(split[2])),
        "add" => Instruction::Add(parse_value(split[1]), parse_value(split[2])),
        "mul" => Instruction::Multiply(parse_value(split[1]), parse_value(split[2])),
        "mod" => Instruction::Modulo(parse_value(split[1]), parse_value(split[2])),
        "rcv" => Instruction::Receive(parse_value(split[1])),
        "jgz" => Instruction::Jump(parse_value(split[1]), parse_value(split[2])),
        _ => panic!("Unexpected instruction {}", line),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(3, challenge_b("snd 1\nsnd 2\nsnd p\nrcv a\nrcv b\nrcv c\nrcv d"));
    }
}