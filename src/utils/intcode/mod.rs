use std::error::Error;
use std::fs::File;
use std::io::Read;

// Intcode Opcode constants
const OPCODE_ADD: i32 = 1;
const OPCODE_MULT: i32 = 2;
const OPCODE_INPUT: i32 = 3;
const OPCODE_OUTPUT: i32 = 4;
const OPCODE_HALT: i32 = 99;
// Parameter modes
const PARAM_MODE_POSITION: i32 = 0;
const PARAM_MODE_IMMEDIATE: i32 = 1;

/// Represents the state of an Intcode Machine.
pub struct IntcodeMachine {
    program_counter: usize,
    memory: Vec<i32>,
    input: Vec<i32>,
    output: Vec<i32>,
}

impl IntcodeMachine {
    /// Creates a new instances of IntcodeMachine.
    pub fn new(initial_memory: Vec<i32>, input: Vec<i32>) -> Self {
        Self {
            program_counter: 0,
            memory: initial_memory,
            input: input,
            output: Vec::<i32>::new(),
        }
    }

    /// Executes the program contained within the machine.
    pub fn execute_program(&mut self) {
        loop { 
            // Extract program parameters for current run
            let arg = self.retrieve_from_memory(self.program_counter);
            let (opcode, mode_1, mode_2, _) = IntcodeMachine::extract_opcode_and_param_modes(arg);
            // Break here if HALT code is reached, just in case we are at end of program array
            if opcode == OPCODE_HALT {
                break;
            }
            // Check the current opcode and perform required operation
            if opcode == OPCODE_ADD {
                let arg_1 = self.retrieve_param_value(self.program_counter + 1, mode_1);
                let arg_2 = self.retrieve_param_value(self.program_counter + 2, mode_2);
                let output_index = self.retrieve_from_memory(self.program_counter + 3);
                let output = arg_1 + arg_2;
                self.store_in_memory(output, output_index as usize);
                self.program_counter += 4;
            } else if opcode == OPCODE_MULT {
                let arg_1 = self.retrieve_param_value(self.program_counter + 1, mode_1);
                let arg_2 = self.retrieve_param_value(self.program_counter + 2, mode_2);
                let output_index = self.retrieve_from_memory(self.program_counter + 3);
                let output = arg_1 * arg_2;
                self.store_in_memory(output, output_index as usize);
                self.program_counter += 4;
            } else if opcode == OPCODE_INPUT {
                if self.input.is_empty() {
                    panic!("Tried to get input from machine when empty.")
                }
                let input_value = self.input.pop().unwrap();
                let output_index = self.retrieve_from_memory(self.program_counter + 1);
                self.store_in_memory(input_value, output_index as usize);
                self.program_counter += 2;
            } else if opcode == OPCODE_OUTPUT {
                let arg_1 = self.retrieve_param_value(self.program_counter + 1, mode_1);
                self.output.push(arg_1);
                self.program_counter += 2;
            } else {
                // Shouldn't get here
                panic!("Opcode not recognised [pc: {}, opcode {}]", self.program_counter, opcode);
            }
        }
    }

    /// Returns the value held in location 0 of the machine memory.
    pub fn get_location_zero(&self) -> i32 {
        if self.memory.is_empty() {
            panic!("Machine memory is empty!");
        }
        return self.memory[0];
    }

    /// Returns a copy of the output stored in the machine.
    pub fn get_output(&self) -> Vec<i32> {
        return self.output.to_vec();
    }

    /// Using the given argument, this function extracts the opcode and parameter
    /// modes encoded in the value.
    /// 
    /// Output is in form: (opcode, mode_1, mode_2, mode_3)
    fn extract_opcode_and_param_modes(arg: i32) -> (i32, i32, i32, i32) {
        let opcode = arg % 100;
        let mode_1 = (arg / 100) % 10;
        let mode_2 = (arg / 1000) % 10;
        let mode_3 = (arg / 10000) % 10;
        return (opcode, mode_1, mode_2, mode_3);
    }

    /// Retrieves the value in the machine memory at the given index. Panics if
    /// an out-of-bounds access is attempted (bad index).
    fn retrieve_from_memory(&self, index: usize) -> i32 {
        if index >= self.memory.len() {
            panic!("Bad memory index.");
        }
        return self.memory[index];
    }

    /// Stores the given value at the specified index in the machine memory.
    /// Panics if an out-of-bounds access is attempted (bad index).
    fn store_in_memory(&mut self, value: i32, index: usize) {
        if index >= self.memory.len() {
            panic!("Bad memory index.");
        }
        self.memory[index] = value;
    }

    /// Retrieves a value from the machine memory, using the specified parameter mode.
    fn retrieve_param_value(&self, index: usize, param_mode: i32) -> i32 {
        if param_mode == PARAM_MODE_POSITION {
            let value_index =  self.retrieve_from_memory(index) as usize;
            let value = self.retrieve_from_memory(value_index);
            return value;
        } else if param_mode == PARAM_MODE_IMMEDIATE {
            let value = self.retrieve_from_memory(index);
            return value;
        } else {
            panic!("BAD PARAMETER MODE");
        }
    }

    /// Extracts the intcode arguments from the given file.
    ///
    /// File is read to string before arguments are split and converted to i32.
    pub fn extract_intcode_memory_from_file(file: &mut File) -> Vec<i32> {
        // Read line from file
        let mut read_buf = String::new();
        match file.read_to_string(&mut read_buf) {
            Err(e) => panic!("Error reading file. ({})", e.description()),
            Ok(_) => 0,
        };
        // Split string program into arguments
        read_buf = String::from(read_buf.trim());
        let str_args: Vec<&str> = read_buf.split(',').collect();
        // Convert string arguments into integers
        let mut int_args = Vec::<i32>::new();
        for str_arg in str_args {
            let value = str_arg.parse::<i32>().unwrap();
            int_args.push(value);
        }
        return int_args;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_halt() {
        let mut machine = IntcodeMachine::new(vec![99], vec![0]);
        machine.execute_program();
        assert_eq!(0, machine.program_counter);
    }

    #[test]
    fn test_write_to_memory() {
        let mut machine = IntcodeMachine::new(vec![3,3,99,0], vec![30]);
        machine.execute_program();
        assert_eq!(30, machine.memory[3]);
    }
    
    #[test]
    fn test_write_to_output() {
        let mut machine = IntcodeMachine::new(vec![4,2,99], vec![]);
        machine.execute_program();
        assert_eq!(99, machine.output[0]);
    }

    #[test]
    fn test_add() {
        let mut machine = IntcodeMachine::new(vec![1,2,2,0,99], vec![]);
        machine.execute_program();
        assert_eq!(4, machine.memory[0]);
    }

    #[test]
    fn test_mul() {
        let mut machine = IntcodeMachine::new(vec![2,2,4,0,99], vec![]);
        machine.execute_program();
        assert_eq!(396, machine.memory[0]);
    }

    #[test]
    fn test_immediate_mode() {
        let mut machine = IntcodeMachine::new(vec![1102,2,4,0,99], vec![]);
        machine.execute_program();
        assert_eq!(8, machine.memory[0]);
    }

    #[test]
    #[ignore]
    fn test_position_mode_equal() {
        let mut machine = IntcodeMachine::new(vec![3,9,8,9,10,9,4,9,99,-1,8], vec![8]);
        machine.execute_program();
        assert_eq!(1, machine.output[0]);
    }

    #[test]
    #[ignore]
    fn test_position_mode_not_equal() {
        let mut machine = IntcodeMachine::new(vec![3,9,8,9,10,9,4,9,99,-1,8], vec![10]);
        machine.execute_program();
        assert_eq!(0, machine.output[0]);
    }

    #[test]
    #[ignore]
    fn test_position_mode_less_than() {
        let mut machine = IntcodeMachine::new(vec![3,9,7,9,10,9,4,9,99,-1,8], vec![3]);
        machine.execute_program();
        assert_eq!(1, machine.output[0]);
    }

    #[test]
    #[ignore]
    fn test_position_mode_greater_than() {
        let mut machine = IntcodeMachine::new(vec![3,9,7,9,10,9,4,9,99,-1,8], vec![10]);
        machine.execute_program();
        assert_eq!(0, machine.output[0]);
    }

    #[test]
    #[ignore]
    fn test_immediate_mode_equal() {
        let mut machine = IntcodeMachine::new(vec![3,3,1108,-1,8,3,4,3,99], vec![8]);
        machine.execute_program();
        assert_eq!(1, machine.output[1]);
    }

    #[test]
    #[ignore]
    fn test_immediate_mode_not_equal() {
        let mut machine = IntcodeMachine::new(vec![3,3,1108,-1,8,3,4,3,99], vec![10]);
        machine.execute_program();
        assert_eq!(0, machine.output[0]);
    }

    #[test]
    #[ignore]
    fn test_immediate_mode_less_than() {
        let mut machine = IntcodeMachine::new(vec![3,3,1107,-1,8,3,4,3,99], vec![3]);
        machine.execute_program();
        assert_eq!(1, machine.output[0]);
    }

    #[test]
    #[ignore]
    fn test_immediate_mode_greater_than() {
        let mut machine = IntcodeMachine::new(vec![3,3,1107,-1,8,3,4,3,99], vec![10]);
        machine.execute_program();
        assert_eq!(0, machine.output[0]);
    }
}
