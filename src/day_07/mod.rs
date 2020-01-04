use super::utils::fs;
use super::utils::intcode::IntcodeMachine;
use itertools::Itertools;

/// Calculates the solution for Day 07 Part 1. Returned value is tuple containing
/// maximum output value (index 0) and associated phase combinations (5 values) for amplifiers
/// (index 1);
pub fn solution_part_1(filename: String) -> (i32, Vec<i32>) {
    // Load initial memory for amplifiers from input file
    let mut file = fs::open_file(filename);
    let initial_memory = IntcodeMachine::extract_intcode_memory_from_file(&mut file);
    // Generate all possible combinations of phase settings (0-4)
    let phase_combinations = (0..5).permutations(5);
    let mut max_output_value = -1;
    let mut max_outputs_phases = vec![-1, -1, -1, -1, -1];
    for combo in phase_combinations {
        // Initial configuration of amplifiers
        let amp_a_input = vec![combo[0], 0];
        let mut amp_b_input = vec![combo[1], -1];
        let mut amp_c_input = vec![combo[2], -1];
        let mut amp_d_input = vec![combo[3], -1];
        let mut amp_e_input = vec![combo[4], -1];
        // Run amplifier A
        let mut amp_a = IntcodeMachine::new(initial_memory.clone(), amp_a_input);
        amp_a.execute_program_halt_on_output(true);
        let amp_a_output = amp_a.get_output();
        if amp_a_output.len() != 1 {
            panic!("Bad Amp A output length! {}", amp_a_output.len());
        }
        amp_b_input[1] = amp_a_output[0];
        // Run amplifier B
        let mut amp_b = IntcodeMachine::new(initial_memory.clone(), amp_b_input);
        amp_b.execute_program_halt_on_output(true);
        let amp_b_output = amp_b.get_output();
        if amp_b_output.len() != 1 {
            panic!("Bad Amp B output length! {}", amp_b_output.len());
        }
        amp_c_input[1] = amp_b_output[0];
        // Run amplifier C
        let mut amp_c = IntcodeMachine::new(initial_memory.clone(), amp_c_input);
        amp_c.execute_program_halt_on_output(true);
        let amp_c_output = amp_c.get_output();
        if amp_c_output.len() != 1 {
            panic!("Bad Amp C output length! {}", amp_c_output.len());
        }
        amp_d_input[1] = amp_c_output[0];
        // Run amplifier D
        let mut amp_d = IntcodeMachine::new(initial_memory.clone(), amp_d_input);
        amp_d.execute_program_halt_on_output(true);
        let amp_d_output = amp_d.get_output();
        if amp_d_output.len() != 1 {
            panic!("Bad Amp D output length! {}", amp_d_output.len());
        }
        amp_e_input[1] = amp_d_output[0];
        // Run amplifier E
        let mut amp_e = IntcodeMachine::new(initial_memory.clone(), amp_e_input);
        amp_e.execute_program_halt_on_output(true);
        let amp_e_output = amp_e.get_output();
        if amp_e_output.len() != 1 {
            panic!("Bad Amp E output length! {}", amp_e_output.len());
        }
        let output_to_thrusters = amp_e_output[0];
        // Check if output is greatest seen so far
        if output_to_thrusters > max_output_value {
            max_output_value = output_to_thrusters;
            max_outputs_phases = combo;
        }
    }
    // Return output value and associated phase setting combination
    return (max_output_value, max_outputs_phases);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1_ex_input_01() {
        let result = solution_part_1(String::from("./input/day_07/test/test_01.txt"));
        assert_eq!(43210, result.0);
        assert_eq!(vec![4,3,2,1,0], result.1);
    }

    #[test]
    fn test_p1_ex_input_02() {
        let result = solution_part_1(String::from("./input/day_07/test/test_02.txt"));
        assert_eq!(54321, result.0);
        assert_eq!(vec![0,1,2,3,4], result.1);
    }

    #[test]
    fn test_p1_ex_input_03() {
        let result = solution_part_1(String::from("./input/day_07/test/test_03.txt"));
        assert_eq!(65210, result.0);
        assert_eq!(vec![1,0,4,3,2], result.1);
    }
}
