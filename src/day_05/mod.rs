// Import project utility modules
use super::utils::fs;
use super::utils::intcode::IntcodeMachine;
use std::collections::VecDeque;

/// Solution for Day 05 Part 1.
pub fn solution_part_1(filename: String) -> i64 {
    let mut file = fs::open_file(filename);
    let int_args = IntcodeMachine::extract_intcode_memory_from_file(&mut file);
    let mut machine = IntcodeMachine::new(int_args, VecDeque::from(vec![1]));
    machine.execute_program();
    let output = machine.get_output_vec().pop_back().unwrap();
    return output;
}

/// Solution for Day 05 Part 2.
pub fn solution_part_2(filename: String) -> i64 {
    let mut file = fs::open_file(filename);
    let int_args = IntcodeMachine::extract_intcode_memory_from_file(&mut file);
    let mut machine = IntcodeMachine::new(int_args, VecDeque::from(vec![5]));
    machine.execute_program();
    let output = machine.get_output();
    return output;
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test actual solution for Part 1 - to check if this has been broken.
    #[test]
    fn test_p1_actual_solution() {
        let result = solution_part_1(String::from("./input/day_05/input.txt"));
        assert_eq!(12428642, result);
    }

    /// Test actual solution for Part 2 - to check if this has been broken.
    #[test]
    fn test_p2_actual_solution() {
        let result = solution_part_2(String::from("./input/day_05/input.txt"));
        assert_eq!(918655, result);
    }
}

