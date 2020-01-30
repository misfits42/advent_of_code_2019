use super::utils::intcode::IntcodeMachine;
use std::collections::VecDeque;
use std::collections::HashMap;

// Tile IDs
const TILE_EMPTY: i64 = 0;
// const TILE_WALL: i64 = 1;
const TILE_BLOCK: i64 = 2;
const TILE_H_PADDLE: i64 = 3;
const TILE_BALL: i64 = 4;

// Joystick move options
const JOYSTICK_NEUTRAL: i64 = 0;
const JOYSTICK_LEFT: i64 = -1;
const JOYSTICK_RIGHT: i64 = 1;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: i64,
    y: i64
}

/// Calculates the solution for Day 13 Part 1.
pub fn solution_part_1(filename: String) -> u64 {
    let initial_memory = IntcodeMachine::extract_intcode_memory_from_filename(filename);
    let mut arcade_machine = IntcodeMachine::new(initial_memory.clone(), VecDeque::from(vec![]));
    let mut screen = HashMap::<Point, i64>::new();
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
        screen.insert(position, tile_id);
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

pub fn solution_part_2(filename: String) -> i64 {
    let mut initial_memory = IntcodeMachine::extract_intcode_memory_from_filename(filename);
    // Insert 2 quarters to play for free
    initial_memory[0] = 2;
    let mut arcade_machine = IntcodeMachine::new(initial_memory.clone(), VecDeque::from(vec![]));
    let mut score = 0;
    let mut ball_positions = Vec::<Point>::from(vec![]);
    let mut paddle_positions: Vec<Point> = vec![];
    let mut block_positions = Vec::<Point>::from(vec![]);
    let mut screen = HashMap::<Point, i64>::new();
    loop {
        // Run machine to get output triple
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
        // Check if machine is awaiting input
        if arcade_machine.is_awaiting_input() {
            // Check what direction to move the joystick
            if ball_positions.is_empty() {
                panic!("Don't know how to move paddle - no ball positions observed.");
            }
            // If we haven't seen the paddle yet, don't try to move it
            if paddle_positions.is_empty() {
                arcade_machine.add_input(JOYSTICK_NEUTRAL);
            }
            // Ball is to left of paddle
            if ball_positions.last().unwrap().x < paddle_positions.first().unwrap().x {
                arcade_machine.add_input(JOYSTICK_LEFT);
            // Ball is to right of paddle
            } else if ball_positions.last().unwrap().x > paddle_positions.last().unwrap().x {
                arcade_machine.add_input(JOYSTICK_RIGHT);
            // Ball is above one of the paddle elements
            } else {
                arcade_machine.add_input(JOYSTICK_NEUTRAL);
            }
            continue;
        }
        let output_1 = arcade_machine.get_output_and_remove();
        let output_2 = arcade_machine.get_output_and_remove();
        let output_3 = arcade_machine.get_output_and_remove();
        // Check if score has been updated
        if output_1 == -1 && output_2 == 0 {
            score = output_3;
            continue;
        }
        // Tile has been updated, so update the screen
        let position = Point{x: output_1, y: output_2};
        screen.insert(position, output_3);
        // Check if ball position has been updated
        if output_3 == TILE_BALL {
            ball_positions.push(position);
            continue;
        }
        // Check if paddle position has been updated
        if output_3 == TILE_H_PADDLE {
            paddle_positions.push(position);
            // sort paddle by xvar
            paddle_positions.sort_by(|a, b| a.x.cmp(&b.x));
            continue;
        }
        // Check if empty has overridden block or paddle
        if output_3 == TILE_EMPTY {
            // Empty tile has been written where paddle tile previously was
            if paddle_positions.contains(&position) {
                for i in 0..paddle_positions.len() {
                    if paddle_positions[i] == position {
                        paddle_positions.remove(i);
                    }
                }
            // Block has been destroyed
            } else if block_positions.contains(&position) {
                for i in 0..block_positions.len() {
                    if block_positions[i] == position {
                        block_positions.remove(i);
                    }
                }
            }
            continue;
        }
    }
    return score;
}
