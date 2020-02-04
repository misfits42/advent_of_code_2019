use std::fs;
use std::collections::HashMap;

struct FftPattern {
    trailing_zeros: usize,
    pattern: Vec<i8>,
}

impl FftPattern {
    pub fn new(trailing_zeros: usize, pattern: Vec<i8>) -> Self {
        Self {
            trailing_zeros: trailing_zeros,
            pattern: pattern,
        }
    }

    pub fn get_value(&self, index: usize) -> i8 {
        if index < self.trailing_zeros {
            return 0;
        }
        return self.pattern[index - self.trailing_zeros];
    }

    pub fn size(&self) -> usize {
        return self.trailing_zeros + self.pattern.len();
    }
}

/// Calculates the solution to Day 16 Part 1 challenge.
pub fn solution_part_1(filename: String) -> String {
    return get_fft_result_string(filename, 1, 100);
}

/// Calculates the solution to Day 16 Part 2 challenge.
pub fn solution_part_2(filename: String) -> String {
    let input_digits = get_input_digits_from_filename(filename);
    let output = perform_fft(&input_digits, 10000, 100);
    let message_offset = get_message_offset(&input_digits);
    let message = get_message_with_offset_from_fft(output, message_offset);
    return message;
}

fn get_message_offset(initial_digits: &Vec<i64>) -> usize {
    let mut offset_string = String::from("");
    for i in 0..7 {
        offset_string.push_str(&initial_digits[i].to_string());
    }
    return offset_string.parse::<usize>().unwrap();
}

fn get_fft_result_string(filename: String, num_repeats_init: usize, num_phases: u64) -> String{
    let input_digits = get_input_digits_from_filename(filename);
    let output = perform_fft(&input_digits, num_repeats_init, num_phases);
    // Generate string of first out digits of output
    return get_message_with_offset_from_fft(output, 0);
}

fn get_message_with_offset_from_fft(fft_output: Vec<i64>, message_offset: usize) -> String {
    let mut message = String::from("");
    for i in 0..8 {
        message.push_str(&fft_output[i + message_offset].to_string());
    }
    return message;
}

fn get_input_digits_from_filename(filename: String) -> Vec<i64> {
    let mut raw_input = fs::read_to_string(filename).unwrap();
    raw_input = String::from(raw_input.trim());
    let input_digits: Vec<i64> = raw_input.chars().map(|x| x.to_digit(10).unwrap() as i64).collect();
    return input_digits;
}

fn generate_pattern(level: usize, length: usize) -> FftPattern {
    let basic_pattern = vec![0, 1, 0, -1];
    let mut basic_cycle = basic_pattern.iter().cycle();
    let mut pattern: Vec<i8> = vec![0; length - level + 1];
    let mut pattern_index = 0;
    let mut first_zeroes_seen = false;
    loop {
        loop {
            let next = *basic_cycle.next().unwrap();
            if next == 0 {
                if !first_zeroes_seen {
                    first_zeroes_seen = true;
                    continue;
                }
                pattern_index += level;
                if pattern_index >= pattern.len() {
                    return FftPattern::new(level-1, pattern);
                }
                continue;
            }
            for _ in 0..level {
                pattern[pattern_index] = next;
                pattern_index += 1;
                if pattern_index >= pattern.len() {
                    return FftPattern::new(level-1, pattern);
                }
            }
        }
    }
}

fn perform_fft(input_digits: &Vec<i64>, num_repeats: usize, num_phases: u64) -> Vec<i64> {
    let signal_length = input_digits.len() * num_repeats;
    println!("Number of input digits: {}", input_digits.len());
    println!("Number of repeats of input: {}", num_repeats);
    println!("Signal length: {}", signal_length);
    println!("Total number of levels to process: {}", signal_length * 100);
    let mut phase_output: Vec<i64> = Vec::with_capacity(signal_length);
    // Copy the input digits based on the specified number of repeats
    let mut phase_input = Vec::with_capacity(signal_length);
    for _ in 0..num_repeats {
        for i in 0..input_digits.len() {
            phase_input.push(input_digits[i]);
        }
    }
    let mut pattern_store: HashMap<usize, FftPattern> = HashMap::new();
    for phase in 0..num_phases {
        for level in 1..signal_length+1 {
            if level % 10 == 0 {
                println!("Starting Phase {} Level {}...", phase, level);
            }
            // Construct the pattern for current level
            if !pattern_store.contains_key(&level) {
                let new_pattern = generate_pattern(level, signal_length);
                pattern_store.insert(level, new_pattern);
            }
            let pattern = pattern_store.get(&level).unwrap();
            let mut output = 0;
            let mut i = 0;
            while i < pattern.size() {
                if pattern.get_value(i) == 0 {
                    let skip_amount = match i {
                        0 => level - 1,
                        _ => level,
                    };
                    i += skip_amount;
                    continue;
                }
                output += phase_input[i] * (pattern.get_value(i) as i64);
                i += 1;
            }
            phase_output.push(output.abs() % 10);
        }
        phase_input = phase_output.clone();
        phase_output.clear();
    }
    return phase_input;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_d16_p1_example_01() {
        let result = get_fft_result_string(String::from("./input/day_16/test/test_01.txt"), 1, 100);
        assert_eq!("24176176", result);
    }

    #[test]
    fn test_d16_p1_example_02() {
        let result = get_fft_result_string(String::from("./input/day_16/test/test_02.txt"), 1, 100);
        assert_eq!("73745418", result);
    }

    #[test]
    fn test_d16_p1_example_03() {
        let result = get_fft_result_string(String::from("./input/day_16/test/test_03.txt"), 1, 100);
        assert_eq!("52432133", result);
    }

    #[test]
    fn test_d16_p1_example_04() {
        let result = get_fft_result_string(String::from("./input/day_16/test/test_04.txt"), 1, 4);
        assert_eq!("01029498", result);
    }

    #[test]
    fn test_d16_p2_example_01() {
        let result = solution_part_2(String::from("./input/day_16/test/test_05.txt"));
        assert_eq!("84462026", result);
    }

    #[test]
    fn test_d16_p2_example_02() {
        let result = solution_part_2(String::from("./input/day_16/test/test_06.txt"));
        assert_eq!("78725270", result);
    }

    #[test]
    fn test_d16_p2_example_03() {
        let result = solution_part_2(String::from("./input/day_16/test/test_07.txt"));
        assert_eq!("53553731", result);
    }
}
