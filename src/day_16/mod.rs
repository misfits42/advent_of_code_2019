use std::fs;

#[derive(Copy, Clone, PartialEq)]
struct FftRangeSum {
    pattern_value: i64,
    left_index: usize,
    right_index: usize,
    sum: i64,
}

impl FftRangeSum {
    /// Creates a new FftRangeSum initialised with the given parameters.
    pub fn new(pattern_value: i64, left_index: usize, right_index: usize, initial_sum: i64) -> Self {
        Self {
            pattern_value: pattern_value,
            left_index: left_index,
            right_index: right_index,
            sum: initial_sum,
        }
    }

    /// Gets the left index of the FftRangeSum.
    pub fn get_left_index(&self) -> usize {
        return self.left_index;
    }

    /// Gets the right index of the FftRangeSum.
    pub fn get_right_index(&self) -> usize {
        return self.right_index;
    }

    /// Gets the pattern value associated with the FftRangeSum.
    pub fn get_pattern_value(&self) -> i64 {
        return self.pattern_value;
    }

    /// Gets the current sum of the input values encompassed by this FftRangeSum.
    pub fn get_sum(&self) -> i64 {
        return self.sum;
    }

    /// Calculates the size of the range, with single digit ranges having a size of 1.
    pub fn get_range_length(&self) -> usize {
        return self.right_index - self.left_index + 1;
    }

    /// Shifts the range to the left by 1 without reducing its size.
    pub fn shift_left_no_shrink(&mut self, phase_input: &Vec<i64>) -> i64 {
        let new_left = self.left_index - 1;
        let new_right = self.right_index - 1;
        return self.update_left_and_right(new_left, new_right, phase_input);
    }

    /// Shifts the left bound further left by the specified amount, with the right index being
    /// shifted left by the 1 + specified amount to shrink the length by 1.
    pub fn shift_left_and_shrink(&mut self, shift_by: usize, phase_input: &Vec<i64>) -> i64 {
        let new_left = self.left_index - shift_by;
        let new_right = self.right_index - shift_by - 1;
        return self.update_left_and_right(new_left, new_right, phase_input);
    }

    /// Expands the FftRangeSum to the left and adds the new value on the left to its sum;
    pub fn expand_left(&mut self, expand_by: usize, phase_input: &Vec<i64>) -> i64 {
        let new_left = self.left_index - expand_by;
        return self.update_left_and_right(new_left, self.right_index, phase_input);
    }

    /// Updates the left and right indices to the given values. Panics if the new right index is
    /// less than the new left index.
    pub fn update_left_and_right(
        &mut self,
        new_left: usize,
        new_right: usize,
        phase_input: &Vec<i64>,
    ) -> i64 {
        if new_right < new_left {
            panic!("Bad indices - right index cannot be less than left index.");
        }
        // Calculate the amount by which the left and right bounds are to be shifted
        let shift_left = self.left_index - new_left;
        let shift_right = self.right_index - new_right;
        let old_left = self.left_index;
        // Update the left and right bounds
        self.left_index = new_left;
        self.right_index = new_right;
        // If the new range has no elements in common with previous, just calculate new sum
        if new_right < old_left {
            let mut new_sum = 0;
            for i in self.left_index..self.right_index + 1 {
                new_sum += phase_input[i] * self.pattern_value;
            }
            self.sum = new_sum;
            return self.sum;
        }
        // Calculate subtract amount
        let mut subtract_amt = 0;
        for i in self.right_index..(self.right_index + shift_right + 1) {
            if i >= phase_input.len() - 1 {
                continue;
            }
            subtract_amt += phase_input[i + 1];
        }
        subtract_amt *= self.pattern_value;
        self.sum -= subtract_amt;
        // Calculate add amount
        let mut add_amt = 0;
        for i in self.left_index..(self.left_index + shift_left) {
            add_amt += phase_input[i];
        }
        add_amt *= self.pattern_value;
        self.sum += add_amt;
        return self.sum;
    }
}

/// This struct is used to represent a pattern at a particular level when processing a signal via
/// the Day 16 FFT method.
struct FftPattern {
    level: usize,
}

impl FftPattern {
    pub fn new(level: usize) -> Self {
        Self { level: level }
    }

    /// Calculates the value occuring at the given index in the pattern.
    pub fn get_value(&self, index: usize) -> i64 {
        // Index is within first repeat of basic pattern
        if index < self.level * 4 - 1 {
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
        // Index 
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
}

/// Calculates the solution to Day 16 Part 1 challenge.
pub fn solution_part_1(filename: String) -> String {
    return get_fft_result_string(filename, 1, 100);
}

/// Calculates the solution to Day 16 Part 2 challenge.
pub fn solution_part_2(filename: String) -> String {
    let input_digits = get_input_signal_from_filename(filename);
    let message_offset = get_message_offset(&input_digits);
    let output = perform_fft(&input_digits, 10000, 100);
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

fn get_fft_result_string(filename: String, num_repeats_init: usize, num_phases: u64) -> String {
    let input_digits = get_input_signal_from_filename(filename);
    let output = perform_fft(&input_digits, num_repeats_init, num_phases);
    // Generate string of first out digits of output
    return get_message_with_offset_from_fft(output, 0);
}

/// Extracts the eight-character message offset by the specifed value from the given FFT output.
fn get_message_with_offset_from_fft(fft_output: Vec<i64>, message_offset: usize) -> String {
    let mut message = String::from("");
    for i in 0..8 {
        message.push_str(&fft_output[i + message_offset].to_string());
    }
    return message;
}

/// Extracts the input signal from the given filename.
fn get_input_signal_from_filename(filename: String) -> Vec<i64> {
    let mut raw_input = fs::read_to_string(filename).unwrap();
    raw_input = String::from(raw_input.trim());
    let input_digits: Vec<i64> = 
        raw_input.chars().map(|x| x.to_digit(10).unwrap() as i64).collect();
    return input_digits;
}

/// Executes the Day 16 FFT algorithm using the input digits repeated the specified number of times
/// as the input signal to the first phase.
fn perform_fft(input_digits: &Vec<i64>, num_repeats: usize, num_phases: u64) -> Vec<i64> {
    let signal_length = input_digits.len() * num_repeats;
    // println!("Number of input digits: {}", input_digits.len());
    // println!("Number of repeats of input: {}", num_repeats);
    // println!("Signal length: {}", signal_length);
    // println!("Total number of levels to process: {}", signal_length * 100);
    let mut phase_output: Vec<i64> = vec![0; signal_length];
    let mut phase_input: Vec<i64> = vec![0; signal_length];
    // Copy input digits into initial phase input
    for repeat in 0..num_repeats {
        for i in 0..input_digits.len() {
            let index = repeat * input_digits.len() + i;
            phase_input[index] = input_digits[i];
        }
    }
    let mut last_pattern_value_seen: i64 = 0;
    for phase in 0..num_phases {
        let mut range_sums: Vec<FftRangeSum> = vec![];
        for level in (1..signal_length + 1).rev() {
            if level % 100 == 0 {
                println!("Starting Phase {} Level {}...", phase + 1, level);
            }
            let pattern = FftPattern::new(level);
            let end_pattern_value = pattern.get_value(signal_length - 1);
            let mut output = 0;
            // Additional non-zero strips occur at and beyond this point - need to add more ranges
            if 3 * level - 1 < signal_length {
                for i in 0..range_sums.len() {
                    // Calculate how much the existing range needs to have left index shifted
                    let shift_amt = (i + 1) * 2 - 1;
                    // Calculate how long range would be if left index was moved without right also
                    let new_expand_len = range_sums[i].get_range_length() + shift_amt;
                    if range_sums[i].get_right_index() == signal_length - 1
                        && new_expand_len > level
                    {
                        let new_left = range_sums[i].get_left_index() - shift_amt;
                        let new_right = new_left + level - 1;
                        output +=
                            range_sums[i].update_left_and_right(new_left, new_right, &phase_input);
                    } else if range_sums[i].get_range_length() + shift_amt < level {
                        output += range_sums[i].expand_left(shift_amt, &phase_input);
                    } else {
                        output += range_sums[i].shift_left_and_shrink(shift_amt, &phase_input);
                    }
                }
                // Add new ranges
                let mut left_index = range_sums.last().unwrap().get_right_index() + level + 1;
                while left_index < signal_length {
                    let mut right_index = left_index + level - 1;
                    if right_index >= signal_length {
                        right_index = signal_length - 1;
                    }
                    let pattern_value = pattern.get_value(left_index);
                    let mut init_sum = 0;
                    for i in left_index..right_index + 1 {
                        init_sum += phase_input[i];
                    }
                    init_sum *= pattern_value;
                    let new_fft_range_sum =
                        FftRangeSum::new(pattern_value, left_index, right_index, init_sum);
                    // Add sum from new range to output
                    output += new_fft_range_sum.get_sum();
                    range_sums.push(new_fft_range_sum);
                    // Move the left index along
                    left_index = right_index + level + 1;
                }
            } else {
                if end_pattern_value == 0 {
                    last_pattern_value_seen = 0;
                    for i in 0..range_sums.len() {
                        let shift_amt = (i + 1) * 2 - 1;
                        if range_sums[i].get_range_length() == level {
                            output += range_sums[i].shift_left_no_shrink(&phase_input);
                        } else {
                            output += range_sums[i].shift_left_and_shrink(shift_amt, &phase_input);
                        }
                    }
                } else {
                    if last_pattern_value_seen == 0 {
                        // Add new range sum to end of vector
                        last_pattern_value_seen = end_pattern_value;
                        let initial_sum = phase_input[signal_length - 1] * end_pattern_value;
                        let new_fft_range_sum = FftRangeSum::new(
                            end_pattern_value,
                            signal_length - 1,
                            signal_length - 1,
                            initial_sum,
                        );
                        output += new_fft_range_sum.get_sum();
                        range_sums.push(new_fft_range_sum);
                    } else if last_pattern_value_seen
                        == range_sums.last().unwrap().get_pattern_value()
                    {
                        // expand the last range sum
                        output += range_sums.last_mut().unwrap().expand_left(1, &phase_input);
                    }
                    // Shift existing range sums
                    for i in 0..(range_sums.len() - 1) {
                        let shift_amt = (i + 1) * 2 - 1;
                        output += range_sums[i].shift_left_and_shrink(shift_amt, &phase_input);
                    }
                }
            }
            phase_output[level - 1] = output.abs() % 10;
        }
        phase_input = phase_output.clone();
    }
    return phase_output;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_d16_p1_solution() {
        let result = solution_part_1(String::from("./input/day_16/input.txt"));
        assert_eq!("27229269", result);
    }

    #[test]
    fn test_d16_p2_solution() {
        let result = solution_part_2(String::from("./input/day_16/input.txt"));
        assert_eq!("26857164", result);
    }

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
