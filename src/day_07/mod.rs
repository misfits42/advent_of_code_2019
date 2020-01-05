use super::utils::fs;
use super::utils::intcode::IntcodeMachine;
use itertools::Itertools;
use std::collections::VecDeque;

/// Calculates the solution for Day 07 Part 1. Returned value is tuple containing maximum output
/// value (index 0) and associated phase combinations (5 values) for amplifiers (index 1).
pub fn solution_part_1(filename: String) -> (i32, Vec<i32>) {
    // Load initial memory for amplifiers from input file
    let mut file = fs::open_file(filename);
    let initial_memory = IntcodeMachine::extract_intcode_memory_from_file(&mut file);
    // Generate all possible permutations of phase settings (0-4)
    let phase_permutations = (0..5).permutations(5);
    let mut max_output_value = 0;
    let mut max_outputs_phases = vec![-1, -1, -1, -1, -1];
    for permu in phase_permutations {
        // Run amplifier A
        let amp_a_output = run_intcode_machine_as_amp(initial_memory.clone(), permu[0], 0);
        // Run amplifier B
        let amp_b_output =
            run_intcode_machine_as_amp(initial_memory.clone(), permu[1], amp_a_output);
        // Run amplifier C
        let amp_c_output =
            run_intcode_machine_as_amp(initial_memory.clone(), permu[2], amp_b_output);
        // Run amplifier D
        let amp_d_output =
            run_intcode_machine_as_amp(initial_memory.clone(), permu[3], amp_c_output);
        // Run amplifier E
        let amp_e_output_to_thruster =
            run_intcode_machine_as_amp(initial_memory.clone(), permu[4], amp_d_output);
        // Check if output is greatest seen so far
        if amp_e_output_to_thruster > max_output_value {
            println!(
                "Found new max: {} > {} ({:?})",
                amp_e_output_to_thruster, max_output_value, permu
            );
            max_output_value = amp_e_output_to_thruster;
            max_outputs_phases = permu;
        }
    }
    // Return output value and associated phase setting combination
    return (max_output_value, max_outputs_phases);
}

/// Calculates the solution for Day 07 Part 2. Returned value is tuple containing maximum output
/// value (index 0) and associated phase combinations (5 values) for amplifiers (index 1).
pub fn solution_part_2(filename: String) -> (i32, Vec<i32>) {
    // Load initial memory for amplifiers from input file
    let mut file = fs::open_file(filename);
    let initial_memory = IntcodeMachine::extract_intcode_memory_from_file(&mut file);
    // Generate all possible permutations of phase settings (5-9)
    let phase_permutations = (5..10).permutations(5);
    let mut max_output_value = 0;
    let mut max_output_phases = vec![-1, -1, -1, -1, -1];
    for permu in phase_permutations {
        // Initial configuration for machines
        let mut amp_a = IntcodeMachine::new(initial_memory.clone(), VecDeque::from(vec![permu[0], 0]));
        let mut amp_b = IntcodeMachine::new(initial_memory.clone(), VecDeque::from(vec![permu[1]]));
        let mut amp_c = IntcodeMachine::new(initial_memory.clone(), VecDeque::from(vec![permu[2]]));
        let mut amp_d = IntcodeMachine::new(initial_memory.clone(), VecDeque::from(vec![permu[3]]));
        let mut amp_e = IntcodeMachine::new(initial_memory.clone(), VecDeque::from(vec![permu[4]]));
        // Maintain the output value of Amp E in case it ends up being the output to thrusters
        let mut amp_e_output = -1;
        // Continue executing until amplifier E has halted
        loop {
            // Amp A
            amp_a.execute_program_break_on_output(true);
            if !amp_a.has_halted() {
                let amp_a_output = amp_a.get_output(); // Assume output len 1 is program works correctly
                amp_a.clear_output();
                amp_b.add_input(amp_a_output);
            }
            // Amp B
            amp_b.execute_program_break_on_output(true);
            if !amp_b.has_halted() {
                let amp_b_output = amp_b.get_output();
                amp_b.clear_output();
                amp_c.add_input(amp_b_output);
            }
            // Amp C
            amp_c.execute_program_break_on_output(true);
            if !amp_c.has_halted() {
                let amp_c_output = amp_c.get_output();
                amp_c.clear_output();
                amp_d.add_input(amp_c_output);
            }
            // Amp D
            amp_d.execute_program_break_on_output(true);
            if !amp_d.has_halted() {
                let amp_d_output = amp_d.get_output();
                amp_d.clear_output();
                amp_e.add_input(amp_d_output);
            }
            // Amp E
            amp_e.execute_program_break_on_output(true);
            if !amp_e.has_halted() {
                amp_e_output = amp_e.get_output();
                amp_e.clear_output();
                amp_a.add_input(amp_e_output);
            } else { // Amp E halted - take its output as the output to thrusters
                let output_to_thrusters = amp_e_output;
                if output_to_thrusters > max_output_value {
                    println!(
                        "Found new max: {} > {} ({:?})",
                        output_to_thrusters, max_output_value, permu
                    );
                    max_output_value = output_to_thrusters;
                    max_output_phases = permu.to_vec();
                }
                break;
            }
        }
    }

    return (max_output_value, max_output_phases);
}

/// Runs the given program in an IntcodeMachine instance, using the given phase and input as the two
/// input values to the machine (prior to program execution).
fn run_intcode_machine_as_amp(initial_memory: Vec<i32>, phase: i32, input_value: i32) -> i32 {
    let amp_input = VecDeque::from(vec![phase, input_value]);
    let mut amp_machine = IntcodeMachine::new(initial_memory, amp_input);
    amp_machine.execute_program();
    let amp_output = amp_machine.get_output();
    return amp_output;
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test actual solution for Part 1 - to check if this has been broken.
    #[test]
    fn test_p1_actual_solution() {
        let result = solution_part_1(String::from("./input/day_07/input.txt"));
        assert_eq!(880726, result.0);
        assert_eq!(vec![2,0,1,4,3], result.1);
    }

    /// Test actual solution for Part 2 - to check if this has been broken.
    #[test]
    fn test_p2_actual_solution() {
        let result = solution_part_2(String::from("./input/day_07/input.txt"));
        assert_eq!(4931744, result.0);
        assert_eq!(vec![7,8,5,6,9], result.1);
    }

    #[test]
    fn test_p1_ex_input_01() {
        let result = solution_part_1(String::from("./input/day_07/test/test_01.txt"));
        assert_eq!(43210, result.0);
        assert_eq!(vec![4, 3, 2, 1, 0], result.1);
    }

    #[test]
    fn test_p1_ex_input_02() {
        let result = solution_part_1(String::from("./input/day_07/test/test_02.txt"));
        assert_eq!(54321, result.0);
        assert_eq!(vec![0, 1, 2, 3, 4], result.1);
    }

    #[test]
    fn test_p1_ex_input_03() {
        let result = solution_part_1(String::from("./input/day_07/test/test_03.txt"));
        assert_eq!(65210, result.0);
        assert_eq!(vec![1, 0, 4, 3, 2], result.1);
    }

    #[test]
    fn test_p2_ex_input_04() {
        let result = solution_part_2(String::from("./input/day_07/test/test_04.txt"));
        assert_eq!(139629729, result.0);
        assert_eq!(vec![9,8,7,6,5], result.1);
    }

    #[test]
    fn test_p2_ex_input_05() {
        let result = solution_part_2(String::from("./input/day_07/test/test_05.txt"));
        assert_eq!(18216, result.0);
        assert_eq!(vec![9,7,8,5,6], result.1);
    }
}
