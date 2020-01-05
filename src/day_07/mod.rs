use super::utils::fs;
use super::utils::intcode::IntcodeMachine;
use itertools::Itertools;
use queues::*;

/// Calculates the solution for Day 07 Part 1. Returned value is tuple containing
/// maximum output value (index 0) and associated phase combinations (5 values) for amplifiers
/// (index 1);
pub fn solution_part_1(filename: String) -> (i32, Vec<i32>) {
    // Load initial memory for amplifiers from input file
    let mut file = fs::open_file(filename);
    let initial_memory = IntcodeMachine::extract_intcode_memory_from_file(&mut file);
    // Generate all possible combinations of phase settings (0-4)
    let phase_combinations = (0..5).permutations(5);
    let mut max_output_value = 0;
    let mut max_outputs_phases = vec![-1, -1, -1, -1, -1];
    for combo in phase_combinations {
        // Run amplifier A
        let amp_a_output = run_intcode_machine_as_amp(initial_memory.clone(), combo[0], 0);
        // Run amplifier B
        let amp_b_output =
            run_intcode_machine_as_amp(initial_memory.clone(), combo[1], amp_a_output);
        // Run amplifier C
        let amp_c_output =
            run_intcode_machine_as_amp(initial_memory.clone(), combo[2], amp_b_output);
        // Run amplifier D
        let amp_d_output =
            run_intcode_machine_as_amp(initial_memory.clone(), combo[3], amp_c_output);
        // Run amplifier E
        let amp_e_output_to_thruster =
            run_intcode_machine_as_amp(initial_memory.clone(), combo[4], amp_d_output);
        // Check if output is greatest seen so far
        if amp_e_output_to_thruster > max_output_value {
            println!(
                "Found new max: {} > {} ({:?})",
                amp_e_output_to_thruster, max_output_value, combo
            );
            max_output_value = amp_e_output_to_thruster;
            max_outputs_phases = combo;
        }
    }
    // Return output value and associated phase setting combination
    return (max_output_value, max_outputs_phases);
}

/// Runs the given program in an IntcodeMachine instance, using the given phase and input as the two
/// input values to the machine (prior to program execution).
fn run_intcode_machine_as_amp(initial_memory: Vec<i32>, phase: i32, input_value: i32) -> i32 {
    let amp_input = queue![phase, input_value];
    let mut amp_machine = IntcodeMachine::new(initial_memory, amp_input);
    amp_machine.execute_program();
    let amp_output = amp_machine.get_output();
    if amp_output.len() != 1 {
        panic!("Bad amplifier output length!");
    }
    return amp_output[0];
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
