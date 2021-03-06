use super::utils::intcode::IntcodeMachine;
use super::utils::strings;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::VecDeque;

use itertools::Itertools;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    /// Calculates the next direction if a turn is made in the given direction.
    pub fn get_rotated_direction(&self, turn_direction: TurnDirection) -> Direction {
        if turn_direction == TurnDirection::Left {
            match self {
                Direction::North => Direction::West,
                Direction::South => Direction::East,
                Direction::East => Direction::North,
                Direction::West => Direction::South,
            }
        } else {
            match self {
                Direction::North => Direction::East,
                Direction::South => Direction::West,
                Direction::East => Direction::South,
                Direction::West => Direction::North,
            }
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum TurnDirection {
    Left,
    Right,
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.y < other.y {
            return Ordering::Less;
        } else if self.y == other.y {
            if self.x < other.x {
                return Ordering::Less;
            } else if self.x == other.x {
                return Ordering::Equal;
            } else {
                return Ordering::Greater;
            }
        } else {
            return Ordering::Greater;
        }
    }
}

impl Point {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x: x, y: y }
    }

    /// Calculates the four points that would surround self.
    pub fn calculate_surrounding_points(&self) -> Vec<Point> {
        let mut up = self.clone();
        up.y -= 1;
        let mut down = self.clone();
        down.y += 1;
        let mut left = self.clone();
        left.x -= 1;
        let mut right = self.clone();
        right.x += 1;
        return vec![up, down, left, right];
    }

    /// Calculates the product of the point's X and Y co-ordinates.
    pub fn calculate_alignment_parameter(&self) -> i64 {
        return self.x * self.y;
    }
}

struct AsciiMachine {
    intcode_computer: IntcodeMachine,
    intcode_computer_orig: IntcodeMachine,
    map: HashMap<Point, char>,
    robot_location: Point,
    robot_direction: Direction,
    scaffold_intersections: Vec<Point>,
    map_width: i64,
    map_height: i64,
}

impl AsciiMachine {
    /// Creates a new ASCII machine and processes the camera view to determine the scaffold map.
    pub fn new(ascii_program: Vec<i64>) -> Self {
        let mut intcode_computer = IntcodeMachine::new(ascii_program.clone(), VecDeque::new());
        let intcode_computer_orig = intcode_computer.clone();
        intcode_computer.execute_program();
        let mut scan_location = Point::new(0, 0);
        let mut scaffold_locations: Vec<Point> = vec![];
        let mut map: HashMap<Point, char> = HashMap::new();
        let mut robot_location = Point::new(-1, -1);
        let mut robot_direction = Direction::North;
        let mut map_width = 0;
        let mut map_height = 0;
        loop {
            if intcode_computer.is_output_empty() {
                break;
            }
            let output_char = (intcode_computer.get_output_and_remove() as u8) as char;
            if output_char == '\n' {
                // Line feed received
                scan_location.x = 0;
                scan_location.y += 1;
                continue;
            } else if "<>^v".contains(output_char) {
                // Observed location of robot
                if robot_location.x != -1 && robot_location.y != -1 {
                    panic!("Already have a location for the vacuum robot.");
                }
                robot_direction = match output_char {
                    '<' => Direction::West,
                    '>' => Direction::East,
                    '^' => Direction::North,
                    'v' => Direction::South,
                    _ => panic!("Shouldn't get here!"),
                };
                robot_location = scan_location;
                scaffold_locations.push(scan_location);
            } else if output_char == '#' {
                scaffold_locations.push(scan_location);
            }
            map.insert(scan_location, output_char);
            // Update scan location after recording location and character
            if scan_location.y == 0 {
                map_width = scan_location.x;
            }
            map_height = scan_location.y;
            scan_location.x += 1;
        }
        return Self {
            intcode_computer: intcode_computer,
            intcode_computer_orig: intcode_computer_orig,
            robot_location: robot_location,
            robot_direction: robot_direction,
            map: map,
            scaffold_intersections: Self::find_scaffold_intersections(scaffold_locations.clone()),
            map_width: map_width + 1, // adjust for zero-indexed map location
            map_height: map_height + 1, // adjust for zero-indexed map location
        };
    }

    /// Resets the state of the internal Intcode computer and awakens the vacuum robot.
    pub fn awaken_robot(&mut self) {
        let mut initial_memory = self.intcode_computer_orig.get_memory_dump();
        initial_memory[0] = 2;
        self.intcode_computer = IntcodeMachine::new(initial_memory, VecDeque::new());
    }

    /// Finds the scaffold intersections and records the locations within the ASCII computer.
    fn find_scaffold_intersections(scaffold_locations: Vec<Point>) -> Vec<Point> {
        let mut scaffold_intersections: Vec<Point> = vec![];
        for point in scaffold_locations.iter() {
            let surrounding_points = point.calculate_surrounding_points();
            let mut is_intersection = true;
            for surr in surrounding_points {
                if !scaffold_locations.contains(&surr) {
                    is_intersection = false;
                    break;
                }
            }
            if is_intersection {
                scaffold_intersections.push(*point);
            }
        }
        return scaffold_intersections;
    }

    pub fn calculate_alignment_parameter_sum(&self) -> i64 {
        let mut sum = 0;
        for intersection in self.scaffold_intersections.iter() {
            sum += intersection.calculate_alignment_parameter();
        }
        return sum;
    }

    pub fn render_map(&self) {
        let mut points: Vec<Point> = self.map.keys().map(|x| *x).collect();
        points.sort_by(|a, b| a.cmp(b));
        for p in points {
            if p.x == 0 && p.y > 0 {
                println!("");
            }
            let c = self.map.get(&p).unwrap();
            print!("{}", c);
        }
        println!("");
    }

    /// Finds the path required to traverse the entire scaffold, including turns required and number
    /// of steps taken after each turn.
    pub fn find_path_to_traverse_scaffold(&self, turn_and_moves_combined: bool) -> Vec<String> {
        let mut path: Vec<String> = vec![];
        let mut current_direction = self.robot_direction;
        let mut current_location = self.robot_location;
        let mut current_move_count = 0;
        let mut holder = String::from("");
        loop {
            if !self.check_target_square_for_scaffold(current_direction, current_location) {
                if current_move_count > 0 {
                    if !turn_and_moves_combined {
                        path.push(current_move_count.to_string());
                    } else {
                        holder = format!("{}{}", holder.clone(), current_move_count.to_string());
                        path.push(holder.clone());
                        holder.clear();
                    }
                    current_move_count = 0;
                }
                // Check left turn
                let temp_left = current_direction.get_rotated_direction(TurnDirection::Left);
                let temp_right = current_direction.get_rotated_direction(TurnDirection::Right);
                if self.check_target_square_for_scaffold(temp_left, current_location) {
                    current_direction = temp_left;
                    current_direction.get_rotated_direction(TurnDirection::Left);
                    if !turn_and_moves_combined {
                        path.push(String::from("L"));
                    } else {
                        holder.push_str("L");
                    }
                } else if self.check_target_square_for_scaffold(temp_right, current_location) {
                    current_direction = temp_right;
                    current_direction.get_rotated_direction(TurnDirection::Right);
                    if !turn_and_moves_combined {
                        path.push(String::from("R"));
                    } else {
                        holder.push_str("R");
                    }
                } else {
                    // No more turns possible - we have traversed all the scaffold.
                    return path;
                }
            } else {
                current_move_count += 1;
                match current_direction {
                    Direction::North => current_location.y -= 1,
                    Direction::South => current_location.y += 1,
                    Direction::East => current_location.x += 1,
                    Direction::West => current_location.x -= 1,
                };
            }
        }
    }

    /// Gets the movement command strings that need to be provided to the robot in order to traverse
    /// all scaffold locations at least once.
    ///
    /// Order of movement commands:
    /// - Main routine
    /// - Subroutine A
    /// - Subroutine B
    /// - Subroutine C
    /// - Camera enabled (y/n)
    pub fn get_movement_commands(&self) -> Vec<String> {
        let mut commands: Vec<String> = vec![];
        let path = self.find_path_to_traverse_scaffold(true);
        for a_end in 0..path.len() - 2 {
            let b_start = a_end + 1;
            for b_end in b_start..path.len() - 1 {
                let mut new_path = path.clone().join("");
                // Remove all of the potential A move blocks
                let mut a_command = String::new();
                for i in 0..a_end + 1 {
                    a_command.push_str(&path[i]);
                }
                new_path = new_path.replace(&a_command, "");
                // Remove all of potential B move blocks
                let mut b_command = String::new();
                for i in b_start..b_end + 1 {
                    b_command.push_str(&path[i]);
                }
                new_path = new_path.replace(&b_command, "");
                // Check if new_path now contains only repeated substring - C command is root string
                let mut c_command = strings::find_repeat_substring(new_path, 9);
                // Process commands
                if !c_command.is_empty() {
                    // Generate the main routine first
                    let mut main_routine = path.clone().join("");
                    main_routine = main_routine.replace(&a_command, "A");
                    main_routine = main_routine.replace(&b_command, "B");
                    main_routine = main_routine.replace(&c_command, "C");
                    main_routine = main_routine.chars().join(",");
                    main_routine.push('\n');
                    if main_routine.len() > 21 {
                        panic!("Bad main routine length: {}", main_routine.len());
                    }
                    commands.push(main_routine);
                    // Generate A sub-routine
                    a_command = Self::format_subroutine_string(a_command);
                    if a_command.len() > 21 {
                        panic!("Bad A-subroutine length: {}", a_command.len());
                    }
                    commands.push(a_command);
                    // Generate B sub-routine
                    b_command = Self::format_subroutine_string(b_command);
                    if b_command.len() > 21 {
                        panic!("Bad B-subroutine length: {}", b_command.len());
                    }
                    commands.push(b_command);
                    // Generate C sub-routine
                    c_command = Self::format_subroutine_string(c_command);
                    if c_command.len() > 21 {
                        panic!("Bad C-subroutine length: {}", c_command.len());
                    }
                    commands.push(c_command);
                    // Camera feed state - not enabled
                    commands.push("n\n".to_owned());
                    return commands;
                }
            }
        }
        return vec![];
    }

    /// Takes the input movement subroutine string and inserts comma characters between turn and
    /// move orders.
    fn format_subroutine_string(input: String) -> String {
        let mut output = input.clone();
        output = output.replace("L", ",L,");
        output = output.replace("R", ",R,");
        output.push('\n');
        return output[1..].to_owned();
    }

    /// Checks if the next square in the given direction contains scaffold or not.
    pub fn check_target_square_for_scaffold(
        &self,
        current_direction: Direction,
        current_location: Point,
    ) -> bool {
        let mut target_square = current_location;
        match current_direction {
            Direction::North => {
                target_square.y -= 1;
            }
            Direction::South => {
                target_square.y += 1;
            }
            Direction::East => {
                target_square.x += 1;
            }
            Direction::West => {
                target_square.x -= 1;
            }
        }
        if target_square.x < 0
            || target_square.x >= self.map_width
            || target_square.y < 0
            || target_square.y >= self.map_height
        {
            return false;
        }
        let target_char = match self.map.get(&target_square) {
            Some(v) => *v,
            None => panic!(
                "Bad target square: [{:?}]. Map width is {}. Map height is {}.",
                target_square, self.map_width, self.map_height
            ),
        };
        // let target_char = *self.map.get(&target_square).unwrap();
        return target_char == '#';
    }
}

/// Solution for Day 17 Part 1 challenge.
pub fn solution_part_1(filename: String) -> i64 {
    let ascii_program = IntcodeMachine::extract_intcode_memory_from_filename(filename);
    let ascii_machine = AsciiMachine::new(ascii_program);
    ascii_machine.render_map();
    let align_param_sum = ascii_machine.calculate_alignment_parameter_sum();
    return align_param_sum;
}

/// Solution for Day 17 Part 2 challenge.
pub fn solution_part_2(filename: String) -> i64 {
    // Load up the ascii program to get camera view of scaffold
    let ascii_program = IntcodeMachine::extract_intcode_memory_from_filename(filename);
    let mut ascii_machine = AsciiMachine::new(ascii_program);
    // Awaken robot and reset memory of intcode computer
    ascii_machine.awaken_robot();
    // Get the commands required for robot to traverse all scaffold locations at least once.
    let commands = ascii_machine.get_movement_commands();
    // Provide movement commands as ASCII characters to robot
    for item in commands {
        for c in item.chars() {
            ascii_machine.intcode_computer.add_input(c as i64);
        }
    }
    // Set robot off on crawl to meet the other robots left outside.
    ascii_machine.intcode_computer.execute_program();
    // Consume all outputs within ASCII character range of values until dust collected output
    loop {
        let output = ascii_machine.intcode_computer.get_output_and_remove();
        if output <= 128 { // Value is within range of ASCII character values
            continue;
        }
        return output; // Output here is amount of dust collected by vacuum robot.
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_17_p1_solution() {
        let result = solution_part_1(String::from("./input/day_17/input.txt"));
        assert_eq!(3936, result);
    }

    #[test]
    fn test_day_17_p2_solution() {
        let result = solution_part_2(String::from("./input/day_17/input.txt"));
        assert_eq!(785733, result);
    }
}
