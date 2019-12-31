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
    // Extract arguments
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
    // Open file
    let mut file: File = super::utils::fs::open_file(filename);
    // Read file line
    let mut input = String::new();
    match file.read_to_string(&mut input) {
        Err(e) => panic!("Error: {}", e.description()),
        Ok(_) => 0,
    };
    // Extract arguments
    let str_args: Vec<&str> = input.split("-").collect();
    if str_args.len() != 2 {
        panic!("Bad input!");
    }
    let lower_bound: i32 = str_args[0].parse::<i32>().unwrap();
    let upper_bound: i32 = str_args[1].parse::<i32>().unwrap();
    // Check password range
    let mut num_valid_passwords = 0;
    for pass_attempt in lower_bound..upper_bound+1 {
        let valid = check_password_validity_part2(pass_attempt);
        if valid {
            num_valid_passwords += 1;
        }
    }
    return num_valid_passwords;
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

/// Checks the validity of the given password attempt, using all the Day 04 Part 1
/// and Part 2 conditions.
fn check_password_validity_part2(pass_attempt: i32) -> bool {
    // Perform checks in first pass of password checker
    if !check_password_validity(pass_attempt) {
        return false;
    }
    // Perform additional check - double digit not part of larger grouping
    let mut repeat_streak = 1; // Default streak of 1 for single digit
    let pass_str = pass_attempt.to_string();
    // Iterate up to but not including 1 less then string length - so we don't index out of bounds
    for i in 0..(pass_str.len() - 1) {
        // Extract characters to compare
        let char_1 = pass_str[i..i+1].to_string();
        let char_2 = pass_str[i+1..i+2].to_string();
        // We have a match - increment the streak count
        if char_1 == char_2 {
            repeat_streak += 1;
        } else { // Streak broken
            // Streak broken after finding double digit - valid password
            if repeat_streak == 2 {
                return true;
            }
            // Otherwise, reset the streak count
            repeat_streak = 1;
        }
    }
    // Check if double digit occurred at end of password
    if repeat_streak == 2 {
        return true;
    }
    // Required condition not met - password invalid
    return false;
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

    /// Checks valid input for Part 2 password validity checker.
    #[test]
    fn test_p2_example_input_1() {
        let result = super::check_password_validity_part2(112233);
        assert_eq!(true, result);
    }

    /// Checks invalid password with only steak of three digits.
    #[test]
    fn test_p2_example_input_2() {
        let result = super::check_password_validity_part2(123444);
        assert_eq!(false, result);
    }

    /// Checks valid password with double digit and another streak of four identical digits.
    #[test]
    fn test_p2_example_input_3() {
        let result = super::check_password_validity_part2(111122);
        assert_eq!(true, result);
    }
}