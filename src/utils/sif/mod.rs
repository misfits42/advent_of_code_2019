//! # sif - Space Image Format
//! 
//! This module contains code used to represent the Space Image Format data first encountered in
//! AoC 2019 Day 8.

use std::collections::HashMap;

/// This struct is used to represent a SIF (Space Image Format) image. Format was first introduced
/// in AoC 2019 Day 08 Part 1.
pub struct SifImage {
    image_width: u32,
    image_height: u32,
    num_layers: u32,
    valid_image_loaded: bool,
    layer_digit_counts: HashMap<u32, HashMap<u32, u32>>,
    digits_map: HashMap<u32, Vec<Vec<u32>>>,
}

impl SifImage {
    pub fn new(image_width: u32, image_height: u32) -> Self {
        Self {
            image_width: image_width,
            image_height: image_height,
            num_layers: 0,
            valid_image_loaded: false,
            layer_digit_counts: HashMap::<u32, HashMap<u32, u32>>::new(),
            digits_map: HashMap::<u32, Vec<Vec<u32>>>::new(),
        }
    }

    pub fn get_layer_area(&self) -> u32 {
        return self.image_height * self.image_width;
    }

    pub fn get_num_layers(&self) -> u32 {
        return self.num_layers;
    }

    pub fn get_layer_digit_count(&self, layer_index: u32, digit: u32) -> Result<u32, &'static str> {
        // Check inputs
        if layer_index >= self.num_layers {
            return Err("Layer index exceeds number of layers.");
        }
        if digit >= 10 {
            return Err("Digit exceeds max. allowed value.");
        }
        return Ok(self.layer_digit_counts[&layer_index][&digit]);
    }

    pub fn load_image_data(&mut self, raw_image_data: String) {
        // Get vector of individual characters
        let image_chars: Vec<char> = raw_image_data.chars().collect();
        // Calculate total number of layers
        let total_digits = image_chars.len() as u32;
        self.num_layers = total_digits / self.get_layer_area();
        // Initialise digit count for layers
        for layer in 0..self.num_layers {
            let mut blank_count = HashMap::<u32, u32>::new();
            for digit in 0..10 {
                blank_count.insert(digit, 0);
            }
            self.layer_digit_counts.insert(layer, blank_count);
            self.digits_map.insert(layer, vec![vec![0; self.image_width as usize]; self.image_height as usize]);
        }
        // Process digits
        for digit_index in 0..total_digits {
            // Calculate layer number
            let layer = digit_index / self.get_layer_area();
            // Calculate X var
            let x_var = (digit_index % self.get_layer_area()) % self.image_width;
            // Calculate Y var
            let y_var = (digit_index % self.get_layer_area()) / self.image_width;
            // Parse digit value
            let digit_value = image_chars[digit_index as usize].to_digit(10).unwrap();
            // Update layer digit count
            if let Some(count_map) = self.layer_digit_counts.get_mut(&layer) {
                if let Some(count) = count_map.get_mut(&digit_value) {
                    *count += 1;
                }
            }
            // Add digit to image map
            if let Some(layer_map) = self.digits_map.get_mut(&layer) {
                // Index in reverse order with 2D array
                layer_map[y_var as usize][x_var as usize] = digit_value;
            }
        }
        self.valid_image_loaded = true;
    }
}
