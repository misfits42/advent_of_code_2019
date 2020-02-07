use std::fs;

#[derive(Copy, Clone, PartialEq)]
struct FftRangeSum {
    pattern_value: i64,
    left_index: usize,
    right_index: usize,
    sum: i64,
}

impl FftRangeSum {
    pub fn new(pattern_value: i64, init_index: usize, initial_sum: i64) -> Self {
        Self {
            pattern_value: pattern_value,
            left_index: init_index,
            right_index: init_index,
            sum: initial_sum,
        }
    }

    pub fn get_right_index(&self) -> usize {
        return self.right_index;
    }

    pub fn get_pattern_value(&self) -> i64 {
        return self.pattern_value;
    }

    pub fn get_sum(&self) -> i64 {
        return self.sum;
    }

    pub fn get_range_length(&self) -> usize {
        return self.right_index - self.left_index + 1;
    }

    /// Expands the FftRangeSum to the left and adds the new value on the left to its sum;
    pub fn expand_left(&mut self, phase_input: &Vec<i64>) -> i64 {
        self.left_index -= 1;
        self.sum += phase_input[self.left_index] * self.pattern_value;
        return self.sum;
    }

    pub fn shift_left_no_shrink(&mut self, phase_input: &Vec<i64>) -> i64 {
        self.left_index -= 1;
        self.right_index -= 1;
        let subtract_amt = phase_input[self.right_index + 1] * self.pattern_value;
        self.sum -= subtract_amt;
        self.sum += phase_input[self.left_index] * self.pattern_value;
        return self.sum;
    }

    pub fn shift_left_and_shrink(&mut self, phase_input: &Vec<i64>) -> i64 {
        self.left_index -= 1;
        self.right_index -= 2;
        // Work out how much to adjust sum by from the two values shifted out of range.
        let subtract_amt = (phase_input[self.right_index + 1] + phase_input[self.right_index + 2]) * self.pattern_value;
        self.sum -= subtract_amt;
        // Add the new value shifted into on the left
        self.sum += phase_input[self.left_index] * self.pattern_value;
        return self.sum;
    }
}

#[allow(dead_code)]
struct FftPattern {
    level: usize,
    signal_length: usize,
}

#[allow(dead_code)]
impl FftPattern {
    pub fn new(level: usize, signal_length: usize) -> Self {
        Self {
            level: level,
            signal_length: signal_length,
        }
    }

    pub fn get_value(&self, index: usize) -> i64 {
        if index < self.level * 4 - 1 {
            // Trailing zeroes
            if index < self.level - 1 {
                return 0;
            }
            let remainder = (index - (self.level - 1)) % (3 * self.level);
            if remainder < self.level {
                return 1;
            } else if remainder < self.level * 2 {
                return 0;
            } else if remainder < self.level * 3 {
                return -1;
            }
            panic!("Shouldn't reach here!");
        }
        let remainder = (index - (4 * self.level - 1)) % (self.level * 4);
        if remainder < self.level {
            return 0;
        } else if remainder < self.level * 2 {
            return 1;
        } else if remainder < self.level * 3 {
            return 0;
        } else if remainder < self.level * 4 {
            return -1;
        }
        panic!("Mathematically shouldn't reach here!");
    }

    pub fn size(&self) -> usize {
        return self.signal_length;
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

/// Gets the message offset from the initial signal. Message offset is encoded as an integer value
/// represented by the first 7 digits.
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
    return FftPattern::new(level, length);
}

fn perform_fft(input_digits: &Vec<i64>, num_repeats: usize, num_phases: u64) -> Vec<i64> {
    let signal_length = input_digits.len() * num_repeats;
    println!("Number of input digits: {}", input_digits.len());
    println!("Number of repeats of input: {}", num_repeats);
    println!("Signal length: {}", signal_length);
    println!("Total number of levels to process: {}", signal_length * 100);
    // Input and output vectors will be cyclically rotated through so output isn't cloned at end of
    // phase.
    let mut phase_data: Vec<Vec<i64>> = vec![vec![0; signal_length]; 3];
    let mut last_pattern_value_seen: i64 = 0;
    // Copy input digits into initial phase input
    let mut in_index = 0;
    for repeat in 0..num_repeats {
        for i in 0..input_digits.len() {
            phase_data[0][i * (repeat + 1)] = input_digits[i];
        }
    }
    for phase in 0..num_phases {
        let mut range_sums: Vec<FftRangeSum> = vec![];
        let out_index = (in_index + 1) % 3;
        for level in (1..signal_length + 1).rev() {
            if level % 1000 == 0 {
                println!("Starting Phase {} Level {}...", phase, level);
            }
            let pattern = generate_pattern(level, signal_length);
            let end_pattern_value = pattern.get_value(signal_length - 1);
            let mut output = 0;

            if 3 * level - 1 < signal_length { // Beyond point where more then one non-zero stripe occurs
                for i in 0..range_sums.len() {
                    output += range_sums[i].shift_left_and_shrink(&phase_data[in_index]);
                }
                let mut level_index = range_sums.last().unwrap().get_right_index() + 1;
                while level_index < pattern.size() {
                    let pattern_value = pattern.get_value(level_index);
                    if pattern_value == 0 {
                        let skip_amount = match level_index {
                            0 => level - 1,
                            _ => level,
                        };
                        level_index += skip_amount;
                        continue;
                    } else if pattern_value == -1 {
                        let mut temp = 0;
                        for inc in 0..level {
                            let index_1 = level_index + inc;
                            let index_2 = index_1 + 2 * level;
                            if index_2 >= signal_length && index_1 >= signal_length {
                                break;
                            } else if index_2 >= signal_length {
                                temp -= phase_data[in_index][index_1];
                            } else {
                                temp -= phase_data[in_index][index_1] - phase_data[in_index][index_2];
                            }
                        }
                        output += temp;
                        level_index += level * 4;
                    } else {
                        panic!("Shouldn't get here!");
                    }
                }
            } else {
                if end_pattern_value == 0 { // All ranges to shift left
                    last_pattern_value_seen = 0;
                    for i in 0..range_sums.len() {
                        if range_sums[i].get_range_length() == level {
                            output += range_sums[i].shift_left_no_shrink(&phase_data[in_index]);
                        } else {
                            output += range_sums[i].shift_left_and_shrink(&phase_data[in_index]);
                        }
                    }
                } else {
                    if last_pattern_value_seen == 0 { // Add new range sum to end of vector
                        last_pattern_value_seen = end_pattern_value;
                        let initial_sum = phase_data[in_index][signal_length - 1] * end_pattern_value;
                        let new_fft_range_sum = FftRangeSum::new(end_pattern_value, signal_length - 1, initial_sum);
                        output += new_fft_range_sum.get_sum();
                        range_sums.push(new_fft_range_sum);
                    } else if last_pattern_value_seen == range_sums.last().unwrap().get_pattern_value() {
                        // expand the last range sum
                        output += range_sums.last_mut().unwrap().expand_left(&phase_data[in_index]);
                    }
                    // Shift existing range sums
                    for i in 0..(range_sums.len() - 1) {
                        output += range_sums[i].shift_left_and_shrink(&phase_data[in_index]);
                    }
                }
            }
            phase_data[out_index][level - 1] = output.abs() % 10;
        }
        in_index = out_index;
    }
    return phase_data[in_index].clone();
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
