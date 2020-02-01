use std::collections::HashMap;
use std::io::Read;
use std::usize::*;

// Config parameters
const GRID_X: usize = 99000;
const GRID_Y: usize = 99000;
const GRID_ORD_MAG: usize = 100000;
const X_VAR: usize = 0;
const Y_VAR: usize = 1;

// Directions
const DIR_RIGHT: char = 'R';
const DIR_LEFT: char = 'L';
const DIR_UP: char = 'U';
const DIR_DOWN: char = 'D';

// Grid values
const GRID_WIRE_1: usize = 1;
const GRID_WIRE_2: usize = 2;

/// Calculates and displays the solution for Day 03 Part 1 challenge.
pub fn solution_part_1(filename: String) -> usize {
    // Open file
    let mut file = super::utils::fs::open_file(filename);
    // Read file lines
    let mut file_str = String::new();
    match file.read_to_string(&mut file_str) {
        Err(e) => panic!("Failed to read file to string. ({})", e.to_string()),
        Ok(_) => 0,
    };
    // Separate out the two wires given in input file
    let mut lines = file_str.lines();
    let wire_one: Vec<&str> = lines.next().unwrap().split(',').collect();
    let wire_two: Vec<&str> = lines.next().unwrap().split(',').collect();
    // Initialise variables required for processing wires
    let mut intersections = Vec::<Vec<usize>>::new();
    let mut wire_move_records = HashMap::<usize, HashMap<usize, usize>>::new();
    let start_loc: Vec<usize> = vec![GRID_X / 2, GRID_Y / 2];
    // Process wire one
    let wire_one_move_record = process_wire_nogrid(
        wire_one,
        &wire_move_records,
        &mut intersections,
        &start_loc,
    );
    wire_move_records.insert(GRID_WIRE_1, wire_one_move_record);
    // Process wire two
    let wire_two_move_record = process_wire_nogrid(
        wire_two,
        &wire_move_records,
        &mut intersections,
        &start_loc,
    );
    wire_move_records.insert(GRID_WIRE_2, wire_two_move_record);
    // Calculate Manhattan distances for all intersection points
    let mut distances = Vec::<usize>::new();
    for intersection in intersections {
        let manhattan_dist = calc_manhattan_dist(&start_loc, &intersection);
        if manhattan_dist == 0 {
            continue;
        }
        distances.push(manhattan_dist);
    }
    // Return result
    let min_manhattan_dist = distances.iter().min().unwrap();
    return *min_manhattan_dist;
}

/// Calculates and displays the solution for Day 03 Part 2.
pub fn solution_part_2(filename: String) -> usize {
    // Open file
    let mut file = super::utils::fs::open_file(filename);
    // Read file lines
    let mut file_str = String::new();
    match file.read_to_string(&mut file_str) {
        Err(e) => panic!("Failed to read file to string. ({})", e.to_string()),
        Ok(_) => 0,
    };
    // Separate out the two wires given in input file
    let mut lines = file_str.lines();
    let wire_one: Vec<&str> = lines.next().unwrap().split(',').collect();
    let wire_two: Vec<&str> = lines.next().unwrap().split(',').collect();
    // Initialise variables required for processing wires
    let mut intersections = Vec::<Vec<usize>>::new();
    let mut wire_move_records = HashMap::<usize, HashMap<usize, usize>>::new();
    let start_loc: Vec<usize> = vec![GRID_X / 2, GRID_Y / 2];
    // Process wire one
    let wire_one_move_record = process_wire_nogrid(
        wire_one,
        &wire_move_records,
        &mut intersections,
        &start_loc,
    );
    wire_move_records.insert(GRID_WIRE_1, wire_one_move_record);
    // Process wire two
    let wire_two_move_record = process_wire_nogrid(
        wire_two,
        &wire_move_records,
        &mut intersections,
        &start_loc,
    );
    wire_move_records.insert(GRID_WIRE_2, wire_two_move_record);
    // Calculated the combined moves to each intersection to find minimum val
    let mut min_combined_moves: usize = MAX;
    for intersection in intersections {
        // Check for starting location intersection
        if intersection == start_loc {
            continue;
        }
        // Calculate key
        let key = intersection[X_VAR] * GRID_ORD_MAG + intersection[Y_VAR];
        let wire_one_moves: usize = wire_move_records[&GRID_WIRE_1][&key];
        let wire_two_moves: usize = wire_move_records[&GRID_WIRE_2][&key];
        let combined_moves = wire_one_moves + wire_two_moves;
        // Check if new minimum value has been found
        if combined_moves < min_combined_moves {
            min_combined_moves = combined_moves;
        }
    }
    // Display result
    return min_combined_moves;
}

/// Calculates the Manhattan distance between the two given points. Function
/// arguments are required to be vectors containing minimum two non-negative
/// location values.
fn calc_manhattan_dist(start_loc: &Vec<usize>, intersection: &Vec<usize>) -> usize {
    let x_diff: usize;
    if start_loc[X_VAR] > intersection[X_VAR] {
        x_diff = start_loc[X_VAR] - intersection[X_VAR];
    } else {
        x_diff = intersection[X_VAR] - start_loc[X_VAR];
    }
    let y_diff: usize;
    if start_loc[Y_VAR] > intersection[Y_VAR] {
        y_diff = start_loc[Y_VAR] - intersection[Y_VAR];
    } else {
        y_diff = intersection[Y_VAR] - start_loc[Y_VAR];
    }
    return x_diff + y_diff;
}

/// Processes the current wire move command, recording any intersections as they
/// occur. The current location and moves made so far are updated so the new values
/// can be used in processing the subsequent wire move commands.
fn wire_make_moves_nogrid(
    dir: char,
    num_moves: i32,
    moves_so_far: &mut usize,
    previous_wire_moves: &HashMap::<usize, HashMap<usize, usize>>,
    intersections: &mut Vec<Vec<usize>>,
    curr_loc: &mut Vec<usize>,
    move_record: &mut HashMap<usize, usize>,
) {
    for _ in 0..num_moves {
        // Generate key for storing wire moves
        let key = curr_loc[X_VAR] * GRID_ORD_MAG + curr_loc[Y_VAR];
        if !move_record.contains_key(&key) {
            move_record.insert(key, *moves_so_far);
        }
        // Only increment moves after current location has been recorded
        // to prevent additional move being added at end of run.
        *moves_so_far += 1;
        // Check if previous wire was in current location
        for wire_types in previous_wire_moves.keys() {
            let previous_moves = previous_wire_moves.get(&wire_types).unwrap();
            if previous_moves.contains_key(&key) {
                intersections.push(curr_loc.to_vec());
                break;
            }
        }
        // Check the current move direction and update location as required
        if dir == DIR_RIGHT {
            if curr_loc[X_VAR] == (GRID_X - 1) {
                panic!("Grid X-var upper limit reached - make the grid BIGGER!");
            }
            curr_loc[X_VAR] += 1;
        } else if dir == DIR_LEFT {
            if curr_loc[X_VAR] == 0 {
                panic!("Grid X-var lower limit reached - make the grid BIGGER!");
            }
            curr_loc[X_VAR] -= 1;
        } else if dir == DIR_UP {
            if curr_loc[Y_VAR] == 0 {
                panic!("Grid Y-var lower limit reached - make the grid BIGGER!");
            }
            curr_loc[Y_VAR] -= 1;
        } else if dir == DIR_DOWN {
            if curr_loc[Y_VAR] == (GRID_Y - 1) {
                panic!("Grid Y-var upper limit reacher - make the grid BIGGER!");
            }
            curr_loc[Y_VAR] += 1;
        } else {
            // Shouldn't get here!
            panic!("Day 03 - HERE BE DRAGONS!");
        }
    }
}

/// Processes the given wire, using the record of previous wire moves to determine
/// if any intersections occur.
fn process_wire_nogrid(
    wire: Vec<&str>,
    previous_wire_move_record: &HashMap::<usize, HashMap<usize, usize>>,
    intersections: &mut Vec<Vec<usize>>,
    start_loc: &Vec<usize>,
) -> HashMap<usize, usize> {
    let mut current_location = start_loc.to_vec();
    let mut moves_so_far = 0;
    let mut move_record = HashMap::<usize, usize>::new();
    // Process each move in the given wire
    for code in wire {
        let dir = code.chars().next().unwrap();
        let num_moves = code[1..].parse::<i32>().unwrap();
        wire_make_moves_nogrid(
            dir,
            num_moves,
            &mut moves_so_far,
            previous_wire_move_record,
            intersections,
            &mut current_location,
            &mut move_record,
        );
    }
    return move_record;
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test actual solution for Part 1 - to check if this has been broken.
    #[test]
    fn test_p1_actual_solution() {
        let result = solution_part_1(String::from("./input/day_03/input.txt"));
        assert_eq!(1211, result);
    }

    /// Test actual solution for Part 2 - to check if this has been broken.
    #[test]
    fn test_p2_actual_solution() {
        let result = solution_part_2(String::from("./input/day_03/input.txt"));
        assert_eq!(101386, result);
    }

    #[test]
    fn test_p1_example_input_1() {
        let result = solution_part_1(String::from("./input/day_03/test/test_01.txt"));
        assert_eq!(159, result);
    }

    #[test]
    fn test_p1_example_input_2() {
        let result = solution_part_1(String::from("./input/day_03/test/test_02.txt"));
        assert_eq!(135, result);
    }

    #[test]
    fn test_p2_example_input_1() {
        let result = solution_part_2(String::from("./input/day_03/test/test_01.txt"));
        assert_eq!(610, result);
    }

    #[test]
    fn test_p2_example_input_2() {
        let result = solution_part_2(String::from("./input/day_03/test/test_02.txt"));
        assert_eq!(410, result);
    }
}
