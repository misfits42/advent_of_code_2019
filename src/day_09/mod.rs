use std::collections::VecDeque;
use super::utils::fs;
use super::utils::intcode::IntcodeMachine;

pub fn solution_part_1(filename: String) -> i64 {
    let mut file = fs::open_file(filename);
    let int_args = IntcodeMachine::extract_intcode_memory_from_file(&mut file);
    let mut machine = IntcodeMachine::new(int_args, VecDeque::from(vec![1]));
    machine.execute_program();
    let output = machine.get_output();
    return output;
}
