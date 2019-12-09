//! Advent of Code - Day 2
//! Given an array/vector of numbers (num[]), analyze every 4 numbers
//! Every 4 numbers (n1, n2, n3, n4) represent a command
//! If n1 == 1, then num[n4] = num[n2] + num[n3]
//! If n1 == 2, then num[n4] = num[n2] * num[n3]
//! If n1 == 99, then stop

use std::fs::File;
use std::io::{BufReader, Read};

/// Find the highest value < desired_intcode by increasing just the noun value
/// Then increase the verb value until the value == desired_intcode
/// Return the noun and verb values
pub fn find_noun_verb(desired_intcode: i32) -> (i32, i32) {
    let (noun, _num): (i32, i32) = (1..100)
        .map(|n| (n, process_intcode(n, 0)))
        .filter(|&(s, y)| y <= desired_intcode)
        .last()
        .unwrap();

    let verb = (1..100)
        .filter(|&v| process_intcode(noun, v) == desired_intcode)
        .nth(0)
        .unwrap();

    (noun, verb)
}

pub fn process_intcode(noun: i32, verb: i32) -> i32 {
    let mut num_vec = get_lines("input/day2_modified_input_paul.txt");
    num_vec[1] = noun;
    num_vec[2] = verb;

    let mut new_num_vec = num_vec.to_vec();
    num_vec
        .chunks_exact(4)
        .for_each(|chunk| process_four_nums(chunk, &mut new_num_vec));

    new_num_vec[0]
}

fn process_four_nums(four_nums: &[i32], num_vec: &mut Vec<i32>) {
    let opcode = four_nums[0].clone();
    let num1 = four_nums[1].clone() as usize;
    let num2 = four_nums[2].clone() as usize;
    let dest = four_nums[3].clone() as usize;
    match opcode {
        1 => num_vec[dest] = num_vec[num1] + num_vec[num2],
        2 => num_vec[dest] = num_vec[num1] * num_vec[num2],
        99 => num_vec[dest] = num_vec[dest],
        _ => println!("ERROR processing numbers"),
    }
}

/// Get the string of numbers from the file and convert them into a Vector of f32
fn get_lines(file_path: &str) -> Vec<i32> {
    let file = File::open(file_path).expect("Could not open input for day1");
    let mut file_str = String::new();
    BufReader::new(file).read_to_string(&mut file_str).unwrap();
    let s: Vec<&str> = file_str.as_str().split(",").collect();
    s.iter().map(|x| x.parse::<i32>().unwrap()).collect()
}
