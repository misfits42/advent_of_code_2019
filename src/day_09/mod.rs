use std::collections::VecDeque;
use super::utils::fs;
use super::utils::intcode::IntcodeMachine;

/// Calculates solution for Day 09 Part 1 challenge.
pub fn solution_part_1(filename: String) -> i64 {
    let mut file = fs::open_file(filename);
    let int_args = IntcodeMachine::extract_intcode_memory_from_file(&mut file);
    let mut machine = IntcodeMachine::new(int_args, VecDeque::from(vec![1]));
    machine.execute_program();
    let output = machine.get_output();
    return output;
}

/// Calculates solution for Day 09 Part 2 challenge.
pub fn solution_part_2(filename: String) -> i64 {
    let mut file = fs::open_file(filename);
    let int_args = IntcodeMachine::extract_intcode_memory_from_file(&mut file);
    let mut machine = IntcodeMachine::new(int_args, VecDeque::from(vec![2]));
    machine.execute_program();
    let output = machine.get_output();
    return output;
}

#[cfg(test)]
mod tests {
    use super::*;

    ///  Tests actual solution for Day 09 Part 1 challenge.
    #[test]
    fn test_p1_actual_solution() {
        let result = solution_part_1(String::from("./input/day_09/input.txt"));
        assert_eq!(result, 2745604242);
    }

    ///  Tests actual solution for Day 09 Part 2 challenge.
    #[test]
    fn test_p2_actual_solution() {
        let result = solution_part_2(String::from("./input/day_09/input.txt"));
        assert_eq!(result, 51135);
    }
}
