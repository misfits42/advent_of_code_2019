use super::utils::intcode::IntcodeMachine;
use std::collections::VecDeque;
use std::collections::HashSet;
use std::collections::HashMap;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    pub fn new(x: i64, y: i64) -> Self {
        Self {
            x: x,
            y: y,
        }
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

    pub fn calculate_alignment_parameter(&self) -> i64 {
        return self.x * self.y;
    }
}

#[allow(dead_code)]
struct AsciiMachine {
    intcode_computer: IntcodeMachine,
    scaffold_locations: HashSet<Point>,
    map: HashMap<Point, char>,
    robot_location: Point,
    scaffold_intersections: HashSet<Point>,
}

impl AsciiMachine {
    pub fn new(ascii_program: Vec<i64>) -> Self {
        let mut intcode_computer = IntcodeMachine::new(ascii_program.clone(), VecDeque::new());
        intcode_computer.execute_program();
        let mut scan_location = Point::new(0, 0);
        let mut scaffold_locations: HashSet<Point> = HashSet::new();
        let mut map: HashMap<Point, char> = HashMap::new();
        let mut robot_location = Point::new(-1, -1);
        loop {
            if intcode_computer.is_output_empty() {
                break;
            }
            let output_char = (intcode_computer.get_output_and_remove() as u8) as char;
            if output_char == '\n' { // Line feed received
                scan_location.x = 0;
                scan_location.y += 1;
                continue;
            } else if "<>^v".contains(output_char) { // Observed location of robot
                if robot_location.x != -1 && robot_location.y != -1 {
                    panic!("Already have a location for the vacuum robot.");
                }
                robot_location = scan_location;
                scaffold_locations.insert(scan_location);
            } else if output_char == '#' {
                scaffold_locations.insert(scan_location);
            }
            map.insert(scan_location, output_char);
            // Update scan location after recording location and character
            scan_location.x += 1;
        }
        return Self {
            intcode_computer: intcode_computer,
            robot_location: robot_location,
            map: map,
            scaffold_locations: scaffold_locations,
            scaffold_intersections: HashSet::new(),
        };
    }

    /// Finds the scaffold intersections and records the locations within the ASCII computer.
    pub fn find_scaffold_intersections(&mut self) {
        for point in self.scaffold_locations.iter() {
            let surrounding_points = point.calculate_surrounding_points();
            let mut is_intersection = true;
            for surr in surrounding_points {
                if !self.scaffold_locations.contains(&surr) {
                    is_intersection = false;
                }
            }
            if is_intersection {
                self.scaffold_intersections.insert(*point);
            }
        }
    }

    pub fn calculate_alignment_parameter_sum(&self) -> i64 {
        let mut sum = 0;
        for intersection in self.scaffold_intersections.iter() {
            sum += intersection.calculate_alignment_parameter();
        }
        return sum;
    }
}

pub fn solution_part_1(filename: String) -> i64 {
    let ascii_program = IntcodeMachine::extract_intcode_memory_from_filename(filename);
    let mut ascii_machine = AsciiMachine::new(ascii_program);
    ascii_machine.find_scaffold_intersections();
    let align_param_sum = ascii_machine.calculate_alignment_parameter_sum();
    return align_param_sum;
}