use std::io::Read;
use std::error::Error;
use super::utils::fs;
use super::utils::sif::SifImage;
use std::u32::*;

pub fn solution_part_1(filename: String) -> u32 {
    let mut file = fs::open_file(filename);
    let mut read_buf = String::from("");
    match file.read_to_string(&mut read_buf) {
        Err(e) => panic!("Error reading file. ({})", e.description()),
        Ok(_) => 0,
    };
    read_buf = String::from(read_buf.trim());
    // Load image
    let mut sif_image = SifImage::new(25, 6);
    sif_image.load_image_data(read_buf);
    // Check layers for one with fewest 0's
    let num_layers = sif_image.get_num_layers();
    let mut min_zero_count: u32 = MAX;
    let mut min_zero_layer_index: u32 = MAX;
    for layer_index in 0..num_layers {
        let num_zero = sif_image.get_layer_digit_count(layer_index, 0).unwrap();
        if num_zero < min_zero_count {
            min_zero_count = num_zero;
            min_zero_layer_index = layer_index;
        }
    }
    // Calculate result
    let num_1 = sif_image.get_layer_digit_count(min_zero_layer_index, 1).unwrap();
    let num_2 = sif_image.get_layer_digit_count(min_zero_layer_index, 2).unwrap();
    let result = num_1 * num_2;
    return result;
}

// pub fn solution_part_2(filename: String) -> i32 {
//     unimplemented!();
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_p1_actual_solution() {
        let result = solution_part_1(String::from("./input/day_08/input.txt"));
        assert_eq!(1463, result);
    }

    #[test]
    #[ignore]
    fn test_p2_actual_solution() {
        unimplemented!();
    }
}