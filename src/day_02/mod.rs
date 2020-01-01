// Import project utility modules
use super::utils::fs;
use super::utils::intcode::IntcodeMachine;

/// Calculates and displays the solution to Day 02 Part 1 challenge.
pub fn solution_part_1(filename: String) -> i32 {
    // Open file
    let mut file = fs::open_file(filename);
    // Extract intcode program arguments
    let int_args = IntcodeMachine::extract_intcode_memory_from_file(&mut file);
    let mut machine = IntcodeMachine::new(int_args, vec![]);
    machine.execute_program();
    // Process the intcode program
    let result = machine.get_location_zero();
    return result;
}

/// Calculates and displays the solution to Day 02 Part 2 challenge.
pub fn solution_part_2(filename: String) -> i32 {
    // This is the value we are looking for in position zero across the runs
    const TARGET_LOC_ZERO: i32 = 19690720;
    // Open file
    let mut file = fs::open_file(filename);
    // Extract intcode program arguments
    let int_args = IntcodeMachine::extract_intcode_memory_from_file(&mut file);
    // Let's process the intcode program with each possible value pair
    for (p0, p1) in iproduct!(0..100, 0..100) {
        let mut updated_int_args = int_args.to_vec();
        updated_int_args[0] = p0;
        updated_int_args[1] = p1;
        let mut machine = IntcodeMachine::new(updated_int_args, vec![]);
        machine.execute_program();
        if machine.get_location_zero() == TARGET_LOC_ZERO {
            let output = 100 * p0 + p1;
            return output;
        }
    }
    // Shouldn't get here!
    panic!("Day 02 Part 2: HERE BE DRAGONS!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_p1_example_input_1() {
        let result = super::solution_part_1(String::from("./input/day_02/test/test_01.txt"));
        assert_eq!(2, result);
    }

    #[test]
    fn test_p1_example_input_2() {
        let result = super::solution_part_1(String::from("./input/day_02/test/test_02.txt"));
        assert_eq!(2, result);
    }

    #[test]
    fn test_p1_example_input_3() {
        let result = super::solution_part_1(String::from("./input/day_02/test/test_03.txt"));
        assert_eq!(2, result);
    }

    #[test]
    fn test_p1_example_input_4() {
        let result = super::solution_part_1(String::from("./input/day_02/test/test_04.txt"));
        assert_eq!(30, result);
    }
}
