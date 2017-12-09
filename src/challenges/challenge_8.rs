use std::collections::HashMap;
use std::cmp;

pub fn execute(input: &str) {
    let (max_at_end, max_during_ops) = challenge_a(input);
    println!("Challenge 8a: {}", max_at_end);
    println!("Challenge 8b: {}", max_during_ops);
}

fn challenge_a(input: &str) -> (i32, i32) {
    let registers = &mut HashMap::new();

    let instructions: Vec<Instruction> = input.lines().map(|line| get_instruction(line, registers).unwrap()).collect();

    let mut max_during_ops = 0;
    for instruction in instructions {
        let condition;
        {
            let cmp_register = registers.get(&instruction.comparison_register).unwrap();
            condition = match instruction.comparison {
                Comparison::Equals(val) => cmp_register.value == val,
                Comparison::NotEquals(val) => cmp_register.value != val,
                Comparison::LessThan(val) => cmp_register.value < val,
                Comparison::GreaterThan(val) => cmp_register.value > val,
                Comparison::LessThanOrEquals(val) => cmp_register.value <= val,
                Comparison::GreaterThanOrEquals(val) => cmp_register.value >= val,
            };
        }

        if condition {
            let register = registers.get_mut(&instruction.register).unwrap();
            register.value += instruction.increment;
            max_during_ops = cmp::max(max_during_ops, register.value);
        }
    }

    let max_at_end = registers.iter()
        .max_by_key(|&(_, reg)| reg.value)
        .map(|(_, reg)| reg.value)
        .unwrap();

    (max_at_end, max_during_ops)
}

fn get_instruction(input: &str, registers: &mut HashMap<String, Register>) -> Result<Instruction, &'static str> {
    let tokens: Vec<&str> = input.split_whitespace().collect();

    if tokens.len() != 7 {
        return Err("Unexpected instruction");
    }

    let register = tokens[0].to_string();
    if !registers.contains_key(&register) {
        registers.insert(register.clone(), Register {
            name: tokens[0].to_string(),
            value: 0
        });
    }

    let multiplier = match tokens[1] {
        "inc" => 1,
        "dec" => -1,
        _ => return Err("Unexpected token[1]"),
    };

    let increment = match tokens[2].parse::<i32>() {
        Ok(val) => multiplier * val,
        _ => return Err("Unexpected token[2]"),
    };

    match tokens[3] {
        "if" => { ; },
        _ => return Err("Unexpected token[3]"),
    }

    let comparison_register = tokens[4].to_string();
    if !registers.contains_key(&comparison_register) {
        registers.insert(comparison_register.clone(), Register {
            name: tokens[4].to_string(),
            value: 0
        });
    }

    let cmp_val = match tokens[6].parse::<i32>() {
        Ok(val) => val,
        _ => return Err("Unexpected token[6]"),
    };

    let comparison = match tokens[5] {
        "==" => Comparison::Equals(cmp_val),
        "!=" => Comparison::NotEquals(cmp_val),
        "<" => Comparison::LessThan(cmp_val),
        ">" => Comparison::GreaterThan(cmp_val),
        "<=" => Comparison::LessThanOrEquals(cmp_val),
        ">=" => Comparison::GreaterThanOrEquals(cmp_val),
        _ => return Err("Unexpected token[5]"),
    };

    Ok(Instruction {
        register,
        increment,
        comparison,
        comparison_register,
    })
}

#[derive(Debug)]
struct Instruction {
    register: String,
    increment: i32,
    comparison: Comparison,
    comparison_register: String,
}

#[derive(Debug)]
struct Register {
    name: String,
    value: i32,
}

#[derive(Debug)]
enum Comparison {
    Equals(i32),
    NotEquals(i32),
    LessThan(i32),
    GreaterThan(i32),
    LessThanOrEquals(i32),
    GreaterThanOrEquals(i32),
}