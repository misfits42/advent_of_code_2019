use super::utils::intcode::IntcodeMachine;
use std::collections::VecDeque;
use std::collections::HashMap;

// const TILE_EMPTY: u64 = 0;
// const TILE_WALL: u64 = 1;
const TILE_BLOCK: u64 = 2;
// const TILE_H_PADDLE: u64 = 3;
// const TILE_BALL: u64 = 4;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: i64,
    y: i64
}

/// Calculates the solution for Day 13 Part 1.
pub fn solution_part_1(filename: String) -> u64 {
    let initial_memory = IntcodeMachine::extract_intcode_memory_from_filename(filename);
    let mut arcade_machine = IntcodeMachine::new(initial_memory.clone(), VecDeque::from(vec![]));
    let mut screen = HashMap::<Point, u64>::new();
    loop {
        // Should be able to run machine and break on output at least three times validly
        for _ in 0..3 {
            arcade_machine.execute_program_break_on_output(true);
        }
        // Check if the arcade machine has halted
        if arcade_machine.has_halted() {
            if arcade_machine.get_output_vec().len() != 0 {
                panic!("Arcade machine output is not empty after halting!");
            }
            break;
        }
        let x_pos = arcade_machine.get_output_and_remove();
        let y_pos = arcade_machine.get_output_and_remove();
        let tile_id = arcade_machine.get_output_and_remove();
        // Check validity of output values
        if x_pos < 0 {
            panic!(format!("Bad x-pos: {}", x_pos));
        }
        if y_pos < 0 {
            panic!(format!("Bad y-pos: {}", y_pos));
        }
        if tile_id < 0 || tile_id > 4 {
            panic!(format!("Bad tile id: {}", tile_id));
        }
        let position = Point{x: x_pos, y: y_pos};
        screen.insert(position, tile_id as u64);
    }
    // Count the number of block tiles on screen when machine halts
    let mut block_count: u64 = 0;
    for (_, tile_id) in screen.into_iter() {
        if tile_id == TILE_BLOCK {
            block_count += 1;
        }
    }
    return block_count;
}
