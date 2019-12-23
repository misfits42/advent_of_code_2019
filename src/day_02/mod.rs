use std::io::Read;
use std::error::Error;
use std::fs::File;

// Intcode Opcode constants
const INTCODE_OPCODE_ADD: i32 = 1;
const INTCODE_OPCODE_MULT: i32 = 2;
const INTCODE_OPCODE_HALT: i32 = 99;

/// Calculates and displays the solution to Day 02 Part 1 challenge.
pub fn solution_part_1(filename: String) {
    // Open file
    let mut file = super::utils::fs::open_file(filename);
    // Extract intcode program arguments
    let int_args = extract_intcode_arguments_from_file(&mut file);
    // Process the intcode program
    let result = process_intcode_program(&int_args, None);
    println!("Day 02 Part 1 solution is: {}", result);
}

/// Calculates and displays the solution to Day 02 Part 2 challenge.
pub fn solution_part_2(filename: String) {
    // This is the value we are looking for in position zero across the runs
    const TARGET_LOC_ZERO: i32 = 19690720;
    // Open file
    let mut file = super::utils::fs::open_file(filename);
    // Extract intcode program arguments
    let int_args = extract_intcode_arguments_from_file(&mut file);
    // Let's process the intcode program with each possible value pair
    for (p0, p1) in iproduct!(0..100, 0..100) {
        let pair = vec![p0, p1];
        let pos_zero = process_intcode_program(&int_args, Some(&pair));
        if pos_zero == TARGET_LOC_ZERO {
            let output = 100 * pair[0] + pair[1];
            println!("Day 02 Part 2 solution is: {}", output);
            return;
        }
    }
    // Shouldn't get here!
    panic!("Day 02 Part 2: HERE BE DRAGONS!");
}

/// Extracts the intcode arguments from the given file. 
/// 
/// File is read to string before arguments are split and converted to i32.
fn extract_intcode_arguments_from_file(file: &mut File) -> Vec<i32> {
    // Read line from file
    let mut read_buf = String::new();
    match file.read_to_string(&mut read_buf) {
        Err(e) => panic!("Error reading file. ({})", e.description()),
        Ok(_) => 0,
    };
    // Split string program into arguments
    let str_args: Vec<&str> = read_buf.split(',').collect();
    // Convert string arguments into integers
    let mut int_args = Vec::<i32>::new();
    for str_arg in str_args {
        let value = str_arg.parse::<i32>().unwrap();
        int_args.push(value);
    }
    return int_args;
}

/// Processes the given IntCode program. Assumes that program will run within
/// bounds of array (assumed good input program).
/// 
/// Returns the value at position 0 after IntCode program processing halts.
fn process_intcode_program(program: &Vec<i32>, maybe_pair: Option<&Vec<i32>>) -> i32 {
    let mut stack_pointer = 0;
    let mut program = program.to_vec();
    // Check if a pair has been given
    if let Some(pair) = maybe_pair {
        program[1] = pair[0];
        program[2] = pair[1];
    }
    loop {
        // Extract program parameters for current run
        let opcode: i32 = program[stack_pointer];
        let arg1_ind: usize = program[stack_pointer + 1] as usize;
        let arg2_ind: usize = program[stack_pointer + 2] as usize;
        let arg1: i32 = program[arg1_ind];
        let arg2: i32 = program[arg2_ind];
        let out_ind: usize = program[stack_pointer + 3] as usize;
        // Check the current opcode and perform required operation
        if opcode == INTCODE_OPCODE_ADD {
            let output = arg1 + arg2;
            program[out_ind] = output;
        } else if opcode == INTCODE_OPCODE_MULT {
            let output = arg1 * arg2;
            program[out_ind] = output;
        } else if opcode == INTCODE_OPCODE_HALT {
            break;
        } else { // Shouldn't get here
            panic!("Day 02 Part 1: HERE BE DRAGONS!")
        }
        // Advance stack pointer to next opcode
        stack_pointer += 4;
    }
    // Return the resulting element at location 0
    return program[0];
}
