use super::utils::intcode::IntcodeMachine;
use std::collections::VecDeque;
use std::collections::HashMap;

// Status codes
const STATUS_HIT_WALL: i64 = 0;
const STATUS_GOOD_MOVE: i64 = 1;
const STATUS_GOOD_MOVE_OXYGEN: i64 = 2;
// Maze states
const STATE_WALL: i64 = 0;
const STATE_CLEAR: i64 = 1;
const STATE_GOAL: i64 = 2;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MoveDirection {
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
    pub fn get_rotated_direction(&self) -> MoveDirection {
        match self {
            MoveDirection::North => MoveDirection::East,
            MoveDirection::East => MoveDirection::South,
            MoveDirection::South => MoveDirection::West,
            MoveDirection::West => MoveDirection::North,
        }
    }

    /// Gets the opposite direction (180 degree opposite).
    pub fn get_opposite_direction(&self) -> MoveDirection {
        match self {
            MoveDirection::North => MoveDirection::South,
            MoveDirection::South => MoveDirection::North,
            MoveDirection::East => MoveDirection::West,
            MoveDirection::West => MoveDirection::East,
        }
    }
}

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

struct RepairDroid {
    computer: IntcodeMachine,
    breadcrumbs: Vec<MoveDirection>,
    known_tiles: HashMap<Location, i64>,
    current_location: Location,
    current_direction: MoveDirection,
}

impl RepairDroid {
    pub fn new(initial_memory: Vec<i64>) -> Self {
        let mut init = Self {
            computer: IntcodeMachine::new(initial_memory.clone(), VecDeque::from(vec![])),
            breadcrumbs: vec![],
            known_tiles: HashMap::new(),
            current_location: Location { x: 0, y: 0 },
            current_direction: MoveDirection::North,
        };
        init.known_tiles.insert(init.current_location, STATE_CLEAR);
        return init;
    }

    pub fn rewind_move(&mut self) {
        if self.breadcrumbs.is_empty() {
            panic!("No breadcrumbs - cannot rewind moves.");
        }
        let back_direction = self.breadcrumbs.pop().unwrap().get_opposite_direction();
        // Update manual tracking of location
        self.current_location = self.current_location.get_updated_location(back_direction);
        // Update location in repair droid intcode computer
        self.current_direction = back_direction;
        self.try_move();
    }

    pub fn get_target_location(&self) -> Location {
        return self.current_location.get_updated_location(self.current_direction);
    }

    pub fn is_target_location_explored(&self) -> bool {
        if let Some(_) = self.known_tiles.get(&self.get_target_location()) {
            return true;
        }
        return false;
    }

    pub fn rotate_direction(&mut self) {
        self.current_direction = self.current_direction.get_rotated_direction();
    }

    pub fn try_move(&mut self) -> i64 {
        self.computer.add_input(self.current_direction.get_code());
        self.computer.execute_program_break_on_output(true);
        return self.computer.get_output_and_remove();
    }

    pub fn add_target_to_known_tiles(&mut self, code: i64) {
        self.known_tiles.insert(self.get_target_location(), code);
    }

    pub fn handle_successful_move(&mut self, oxygen_found: bool) {
        if oxygen_found {
            self.add_target_to_known_tiles(STATE_GOAL);
        } else {
            self.add_target_to_known_tiles(STATE_CLEAR);
        }
        self.current_location = self.get_target_location();
        self.breadcrumbs.push(self.current_direction);
    }

    pub fn handle_bad_move(&mut self) {
        self.add_target_to_known_tiles(STATE_WALL);
        self.rotate_direction();
    }

    pub fn get_num_moves_from_origin(&self) -> u64 {
        return self.breadcrumbs.len() as u64;
    }
}

pub fn solution_part_1(filename: String) -> u64 {
    let initial_memory: Vec<i64> = IntcodeMachine::extract_intcode_memory_from_filename(filename);
    let mut repair_droid = RepairDroid::new(initial_memory.clone());
    let moves_taken = crawl_maze(&mut repair_droid);
    return moves_taken;
}

fn crawl_maze(repair_droid: &mut RepairDroid,) -> u64 {
    let mut moves_attempted_from_current = 0;
    loop {
        if moves_attempted_from_current == 4 {
            moves_attempted_from_current = 0;
            repair_droid.rewind_move();
            // println!("XXX Rewound move to: {:?}", repair_droid.current_location);
            continue;
        }
        // Check if already explored or blocked
        if repair_droid.is_target_location_explored() {
            moves_attempted_from_current += 1;
            repair_droid.rotate_direction();
            continue;
        }
        // Get outcome status from move
        let status = repair_droid.try_move();
        match status {
            STATUS_HIT_WALL => {
                moves_attempted_from_current += 1;
                repair_droid.handle_bad_move();
                // println!("XXX Bad move attempted to: {:?}", repair_droid.current_direction);
            },
            STATUS_GOOD_MOVE => {
                moves_attempted_from_current = 0;
                repair_droid.handle_successful_move(false);
                // println!("$$$ Successful move to: {:?} [{:?}]", repair_droid.current_direction, repair_droid.current_location);
            },
            STATUS_GOOD_MOVE_OXYGEN => {
                repair_droid.handle_successful_move(true);
                // println!("$$$ Successful move to: {:?} [{:?}]", repair_droid.current_direction, repair_droid.current_location);
                return repair_droid.get_num_moves_from_origin();
            },
            _ => panic!("Bad move status observed: {}", status),
        }
    }
}
