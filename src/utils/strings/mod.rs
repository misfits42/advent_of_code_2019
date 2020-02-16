/// Checks if the input string is solely made up of a repeating substring. If so, the root
/// substring is returned.
pub fn find_repeat_substring(input: String, max_root_subtring_len: usize) -> String {
    let size_limit = match max_root_subtring_len {
        0 => input.len(),
        _ => max_root_subtring_len,
    };
    for end in 0..size_limit {
        let check_input = input.clone().replace(&input[0..end], "");
        if check_input.is_empty() {
            return input[0..end].to_owned();
        }
    }
    return String::new();
}