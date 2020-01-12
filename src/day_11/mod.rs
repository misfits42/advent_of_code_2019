use super::utils::fs;
use super::utils::intcode::IntcodeMachine;
use super::utils::maps::CardinalDirection;
use std::collections::VecDeque;
use std::collections::HashMap;
use euclid::*;

/// This struct is used to represent the current state of the hull-paining robot introduced in Day
/// 11.
struct HullPaintingRobot {
    computer: IntcodeMachine,
    location: Point2D<i64, UnknownUnit>,
    direction: CardinalDirection,
}

/// Enum used to represent the state of the hull squares. All locations are assumed to be
/// BlackUnpainted initially.
#[derive(PartialEq, Clone, Copy)]
enum GridPaintState {
    BlackUnpainted,
    BlackPainted,
    WhitePainted,
}

impl GridPaintState {
    pub fn get_state_from_integer(value: i64) -> Result<GridPaintState, String> {
        if value == 0 {
            return Ok(GridPaintState::BlackPainted);
        } else if value == 1 {
            return Ok(GridPaintState::WhitePainted);
        } else {
            return Err(String::from("Unknown value given for conversion to GridPaintState."));
        }
    }
}

pub fn solution_part_1(filename: String) -> i64 {
    // Load up robot initial memory
    let mut file = fs::open_file(filename);
    let initial_memory = IntcodeMachine::extract_intcode_memory_from_file(&mut file);
    // Initialise robot
    let mut robot = HullPaintingRobot {
        computer: IntcodeMachine::new(initial_memory.clone(), VecDeque::from(vec![])),
        location: Point2D::new(0, 0),
        direction: CardinalDirection::North,
    };
    // Initialise variables to track grid state
    let mut grid_state: HashMap<Point2D<i64, UnknownUnit>, GridPaintState> = HashMap::new();
    let mut coloured_at_least_once = 0;
    // Keep track of the minimum and maximum values seen in the simulated robot location
    let mut min_x = 0;
    let mut max_x = 0;
    let mut min_y = 0;
    let mut max_y = 0;
    // Now we commence painting
    loop {
        // Check colour of current grid square
        let prev_colour = match grid_state.contains_key(&robot.location) {
            true => *grid_state.get(&robot.location).unwrap(),
            false => GridPaintState::BlackUnpainted,
        };
        let robot_input: i64 = match prev_colour {
            GridPaintState::BlackUnpainted => 0,
            GridPaintState::BlackPainted => 0,
            GridPaintState::WhitePainted => 1,
        };
        // Provide appropriate input to robot and execute program
        robot.computer.add_input(robot_input);
        robot.computer.execute_program();
        // Read the two output values from robot
        let paint_colour_arg = robot.computer.get_output_and_remove();
        let rotate_arg = robot.computer.get_output_and_remove();
        if !robot.computer.is_output_empty() {
            panic!("Output should be empty but isn't!");
        }
        // Paint current grid square
        let new_colour = GridPaintState::get_state_from_integer(paint_colour_arg);
        match new_colour {
            Err(e) => panic!(e),
            Ok(val) => {
                grid_state.insert(robot.location, val);
            },
        }
        // Increment result if square was not already painted
        if prev_colour == GridPaintState::BlackUnpainted {
            coloured_at_least_once += 1;
        }
        // if robot has halted, result result
        if robot.computer.has_halted() {
            return coloured_at_least_once;
        }
        // Rotate robot by 90 degrees and move by one square
        let rotate_direction = match rotate_arg {
            0 => false,
            1 => true,
            _ => panic!("Unknown rotation direction value observed."),
        };
        robot.direction = robot.direction.get_90deg_rotated_direction(rotate_direction);
        // Check if new location is outside previous bounds, and record new value as needed
        match robot.direction {
            CardinalDirection::North => {
                robot.location.y -= 1;
                if robot.location.y < min_y {
                    min_y = robot.location.y;
                }
            },
            CardinalDirection::East => {
                robot.location.x += 1;
                if robot.location.x > max_x {
                    max_x = robot.location.x;
                }
            },
            CardinalDirection::South => {
                robot.location.y += 1;
                if robot.location.y > max_y {
                    max_y = robot.location.y;
                }
            },
            CardinalDirection::West => {
                robot.location.x -= 1;
                if robot.location.x < min_x {
                    min_x = robot.location.x;
                }
            },
        };
    }
}
