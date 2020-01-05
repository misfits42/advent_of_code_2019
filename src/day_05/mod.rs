// Import project utility modules
use super::utils::fs;
use super::utils::intcode::IntcodeMachine;
use queues::*;

pub fn solution_part_1(filename: String) -> Vec<i32> {
    let mut file = fs::open_file(filename);
    let int_args = IntcodeMachine::extract_intcode_memory_from_file(&mut file);
    let mut machine = IntcodeMachine::new(int_args, queue![1]);
    machine.execute_program();
    let output = machine.get_output();
    return output;
}

pub fn solution_part_2(filename: String) -> Vec<i32> {
    let mut file = fs::open_file(filename);
    let int_args = IntcodeMachine::extract_intcode_memory_from_file(&mut file);
    let mut machine = IntcodeMachine::new(int_args, queue![5]);
    machine.execute_program();
    let output = machine.get_output();
    return output;
}

