use std::fs::File;
use std::io::Read;
use std::error::Error;

/// Calculates solution for Day 04 Part 1 challenge.
pub fn solution_part_1(filename: String) -> i32 {
    // Open file
    let mut file: File = super::utils::fs::open_file(filename);
    // Read file line
    let mut input = String::new();
    match file.read_to_string(&mut input) {
        Err(e) => panic!("Error: {}", e.description()),
        Ok(_) => 0
    };
    let str_args: Vec<&str> = input.split("-").collect();
    if str_args.len() != 2 {
        panic!("Bad input!");
    }
    let lower_bound: i32 = str_args[0].parse::<i32>().unwrap();
    let upper_bound: i32 = str_args[1].parse::<i32>().unwrap();
    // Check password range
    let mut num_valid_passwords = 0;
    for pass_attempt in lower_bound..upper_bound+1 {
        let valid = check_password_validity(pass_attempt);
        if valid {
            num_valid_passwords += 1;
        }
    }
    return num_valid_passwords;
}

/// Calculates solution for Day 04 Part 2 challenge.
pub fn solution_part_2(filename: String) -> i32 {
    unimplemented!();
}

/// Checks the validity of the given password attempt using the conditions
/// specified in Day 04 Part 1 challenge.
fn check_password_validity(pass_attempt: i32) -> bool {
    let pass_str = pass_attempt.to_string();
    // Check if password attempt is six digits long
    if pass_str.len() != 6 {
        return false;
    }
    // Check if two adjacent digits are the same
    let mut seen_double_digit = false;
    for i in 0..(pass_str.len() - 1) {
        // Extract characters to compare
        let char_1 = pass_str[i..i+1].to_string();
        let char_2 = pass_str[i+1..i+2].to_string();
        if char_1 == char_2 {
            seen_double_digit = true;
            break;
        }
    }
    if !seen_double_digit {
        return false;
    }
    // Check if digits are monotonically increasing
    let mut last_digit = 0;
    for c in pass_str.chars() {
        let digit = c.to_digit(10).unwrap();
        if digit < last_digit {
            return false;
        }
        last_digit = digit;
    }
    // All conditions have been satisfied - so given password is valid.
    return true;
}

#[cfg(test)]
mod tests {
    /// Checks valid password consisting of all same digit.
    #[test]
    fn test_p1_example_input_1() {
        let result = super::check_password_validity(111111);
        assert_eq!(true, result);
    }

    /// Checks invalid password with reducing digits at end.
    #[test]
    fn test_p1_example_input_2() {
        let result = super::check_password_validity(223450);
        assert_eq!(false, result);
    }

    /// Checks invalid password with no double digits.
    #[test]
    fn test_p1_example_input_3() {
        let result = super::check_password_validity(123789);
        assert_eq!(false, result);
    }
}