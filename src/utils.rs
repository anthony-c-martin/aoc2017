use std::collections::HashMap;
use std::hash::Hash;

fn to_digit(c: char) -> u32 {
    c.to_digit(10).unwrap()
}

fn to_integer(input: &str) -> i32 {
    input.parse::<i32>().unwrap()
}

pub fn parse_int(input: &str) -> Vec<u32> {
    input.chars().map(to_digit).collect()
}

pub fn parse_tabbed_data(input: &str) -> Vec<i32> {
    input.split_whitespace().map(to_integer).collect()
}

pub fn parse_array_input(input: &str) -> Vec<Vec<i32>> {
    input.lines().map(parse_tabbed_data).collect()
}

pub struct TrieNode<T: Hash> {
    is_end: bool,
    children: HashMap<T, TrieNode<T>>
}

impl<T: Eq + Hash + Clone> TrieNode<T> {
    pub fn new() -> TrieNode<T> {
        TrieNode::<T> {
            is_end: false,
            children: HashMap::new()
        }
    }

    pub fn insert_check_dupe(&mut self, input: &[T]) -> bool {
        if input.is_empty() {
            if !self.is_end {
                self.is_end = true;
                return false;
            }

            return true;
        }

        self.children.entry(input[0].clone())
            .or_insert(TrieNode::new())
            .insert_check_dupe(&input[1..])
    }
}