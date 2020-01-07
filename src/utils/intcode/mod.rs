use std::collections::VecDeque;
use std::error::Error;
use std::fs::File;
use std::io::Read;

// Intcode Opcode constants
const OPCODE_ADD: i64 = 1;
const OPCODE_MULT: i64 = 2;
const OPCODE_INPUT: i64 = 3;
const OPCODE_OUTPUT: i64 = 4;
const OPCODE_JUMP_IF_TRUE: i64 = 5;
const OPCODE_JUMP_IF_FALSE: i64 = 6;
const OPCODE_LESS_THAN: i64 = 7;
const OPCODE_EQUALS: i64 = 8;
const OPCODE_ADJUST_REL_BASE: i64 = 9;
const OPCODE_HALT: i64 = 99;
// Parameter modes
const PARAM_MODE_POSITION: i64 = 0;
const PARAM_MODE_IMMEDIATE: i64 = 1;
const PARAM_MODE_RELATIVE: i64 = 2;
// Memory config
const MEMORY_SIZE: usize = 10000;

/// Represents the state of an Intcode Machine.
pub struct IntcodeMachine {
    prog_c: usize,
    memory: Vec<i64>,
    input: VecDeque<i64>,
    output: VecDeque<i64>,
    halted: bool,
    relative_base: i64,
}

impl IntcodeMachine {
    /// Creates a new instances of IntcodeMachine.
    pub fn new(initial_memory: Vec<i64>, input: VecDeque<i64>) -> Self {
        Self {
            prog_c: 0,
            memory: IntcodeMachine::create_initial_memory_array(initial_memory),
            input: input,
            output: VecDeque::new(),
            halted: false,
            relative_base: 0,
        }
    }

    /// Copies the given initial memory into a vector with maximum memory size of machine, and
    /// returns the result.
    fn create_initial_memory_array(initial_memory: Vec<i64>) -> Vec<i64> {
        let mut memory = Vec::<i64>::with_capacity(MEMORY_SIZE);
        for i in 0..MEMORY_SIZE {
            if i < initial_memory.len() {
                memory.push(initial_memory[i]);
            } else {
                memory.push(0);
            }
        }
        return memory;
    }

    /// Adds the given input value to the input queue of the machine.
    pub fn add_input(&mut self, input_value: i64) {
        self.input.push_back(input_value);
    }

    /// Returns the current halt state of the machine.
    pub fn has_halted(&self) -> bool {
        return self.halted;
    }

    /// Clears the output queue of the machine.
    pub fn clear_output(&mut self) {
        self.output = VecDeque::new();
    }

    /// Executes the program contained in machine memory. Has the option of breaking program
    /// execution after the machine executes the first output instruction.
    ///
    /// If the machine has already halted (i.e. encountered a HALT opcode), this function will
    /// immediately return.
    pub fn execute_program_break_on_output(&mut self, break_on_output: bool) {
        // If machine has already halted, don't try to execute any instructions.
        if self.has_halted() {
            return;
        }
        loop {
            // Extract program parameters for current run
            let arg = self.retrieve_from_memory(self.prog_c);
            let (opcode, mode_1, mode_2, mode_3) =
                IntcodeMachine::extract_opcode_and_param_modes(arg);
            // Break here if HALT code is reached, just in case we are at end of program array
            if opcode == OPCODE_HALT {
                self.halted = true;
                break;
            }
            // Check the current opcode and perform required operation
            if opcode == OPCODE_ADD {
                let param_1 = self.retrieve_param_value(self.prog_c + 1, mode_1, true);
                let param_2 = self.retrieve_param_value(self.prog_c + 2, mode_2, true);
                let output_addr = self.retrieve_param_value(self.prog_c + 3, mode_3, false);
                let output = param_1 + param_2;
                self.store_in_memory(output, output_addr as usize);
                self.prog_c += 4;
            } else if opcode == OPCODE_MULT {
                let param_1 = self.retrieve_param_value(self.prog_c + 1, mode_1, true);
                let param_2 = self.retrieve_param_value(self.prog_c + 2, mode_2, true);
                let output_addr = self.retrieve_param_value(self.prog_c + 3, mode_3, false);
                let output = param_1 * param_2;
                self.store_in_memory(output, output_addr as usize);
                self.prog_c += 4;
            } else if opcode == OPCODE_INPUT {
                if self.input.len() == 0 {
                    panic!("Tried to get input from machine when empty.");
                }
                let input_value = self.input.pop_front().unwrap();
                let output_addr = self.retrieve_param_value(self.prog_c + 1, mode_1, false);
                self.store_in_memory(input_value, output_addr as usize);
                self.prog_c += 2;
            } else if opcode == OPCODE_OUTPUT {
                let param_1 = self.retrieve_param_value(self.prog_c + 1, mode_1, true);
                self.output.push_back(param_1);
                self.prog_c += 2;
                // Check if the machine should break after executing an output instruction
                if break_on_output {
                    break;
                }
            } else if opcode == OPCODE_JUMP_IF_TRUE {
                let param_1 = self.retrieve_param_value(self.prog_c + 1, mode_1, true);
                let param_2 = self.retrieve_param_value(self.prog_c + 2, mode_2, true);
                if param_1 != 0 {
                    self.prog_c = param_2 as usize;
                } else {
                    self.prog_c += 3;
                }
            } else if opcode == OPCODE_JUMP_IF_FALSE {
                let param_1 = self.retrieve_param_value(self.prog_c + 1, mode_1, true);
                let param_2 = self.retrieve_param_value(self.prog_c + 2, mode_2, true);
                if param_1 == 0 {
                    self.prog_c = param_2 as usize;
                } else {
                    self.prog_c += 3;
                }
            } else if opcode == OPCODE_LESS_THAN {
                let param_1 = self.retrieve_param_value(self.prog_c + 1, mode_1, true);
                let param_2 = self.retrieve_param_value(self.prog_c + 2, mode_2, true);
                let output_addr = self.retrieve_param_value(self.prog_c + 3, mode_3, false);
                if param_1 < param_2 {
                    self.store_in_memory(1, output_addr as usize);
                } else {
                    self.store_in_memory(0, output_addr as usize);
                }
                self.prog_c += 4;
            } else if opcode == OPCODE_EQUALS {
                let param_1 = self.retrieve_param_value(self.prog_c + 1, mode_1, true);
                let param_2 = self.retrieve_param_value(self.prog_c + 2, mode_2, true);
                let output_addr = self.retrieve_param_value(self.prog_c + 3, mode_3, false);
                if param_1 == param_2 {
                    self.store_in_memory(1, output_addr as usize);
                } else {
                    self.store_in_memory(0, output_addr as usize);
                }
                self.prog_c += 4;
            } else if opcode == OPCODE_ADJUST_REL_BASE {
                let delta = self.retrieve_param_value(self.prog_c + 1, mode_1, true);
                self.relative_base += delta;
                self.prog_c += 2;
            } else {
                // Shouldn't get here
                panic!(
                    "Opcode not recognised [pc: {}, opcode {}]",
                    self.prog_c, opcode
                );
            }
        }
    }

    /// Executes the program contained within the machine.
    pub fn execute_program(&mut self) {
        self.execute_program_break_on_output(false);
    }

    /// Returns the value held in location 0 of the machine memory.
    pub fn get_location_zero(&self) -> i64 {
        if self.memory.is_empty() {
            panic!("Machine memory is empty!");
        }
        return self.memory[0];
    }

    /// Returns the first value in the output queue of the machine.
    pub fn get_output(&self) -> i64 {
        if self.output.len() == 0 {
            panic!("Tried to get output from IntcodeMachine with an empty output.");
        }
        return self.output[0];
    }

    /// Returns a copy of the output queue.
    pub fn get_output_vec(&self) -> VecDeque<i64> {
        return self.output.clone();
    }

    /// Using the given argument, this function extracts the opcode and parameter
    /// modes encoded in the value.
    ///
    /// Output is in form: (opcode, mode_1, mode_2, mode_3)
    fn extract_opcode_and_param_modes(arg: i64) -> (i64, i64, i64, i64) {
        let opcode = arg % 100;
        let mode_1 = (arg / 100) % 10;
        let mode_2 = (arg / 1000) % 10;
        let mode_3 = (arg / 10000) % 10;
        return (opcode, mode_1, mode_2, mode_3);
    }

    /// Retrieves the value in the machine memory at the given index. Panics if
    /// an out-of-bounds access is attempted (bad index).
    fn retrieve_from_memory(&self, index: usize) -> i64 {
        if index >= self.memory.len() {
            panic!("Bad memory index.");
        }
        return self.memory[index];
    }

    /// Stores the given value at the specified index in the machine memory.
    /// Panics if an out-of-bounds access is attempted (bad index).
    fn store_in_memory(&mut self, value: i64, index: usize) {
        if index >= self.memory.len() {
            panic!("Bad memory index.");
        }
        self.memory[index] = value;
    }

    /// Looks up values from the memory of the machine using the provided index and parameter mode.
    /// The third argument indicates whether or not the calculated address value should be used to
    /// lookup the return value or if the address should itself be returned as the result.
    fn retrieve_param_value(&self, index: usize, param_mode: i64, do_memory_lookup: bool) -> i64 {
        if param_mode == PARAM_MODE_POSITION {
            let address = self.retrieve_from_memory(index);
            if !do_memory_lookup {
                return address;
            } else {
                return self.retrieve_from_memory(address as usize);
            }
        } else if param_mode == PARAM_MODE_IMMEDIATE {
            let value = self.retrieve_from_memory(index);
            return value;
        } else if param_mode == PARAM_MODE_RELATIVE {
            let address = self.relative_base + self.retrieve_from_memory(index);
            if !do_memory_lookup {
                return address;
            } else {
                return self.retrieve_from_memory(address as usize);
            }
        } else {
            panic!("BAD PARAMETER MODE");
        }
    }

    /// Extracts the intcode arguments from the given file.
    ///
    /// File is read to string before arguments are split and converted to i64.
    pub fn extract_intcode_memory_from_file(file: &mut File) -> Vec<i64> {
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
        let mut int_args = Vec::<i64>::new();
        for str_arg in str_args {
            let value = str_arg.parse::<i64>().unwrap();
            int_args.push(value);
        }
        return int_args;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_09_p1_copy_output() {
        let mut machine = IntcodeMachine::new(
            vec![
                109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
            ],
            VecDeque::from(vec![]),
        );
        machine.execute_program();
        assert_eq!(
            VecDeque::from(vec![
                109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99
            ]),
            machine.output
        );
    }

    #[test]
    fn test_day_09_p1_16digit_output() {
        let mut machine = IntcodeMachine::new(
            vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0],
            VecDeque::from(vec![]),
        );
        machine.execute_program();
        assert_eq!(1219070632396864, machine.output[0]);
    }

    #[test]
    fn test_day_09_p1_bit_output() {
        let mut machine =
            IntcodeMachine::new(vec![104, 1125899906842624, 99], VecDeque::from(vec![]));
        machine.execute_program();
        assert_eq!(1125899906842624, machine.output[0]);
    }

    #[test]
    fn test_halt() {
        let mut machine = IntcodeMachine::new(vec![99], VecDeque::from(vec![0]));
        machine.execute_program();
        assert_eq!(0, machine.prog_c);
    }

    #[test]
    fn test_write_to_memory() {
        let mut machine = IntcodeMachine::new(vec![3, 3, 99, 0], VecDeque::from(vec![30]));
        machine.execute_program();
        assert_eq!(30, machine.memory[3]);
    }
    #[test]
    fn test_write_to_output() {
        let mut machine = IntcodeMachine::new(vec![4, 2, 99], VecDeque::from(vec![]));
        machine.execute_program();
        assert_eq!(99, machine.get_output());
    }

    #[test]
    fn test_add() {
        let mut machine = IntcodeMachine::new(vec![1, 2, 2, 0, 99], VecDeque::from(vec![]));
        machine.execute_program();
        assert_eq!(4, machine.memory[0]);
    }

    #[test]
    fn test_mul() {
        let mut machine = IntcodeMachine::new(vec![2, 2, 4, 0, 99], VecDeque::from(vec![]));
        machine.execute_program();
        assert_eq!(396, machine.memory[0]);
    }

    #[test]
    fn test_immediate_mode() {
        let mut machine = IntcodeMachine::new(vec![1102, 2, 4, 0, 99], VecDeque::from(vec![]));
        machine.execute_program();
        assert_eq!(8, machine.memory[0]);
    }

    #[test]
    fn test_position_mode_equal() {
        let mut machine = IntcodeMachine::new(
            vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8],
            VecDeque::from(vec![8]),
        );
        machine.execute_program();
        assert_eq!(1, machine.get_output());
    }

    #[test]
    fn test_position_mode_not_equal() {
        let mut machine = IntcodeMachine::new(
            vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8],
            VecDeque::from(vec![10]),
        );
        machine.execute_program();
        assert_eq!(0, machine.get_output());
    }

    #[test]
    fn test_position_mode_less_than() {
        let mut machine = IntcodeMachine::new(
            vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8],
            VecDeque::from(vec![3]),
        );
        machine.execute_program();
        assert_eq!(1, machine.get_output());
    }

    #[test]
    fn test_position_mode_greater_than() {
        let mut machine = IntcodeMachine::new(
            vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8],
            VecDeque::from(vec![10]),
        );
        machine.execute_program();
        assert_eq!(0, machine.get_output());
    }

    #[test]
    fn test_immediate_mode_equal() {
        let mut machine = IntcodeMachine::new(
            vec![3, 3, 1108, -1, 8, 3, 4, 3, 99],
            VecDeque::from(vec![8]),
        );
        machine.execute_program();
        assert_eq!(1, machine.get_output());
    }

    #[test]
    fn test_immediate_mode_not_equal() {
        let mut machine = IntcodeMachine::new(
            vec![3, 3, 1108, -1, 8, 3, 4, 3, 99],
            VecDeque::from(vec![10]),
        );
        machine.execute_program();
        assert_eq!(0, machine.get_output());
    }

    #[test]
    fn test_immediate_mode_less_than() {
        let mut machine = IntcodeMachine::new(
            vec![3, 3, 1107, -1, 8, 3, 4, 3, 99],
            VecDeque::from(vec![3]),
        );
        machine.execute_program();
        assert_eq!(1, machine.get_output());
    }

    #[test]
    fn test_immediate_mode_greater_than() {
        let mut machine = IntcodeMachine::new(
            vec![3, 3, 1107, -1, 8, 3, 4, 3, 99],
            VecDeque::from(vec![10]),
        );
        machine.execute_program();
        assert_eq!(0, machine.get_output());
    }

    #[test]
    fn test_position_mode_jump_zero() {
        let mut machine = IntcodeMachine::new(
            vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
            VecDeque::from(vec![0]),
        );
        machine.execute_program();
        assert_eq!(0, machine.get_output());
    }

    #[test]
    fn test_position_mode_jump_nonzero() {
        let mut machine = IntcodeMachine::new(
            vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
            VecDeque::from(vec![1]),
        );
        machine.execute_program();
        assert_eq!(1, machine.get_output());
    }

    #[test]
    fn test_immediate_mode_jump_zero() {
        let mut machine = IntcodeMachine::new(
            vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1],
            VecDeque::from(vec![0]),
        );
        machine.execute_program();
        assert_eq!(0, machine.get_output());
    }

    #[test]
    fn test_immediate_mode_jump_nonzero() {
        let mut machine = IntcodeMachine::new(
            vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1],
            VecDeque::from(vec![1]),
        );
        machine.execute_program();
        assert_eq!(1, machine.get_output());
    }

    #[test]
    fn test_big_input_less_than() {
        let mut machine = IntcodeMachine::new(
            vec![
                3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36,
                98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000,
                1, 20, 4, 20, 1105, 1, 46, 98, 99,
            ],
            VecDeque::from(vec![7]),
        );
        machine.execute_program();
        assert_eq!(999, machine.get_output());
    }

    #[test]
    fn test_big_input_equal() {
        let mut machine = IntcodeMachine::new(
            vec![
                3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36,
                98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000,
                1, 20, 4, 20, 1105, 1, 46, 98, 99,
            ],
            VecDeque::from(vec![8]),
        );
        machine.execute_program();
        assert_eq!(1000, machine.get_output());
    }

    #[test]
    fn test_big_input_greater_than() {
        let mut machine = IntcodeMachine::new(
            vec![
                3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36,
                98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000,
                1, 20, 4, 20, 1105, 1, 46, 98, 99,
            ],
            VecDeque::from(vec![9]),
        );
        machine.execute_program();
        assert_eq!(1001, machine.get_output());
    }

    #[test]
    fn test_day09_p1_error203_1() {
        let mut machine = IntcodeMachine::new(vec![109, -1, 4, 1, 99], VecDeque::from(vec![]));
        machine.execute_program();
        assert_eq!(-1, machine.get_output());
    }

    #[test]
    fn test_day09_p1_error203_2() {
        let mut machine = IntcodeMachine::new(vec![109, -1, 104, 1, 99], VecDeque::from(vec![]));
        machine.execute_program();
        assert_eq!(1, machine.get_output());
    }

    #[test]
    fn test_day09_p1_error203_3() {
        let mut machine = IntcodeMachine::new(vec![109, -1, 204, 1, 99], VecDeque::from(vec![]));
        machine.execute_program();
        assert_eq!(109, machine.get_output());
    }

    #[test]
    fn test_day09_p1_error203_4() {
        let mut machine =
            IntcodeMachine::new(vec![109, 1, 9, 2, 204, -6, 99], VecDeque::from(vec![]));
        machine.execute_program();
        assert_eq!(204, machine.get_output());
    }

    #[test]
    fn test_day09_p1_error203_5() {
        let mut machine =
            IntcodeMachine::new(vec![109, 1, 109, 9, 204, -6, 99], VecDeque::from(vec![]));
        machine.execute_program();
        assert_eq!(204, machine.get_output());
    }

    #[test]
    fn test_day09_p1_error203_6() {
        let mut machine =
            IntcodeMachine::new(vec![109, 1, 209, -1, 204, -106, 99], VecDeque::from(vec![]));
        machine.execute_program();
        assert_eq!(204, machine.get_output());
    }

    #[test]
    fn test_day09_p1_error203_7() {
        let mut machine =
            IntcodeMachine::new(vec![109, 1, 3, 3, 204, 2, 99], VecDeque::from(vec![123]));
        machine.execute_program();
        assert_eq!(123, machine.get_output());
    }

    #[test]
    fn test_day09_p1_error203_8() {
        let mut machine =
            IntcodeMachine::new(vec![109, 1, 203, 2, 204, 2, 99], VecDeque::from(vec![456]));
        machine.execute_program();
        assert_eq!(456, machine.get_output());
    }
}
