use std::collections::HashMap;

pub fn execute(input: &str) {
    let result_a = challenge_a(input);
    println!("Challenge 12a: {}", result_a);
    let result_b = challenge_b(input);
    println!("Challenge 12b: {}", result_b);
}

fn challenge_a(input: &str) -> i32 {
    let mut pipes = HashMap::new();
    for line in input.lines() {
        store_pipe(line, &mut pipes);
    }

    count_programs(&mut pipes, 0)
}

fn challenge_b(input: &str) -> i32 {
    let mut pipes = HashMap::new();
    for line in input.lines() {
        store_pipe(line, &mut pipes);
    }

    count_unique_pipes(&mut pipes)
}

fn count_programs(pipes: &mut HashMap<i32, ProgramNode>, from_program: i32) -> i32 {
    let mut program_queue = vec![from_program];
    let mut count = 0;

    while program_queue.len() > 0 {
        let program_pid = program_queue.pop().unwrap();
        let program = pipes.get_mut(&program_pid).unwrap();

        if program.visited {
            continue;
        }

        program.visited = true;
        count += 1;

        for &child in &program.children[..] {
            program_queue.push(child);
        }
    }
    count
}

fn count_unique_pipes(pipes: &mut HashMap<i32, ProgramNode>) -> i32 {
    let mut count = 0;
    let program_pids = pipes.keys().map(|&v| v).collect::<Vec<i32>>();
    for program_pid in program_pids {
        {
            let program = pipes.get(&program_pid).unwrap();

            if program.visited {
                continue;
            }
        }

        count_programs(pipes, program_pid);
        count += 1;
    }

    count
}

fn store_pipe(input: &str, pipes: &mut HashMap<i32, ProgramNode>) {
    let tokens: Vec<&str> = input.split(|c| c == ' ' || c == ',').collect();
    let current_pid: i32 = tokens[0].parse::<i32>().unwrap();

    if tokens[1] != "<->" {
        panic!("Unexpected tokens[1]");
    }

    for token in &tokens[2..] {
        let piped_pid = match token.parse::<i32>() {
            Ok(i) => i,
            _ => continue,
        };

        store_mapped_pid(pipes, current_pid, piped_pid);
        store_mapped_pid(pipes, piped_pid, current_pid);
    }
}

fn store_mapped_pid(pipes: &mut HashMap<i32, ProgramNode>, from_pid: i32, to_pid: i32) {
    let entry = pipes.entry(from_pid).or_insert(ProgramNode{
        pid: from_pid,
        visited: false,
        children: vec![],
    });

    if !entry.children.contains(&to_pid) {
        entry.children.push(to_pid);
    }
}

#[derive(Debug)]
struct ProgramNode {
    pid: i32,
    visited: bool,
    children: Vec<i32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(6, challenge_a("0 <-> 2\n1 <-> 1\n2 <-> 0, 3, 4\n3 <-> 2, 4\n4 <-> 2, 3, 6\n5 <-> 6\n6 <-> 4, 5"));
    }
}