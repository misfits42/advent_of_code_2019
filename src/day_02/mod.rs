use std::io::Read;
use std::error::Error;

// Intcode Opcode contants
const INTCODE_OPCODE_ADD: i32 = 1;
const INTCODE_OPCODE_MULT: i32 = 2;
const INTCODE_OPCODE_HALT: i32 = 99;

pub fn day_02_solution_p1(filename: String) {
    // Open file
    let mut file = super::utils::fs::open_file(filename);
    // Read line from file
    let mut intcode_program = String::new();
    match file.read_to_string(&mut intcode_program) {
        Err(e) => panic!("Error reading file. ({})", e.description()),
        Ok(_) => 0,
    };
    // Split string program into arguments
    let str_args: Vec<&str> = intcode_program.split(',').collect();
    // Convert string arguments into integers
    let mut int_args = Vec::<i32>::new();
    for str_arg in str_args {
        let value = str_arg.parse::<i32>().unwrap();
        int_args.push(value);
    }
    // Process the intcode program
    let result = process_intcode_program(int_args);
    println!("Day 02 Part 1 solution is: {}", result);
}

/// Processes the given IntCode program. Assumes that program will run within
/// bounds of array (assumed good input program).
/// 
/// Returns the value at position 0 after IntCode program processing halts.
fn process_intcode_program(program: Vec<i32>) -> i32 {
    let mut stack_pointer = 0;
    let mut program = program.to_vec();
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
