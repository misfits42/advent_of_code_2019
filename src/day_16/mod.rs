use std::fs;

/// Calculates the solution to Day 16 Part 1 challenge.
pub fn solution_part_1(filename: String) -> String {
    return get_fft_result_string(filename, 100);
}

fn get_fft_result_string(filename: String, num_phases: u64) -> String{
    let input_digits = get_input_digits_from_filename(filename);
    let output = perform_fft(input_digits, num_phases);
    // Generate string of first out digits of output
    let mut result = String::from("");
    for i in 0..8 {
        result.push_str(&output[i].to_string());
    }
    return result;
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

fn perform_fft(input_digits: Vec<i64>, num_phases: u64) -> Vec<i64> {
    let mut phase_input = input_digits.clone();
    let mut phase_output: Vec<i64> = Vec::with_capacity(input_digits.len());
    for _ in 0..num_phases {
        for level in 1..phase_input.len()+1 {
            // Construct the pattern for current level
            let pattern = generate_pattern(level, phase_input.len());
            let mut output = 0;
            for i in 0..pattern.len() {
                output += phase_input[i] * pattern[i];
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
        let result = get_fft_result_string(String::from("./input/day_16/test/test_01.txt"), 100);
        assert_eq!("24176176", result);
    }

    #[test]
    fn test_d16_p1_example_02() {
        let result = get_fft_result_string(String::from("./input/day_16/test/test_02.txt"), 100);
        assert_eq!("73745418", result);
    }

    #[test]
    fn test_d16_p1_example_03() {
        let result = get_fft_result_string(String::from("./input/day_16/test/test_03.txt"), 100);
        assert_eq!("52432133", result);
    }

    #[test]
    fn test_d16_p1_example_04() {
        let result = get_fft_result_string(String::from("./input/day_16/test/test_04.txt"), 4);
        assert_eq!("01029498", result);
    }
}
