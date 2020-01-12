use super::utils::fs;
use super::utils::intcode::IntcodeMachine;
use super::utils::maps::CardinalDirection;
use euclid::*;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

/// This struct is used to represent the current state of the hull-paining robot introduced in Day
/// 11.
struct HullPaintingRobot {
    computer: IntcodeMachine,
    location: Point2D<i32, UnknownUnit>,
    direction: CardinalDirection,
}

#[allow(dead_code)]
struct GridDimensions {
    width: u32,
    height: u32,
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
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
    /// Gets the corresponding paint colour for the given value.
    pub fn get_state_from_integer(value: i32) -> Result<GridPaintState, String> {
        if value == 0 {
            return Ok(GridPaintState::BlackPainted);
        } else if value == 1 {
            return Ok(GridPaintState::WhitePainted);
        } else {
            return Err(String::from(
                "Unknown value given for conversion to GridPaintState.",
            ));
        }
    }

    /// Converts the paint colour into the corresponding RGBA values.
    pub fn get_rgba_from_paint_colour(&self) -> Vec<u8> {
        match self {
            GridPaintState::BlackUnpainted => vec![0, 0, 0, 255],
            GridPaintState::BlackPainted => vec![0, 0, 0, 255],
            GridPaintState::WhitePainted => vec![255, 255, 255, 255],
        }
    }
}

/// Calculates the solution for Day 11 Part 1 challenge.
pub fn solution_part_1(filename: String) -> i32 {
    // Load up robot initial memory
    let mut file = fs::open_file(filename);
    let initial_memory = IntcodeMachine::extract_intcode_memory_from_file(&mut file);
    // Let's paint the hull
    let result = paint_hull(initial_memory, HashMap::new());
    return result.0;
}

/// Calculates the solution for Day 11 Part 2 challenge.
pub fn solution_part_2(filename: String) {
    // Load up robot initial memory
    let mut file = fs::open_file(filename);
    let initial_memory = IntcodeMachine::extract_intcode_memory_from_file(&mut file);
    // Let's paint the hull
    let mut initial_grid_state: HashMap<Point2D<i32, UnknownUnit>, GridPaintState> = HashMap::new();
    initial_grid_state.insert(Point2D::new(0, 0), GridPaintState::WhitePainted);
    let result = paint_hull(initial_memory, initial_grid_state);
    // Extract data from the result tuple
    let grid_dimensions: GridDimensions = result.1;
    let grid_state: HashMap<Point2D<i32, UnknownUnit>, GridPaintState> = result.2;
    // Reconstruct the painted state
    let mut image_data: Vec<u8> = vec![];
    for y_loc in 0..grid_dimensions.height {
        for x_loc in 0..grid_dimensions.width {
            // Calculate the adjusted location as it would have been seen by robot
            let x_adj = x_loc as i32 + grid_dimensions.x_min;
            let y_adj = y_loc as i32 + grid_dimensions.y_min;
            let adjusted_location: Point2D<i32, UnknownUnit> = Point2D::new(x_adj, y_adj);
            // Get the square colour to reconstruct the end state
            let mut rgba_values = match grid_state.get(&adjusted_location) {
                None => GridPaintState::BlackUnpainted.get_rgba_from_paint_colour(),
                Some(colour) => colour.get_rgba_from_paint_colour(),
            };
            image_data.append(&mut rgba_values);
        }
    }
    // Write the data to a PNG image
    let path = Path::new(r"./aoc-2019-day-11-p2.png");
    let file = File::create(path).unwrap();
    let w = BufWriter::new(file);
    let mut encoder = png::Encoder::new(w, grid_dimensions.width, grid_dimensions.height);
    encoder.set_color(png::ColorType::RGBA);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&image_data).unwrap();
}

/// Calculates the width and height of a grid from the minimum and maximum observed x- and y-
/// co-ordinates.
fn calculate_grid_dimensions(min_x: i32, max_x: i32, min_y: i32, max_y: i32) -> GridDimensions {
    return GridDimensions {
        width: (min_x.abs() + max_x + 1) as u32,
        height: (min_y.abs() + max_y + 1) as u32,
        x_min: min_x,
        x_max: max_x,
        y_min: min_y,
        y_max: max_y,
    };
}

/// Paints the hull using a robot containing an IntcodeMachine loaded with the given initial memory.
/// The paint state of the hull is initialised using the given values.GridPaintState
///
/// The return data is a tuple consisting of: (index 0) number of squares painted at least once,
/// (index 1) width and height dimenions of the resulting painted area, and (index 2) the paint
/// state of the hull.
fn paint_hull(
    initial_memory: Vec<i64>,
    initial_grid_state: HashMap<Point2D<i32, UnknownUnit>, GridPaintState>,
) -> (
    i32,
    GridDimensions,
    HashMap<Point2D<i32, UnknownUnit>, GridPaintState>,
) {
    // Initialise robot
    let mut robot = HullPaintingRobot {
        computer: IntcodeMachine::new(initial_memory.clone(), VecDeque::from(vec![])),
        location: Point2D::new(0, 0),
        direction: CardinalDirection::North,
    };
    // Initialise variables to track grid state
    let mut grid_state: HashMap<Point2D<i32, UnknownUnit>, GridPaintState> = HashMap::new();
    for (key, value) in initial_grid_state.into_iter() {
        grid_state.insert(key, value);
    }
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
        let paint_colour_arg = robot.computer.get_output_and_remove() as i32;
        let rotate_arg = robot.computer.get_output_and_remove() as i32;
        if !robot.computer.is_output_empty() {
            panic!("Output should be empty but isn't!");
        }
        // Paint current grid square
        let new_colour = GridPaintState::get_state_from_integer(paint_colour_arg);
        match new_colour {
            Err(e) => panic!(e),
            Ok(val) => {
                grid_state.insert(robot.location, val);
            }
        }
        // Increment result if square was not already painted
        if prev_colour == GridPaintState::BlackUnpainted {
            coloured_at_least_once += 1;
        }
        // if robot has halted, result result
        if robot.computer.has_halted() {
            let grid_dimensions = calculate_grid_dimensions(min_x, max_x, min_y, max_y);
            return (coloured_at_least_once, grid_dimensions, grid_state);
        }
        // Rotate robot by 90 degrees and move by one square
        let rotate_direction = match rotate_arg {
            0 => false,
            1 => true,
            _ => panic!("Unknown rotation direction value observed."),
        };
        robot.direction = robot
            .direction
            .get_90deg_rotated_direction(rotate_direction);
        // Check if new location is outside previous bounds, and record new value as needed
        match robot.direction {
            CardinalDirection::North => {
                robot.location.y -= 1;
                if robot.location.y < min_y {
                    min_y = robot.location.y;
                }
            }
            CardinalDirection::East => {
                robot.location.x += 1;
                if robot.location.x > max_x {
                    max_x = robot.location.x;
                }
            }
            CardinalDirection::South => {
                robot.location.y += 1;
                if robot.location.y > max_y {
                    max_y = robot.location.y;
                }
            }
            CardinalDirection::West => {
                robot.location.x -= 1;
                if robot.location.x < min_x {
                    min_x = robot.location.x;
                }
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1_actual_solution() {
        let result = solution_part_1(String::from("./input/day_11/input.txt"));
        assert_eq!(1686, result);
    }
}
