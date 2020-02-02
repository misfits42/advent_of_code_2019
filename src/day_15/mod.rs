use super::utils::intcode::IntcodeMachine;
use std::collections::VecDeque;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
enum MoveDirection {
    North,
    South,
    West,
    East,
}

impl MoveDirection {
    /// Gets integer representation of direction.
    pub fn get_code(&self) -> i64 {
        match self {
            MoveDirection::North => 1,
            MoveDirection::South => 2,
            MoveDirection::West => 3,
            MoveDirection::East => 4,
        }
    }

    /// Gets the next direction in clockwise rotation.
    pub fn get_rotated_direction(&self) -> Self {
        match self {
            MoveDirection::North => MoveDirection::East,
            MoveDirection::East => MoveDirection::South,
            MoveDirection::South => MoveDirection::West,
            MoveDirection::West => MoveDirection::North,
        }
    }

    /// Gets the opposite direction (180 degree opposite).
    pub fn get_opposite_direction(&self) -> Self {
        match self {
            MoveDirection::North => MoveDirection::South,
            MoveDirection::South => MoveDirection::North,
            MoveDirection::East => MoveDirection::West,
            MoveDirection::West => MoveDirection::East,
        }
    }
}

// Status codes
const STATUS_HIT_WALL: i64 = 0;
const STATUS_GOOD_MOVE: i64 = 1;
const STATUS_GOOD_MOVE_OXYGEN: i64 = 2;
// Maze states
const STATE_WALL: i64 = 0;
const STATE_CLEAR: i64 = 1;

// #[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
// enum MazeState {
//     Wall,
//     Clear,
// }

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Location {
    x: i64,
    y: i64,
}

impl Location {
    pub fn get_updated_location(&self, direction: MoveDirection) -> Self {
        let mut updated = self.clone();
        match direction {
            MoveDirection::North => updated.y -= 1,
            MoveDirection::East => updated.x += 1,
            MoveDirection::South => updated.y += 1,
            MoveDirection::West => updated.x -= 1,
        }
        return updated;
    }
}

pub fn solution_part_1(filename: String) -> u64 {
    let initial_memory: Vec<i64> = IntcodeMachine::extract_intcode_memory_from_filename(filename);
    let mut repair_droid = IntcodeMachine::new(initial_memory.clone(), VecDeque::from(vec![]));
    let moves_taken = crawl_maze(&mut repair_droid);
    return moves_taken;
}

fn rewind_move(breadcrumbs: &mut Vec<MoveDirection>, current_location: &Location) -> Location {
    if breadcrumbs.is_empty() {
        panic!("No breadcrumbs - cannot rewind moves.");
    }
    let back_direction = breadcrumbs.pop().unwrap().get_opposite_direction();
    return current_location.get_updated_location(back_direction);
}

fn crawl_maze(repair_droid: &mut IntcodeMachine) -> u64 {
    let mut breadcrumbs: Vec<MoveDirection> = vec![];
    let mut current_direction = MoveDirection::North;
    let mut maze_map: HashMap<Location, i64> = HashMap::new();
    let mut current_location = Location { x: 0, y: 0 };
    maze_map.insert(current_location, STATE_CLEAR);
    let mut moves_attempted_from_current = 0;
    loop {
        println!("Up to {} moves...", breadcrumbs.len());
        if moves_attempted_from_current == 4 {
            moves_attempted_from_current = 0;
            current_location = rewind_move(&mut breadcrumbs, &current_location);
            current_direction = MoveDirection::North;
            continue;
        }
        // Check if already explored or blocked
        let target_location = current_location.get_updated_location(current_direction);
        if let Some(_) = maze_map.get(&target_location) {
            moves_attempted_from_current += 1;
            current_direction = current_direction.get_rotated_direction();
            continue;
        }
        // Give move command to droid
        repair_droid.add_input(current_direction.get_code());
        // Instruct droid to execute move
        repair_droid.execute_program_break_on_output(true);
        // Get outcome status from move
        let status = repair_droid.get_output_and_remove();
        match status {
            STATUS_HIT_WALL => {
                moves_attempted_from_current += 1;
                maze_map.insert(target_location, STATE_WALL);
                // Rotate direction but don't update current location
                current_direction = current_direction.get_rotated_direction();
            },
            STATUS_GOOD_MOVE => {
                moves_attempted_from_current = 1;
                breadcrumbs.push(current_direction.clone());
                maze_map.insert(target_location, STATE_CLEAR);
                // Update current location and maintain current direction
                current_location = target_location;
            },
            STATUS_GOOD_MOVE_OXYGEN => {
                breadcrumbs.push(current_direction.clone());
                return breadcrumbs.len() as u64;
            },
            _ => panic!("Bad move status observed: {}", status),
        }
    }
}
