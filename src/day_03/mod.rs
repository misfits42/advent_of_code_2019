use std::io::Read;
use std::error::Error;
use ndarray::Array2;

// Config parameters
const GRID_X: usize = 20000;
const GRID_Y: usize = 20000;
const X_VAR: usize = 0;
const Y_VAR: usize = 1;

// Directions
const DIR_RIGHT: char = 'R';
const DIR_LEFT: char = 'L';
const DIR_UP: char = 'U';
const DIR_DOWN: char = 'D';

// Grid values
const GRID_BLANK: usize = 0;
const GRID_WIRE_1: usize = 1;
const GRID_WIRE_2: usize = 2;
const GRID_INTERSECTION: usize = 3;

/// Calculates and displays the solution for Day 03 Part 1 challenge.
pub fn day_03_solution_p1(filename: String) {
    // Open file
    let mut file = super::utils::fs::open_file(filename);
    // Read file lines
    let mut file_str = String::new();
    match file.read_to_string(&mut file_str) {
        Err(e) => panic!("Failed to read file to string. ({})", e.description()),
        Ok(_) => 0,
    };
    // Separate out the two wires given in input file
    let mut lines = file_str.lines();
    let wire_one: Vec<&str> = lines.next().unwrap().split(',').collect();
    let wire_two: Vec<&str> = lines.next().unwrap().split(',').collect();
    // Create blank grid and initialise starting location
    let mut grid = Array2::<usize>::zeros((GRID_X, GRID_Y));
    let mut intersections = Vec::<Vec<usize>>::new();
    // Process wires
    let start_loc: Vec<usize> = vec![GRID_X / 2, GRID_Y / 2];
    process_wire(wire_one, GRID_WIRE_1, &mut grid, &mut intersections, &start_loc);
    process_wire(wire_two, GRID_WIRE_2, &mut grid, &mut intersections, &start_loc);
    // Calculate Manhattan distances for all intersection points
    let mut distances = Vec::<usize>::new();
    for intersection in intersections {
        let manhattan_dist = calc_manhattan_dist(&start_loc, &intersection);
        if manhattan_dist == 0 {
            continue;
        }
        distances.push(manhattan_dist);
    }
    // Display result
    let min_manhattan_dist = distances.iter().min().unwrap();
    println!("Day 03 Part 1 solution is: {}", min_manhattan_dist);
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

/// Makes the required number of moves in the specified direction. Checks if
/// squares are still blank or already occupied (in which case an intersection
/// is recorded).
fn grid_make_moves(dir: char, num_moves: i32, grid: &mut Array2<usize>,
        intersections: &mut Vec<Vec<usize>>, curr_loc: &mut Vec<usize>,
        wire_type: usize) {
    for _ in 0..num_moves {
        // Check if square is blank
        let location = (curr_loc[X_VAR], curr_loc[Y_VAR]);
        if grid[location] == GRID_BLANK {
            grid[location] = wire_type;
        // Square already occupied - record an intersection
        } else if grid[location] != wire_type {
            grid[location] = GRID_INTERSECTION;
            intersections.push(curr_loc.to_vec());
        }
        // Check the current move direction and update location as required
        if dir == DIR_RIGHT {
            curr_loc[X_VAR] += 1;
        } else if dir == DIR_LEFT {
            curr_loc[X_VAR] -= 1;
        } else if dir == DIR_UP {
            curr_loc[Y_VAR] -= 1;
        } else if dir == DIR_DOWN {
            curr_loc[Y_VAR] += 1;
        } else { // Shouldn't get here!
            panic!("Day 03 - HERE BE DRAGONS!");
        }
    }
}

/// Takes the move instructions detailing the wire and runs the wire through the
/// given grid. If the wire comes across a square with a wire type already in it
/// not matching the current wire type, an intersection of wires is recorded.
fn process_wire(wire: Vec<&str>, wire_type: usize, grid: &mut Array2<usize>,
        intersections: &mut Vec<Vec<usize>>, start_loc: &Vec<usize>) {
    let mut current_location = start_loc.to_vec();
    for code in wire {
        let dir = code.chars().next().unwrap();
        let num_moves = code[1..].parse::<i32>().unwrap();
        grid_make_moves(dir, num_moves, grid, intersections,
            &mut current_location, wire_type);
    }
}