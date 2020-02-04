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

fn generate_pattern(level: usize, length: usize) -> Vec<i64> {
    let basic_pattern = vec![0, 1, 0, -1];
    let mut basic_cycle = basic_pattern.iter().cycle();
    let mut pattern: Vec<i64> = Vec::with_capacity(length);
    let mut first_ignored = false;
    let mut elements_added = 0;
    loop {
        let next = *basic_cycle.next().unwrap();
        for _ in 0..level {
            if !first_ignored {
                first_ignored = true;
                continue;
            }
            pattern.push(next);
            elements_added += 1;
            if elements_added == length {
                return pattern;
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
    let mut phase_input = phase_output.clone();
    let mut input_cycle = input_digits.iter().cycle(); // For input in first phase
    for phase in 0..num_phases {
        for level in 1..signal_length+1 {
            if level % 10 == 0 {
                println!("Starting Phase {} Level {}...", phase, level);
            }
            // Construct the pattern for current level
            let pattern = generate_pattern(level, signal_length);
            let mut output = 0;
            for i in 0..pattern.len() {
                let input = match phase {
                    0 => *input_cycle.next().unwrap(),
                    _ => phase_input[i],
                };
                output += input * pattern[i];
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
