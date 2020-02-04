use std::fs;

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

fn generate_pattern(level: usize) -> Vec<i64> {
    let basic_pattern = vec![0, 1, 0, -1];
    let mut basic_cycle = basic_pattern.iter().cycle();
    let target_pattern_length = basic_pattern.len() * level;
    let mut pattern: Vec<i64> = Vec::with_capacity(target_pattern_length);
    loop {
        let next = *basic_cycle.next().unwrap();
        pattern.push(next);
        if pattern.len() == target_pattern_length {
            return pattern;
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
    let mut phase_input: Vec<i64> = Vec::with_capacity(signal_length);
    for _ in 0..num_repeats {
        for input_index in 0..input_digits.len() {
            phase_input.push(input_digits[input_index]);
        }
    }
    for phase in 0..num_phases {
        for level in 1..signal_length+1 {
            if level % 1 == 0 {
                println!("Starting Phase {} Level {}...", phase, level);
            }
            // Construct the pattern for current level
            let basic_pattern = generate_pattern(level);
            let mut pattern_cycle = basic_pattern.iter().cycle();
            pattern_cycle.next();
            let mut output = 0;
            let mut pattern_index = 0;
            while pattern_index < signal_length {
                let pattern_val = *pattern_cycle.next().unwrap();
                if pattern_val == 0 {
                    let skip_amount = match pattern_index {
                        0 => level - 1,
                        _ => level,
                    };
                    for _ in 0..skip_amount {
                        pattern_cycle.next();
                    }
                    continue;
                }
                let input = phase_input[pattern_index];
                output += input * pattern_val;
                pattern_index += 1;
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

    #[ignore]
    #[test]
    fn test_d16_p2_example_01() {
        let result = solution_part_2(String::from("./input/day_16/test/test_05.txt"));
        assert_eq!("84462026", result);
    }

    #[ignore]
    #[test]
    fn test_d16_p2_example_02() {
        let result = solution_part_2(String::from("./input/day_16/test/test_06.txt"));
        assert_eq!("78725270", result);
    }

    #[ignore]
    #[test]
    fn test_d16_p2_example_03() {
        let result = solution_part_2(String::from("./input/day_16/test/test_07.txt"));
        assert_eq!("53553731", result);
    }
}
