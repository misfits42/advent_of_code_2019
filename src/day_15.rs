use super::utils::intcode::IntcodeMachine;
use std::collections::HashMap;
use std::collections::VecDeque;

// Status codes
const STATUS_HIT_WALL: i64 = 0;
const STATUS_GOOD_MOVE: i64 = 1;
const STATUS_GOOD_MOVE_OXYGEN: i64 = 2;

#[derive(Debug, Copy, Clone, Hash, PartialEq)]
enum TileState {
    Wall,
    Clear,
    Goal, // Oxygen tank
}

#[derive(Debug, Copy, Clone, PartialEq)]
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

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Location {
    x: i64,
    y: i64,
}

impl Location {
    /// Generates the new location by moving one unit in the specified direction.
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

/// Represents the repair droid introduced in Day 15. Has an internal Intcode computer. Keeps track
/// of directions moved from starting point, known tile states, current location and current
/// direction.
struct RepairDroid {
    computer: IntcodeMachine,
    breadcrumbs: Vec<MoveDirection>,
    known_tiles: HashMap<Location, TileState>,
    current_location: Location,
    current_direction: MoveDirection,
}

impl RepairDroid {
    /// Creates a new RepairDroid with the given initial memory for its Intcode computer. Starts at
    /// location {x: 0, y: 0} and facing North. Its starting location is added to the known tiles
    /// as a known CLEAR tile.
    pub fn new(initial_memory: Vec<i64>) -> Self {
        let mut init = Self {
            computer: IntcodeMachine::new(initial_memory.clone(), VecDeque::from(vec![])),
            breadcrumbs: vec![],
            known_tiles: HashMap::new(),
            current_location: Location { x: 0, y: 0 },
            current_direction: MoveDirection::North,
        };
        init.known_tiles.insert(init.current_location, TileState::Clear);
        return init;
    }

    /// Reverses the last successful move by changing the manual tracking location and processing
    /// the reverse move through the internal Intcode computer.
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

    /// Gets the next location in the current direction.
    pub fn get_target_location(&self) -> Location {
        return self
            .current_location
            .get_updated_location(self.current_direction);
    }

    /// Checks if the targeted location of the RepairDroid has been explored yet.
    pub fn is_target_location_explored(&self) -> bool {
        if let Some(_) = self.known_tiles.get(&self.get_target_location()) {
            return true;
        }
        return false;
    }

    /// Rotates the direction of the RepairDroid by 90 degrees clockwise.
    pub fn rotate_direction(&mut self) {
        self.current_direction = self.current_direction.get_rotated_direction();
    }

    /// Trys to process a move through the internal Intcode computer and returns the status code
    /// indicating success or failure.
    pub fn try_move(&mut self) -> i64 {
        self.computer.add_input(self.current_direction.get_code());
        self.computer.execute_program_break_on_output(true);
        return self.computer.get_output_and_remove();
    }

    /// Adds the target location to the known tiles with the given tile code.
    pub fn add_target_to_known_tiles(&mut self, tile_state: TileState) {
        self.known_tiles.insert(self.get_target_location(), tile_state);
    }

    /// Handles a successful move by updating the internal state of the RepairDroid.
    pub fn handle_successful_move(&mut self, oxygen_found: bool) {
        if oxygen_found {
            self.add_target_to_known_tiles(TileState::Goal);
        } else {
            self.add_target_to_known_tiles(TileState::Clear);
        }
        self.current_location = self.get_target_location();
        self.breadcrumbs.push(self.current_direction);
    }

    /// Handles a bad move by inserting a wall at the targeted location and rotating the droid.
    pub fn handle_bad_move(&mut self) {
        self.add_target_to_known_tiles(TileState::Wall);
        self.rotate_direction();
    }

    /// Gets the number of moves made by the droid from its starting location.
    pub fn get_num_moves_from_origin(&self) -> u64 {
        return self.breadcrumbs.len() as u64;
    }

    /// Resets the RepairDroid's knowledge of the map to only it's current location.
    pub fn reset_to_current_location(&mut self) {
        // Reset knowledge of how repair droid got to current location
        self.breadcrumbs.clear();
        // Clear the known tiles and reinsert the current location as the only known tile
        let current_location_state = *self.known_tiles.get(&self.current_location).unwrap();
        self.known_tiles.clear();
        self.known_tiles
            .insert(self.current_location.clone(), current_location_state);
        self.current_direction = MoveDirection::North;
    }

    /// Checks if a given location has been explored by the RepairDroid.
    pub fn check_if_location_explored(&self, location: Location) -> bool {
        if let Some(_) = self.known_tiles.get(&location) {
            return true;
        }
        return false;
    }

    /// Checks if all locations around the RepairDroid's current location has been explored.
    pub fn check_all_around_explored(&self) -> bool {
        let north_target = self
            .current_location
            .get_updated_location(MoveDirection::North);
        let east_target = self
            .current_location
            .get_updated_location(MoveDirection::East);
        let south_target = self
            .current_location
            .get_updated_location(MoveDirection::South);
        let west_target = self
            .current_location
            .get_updated_location(MoveDirection::West);
        return self.check_if_location_explored(north_target)
            && self.check_if_location_explored(east_target)
            && self.check_if_location_explored(south_target)
            && self.check_if_location_explored(west_target);
    }

    /// Crawls the repair droid through the map until it finds the oxygen tank.
    pub fn crawl_map_to_oxygen(&mut self) -> u64 {
        let mut moves_attempted_from_current = 0;
        loop {
            if moves_attempted_from_current == 4 {
                moves_attempted_from_current = 0;
                self.rewind_move();
                continue;
            }
            // Check if already explored or blocked
            if self.is_target_location_explored() {
                moves_attempted_from_current += 1;
                self.rotate_direction();
                continue;
            }
            // Get outcome status from move
            let status = self.try_move();
            match status {
                STATUS_HIT_WALL => {
                    moves_attempted_from_current += 1;
                    self.handle_bad_move();
                }
                STATUS_GOOD_MOVE => {
                    moves_attempted_from_current = 0;
                    self.handle_successful_move(false);
                }
                STATUS_GOOD_MOVE_OXYGEN => {
                    self.handle_successful_move(true);
                    return self.get_num_moves_from_origin();
                }
                _ => panic!("Bad move status observed: {}", status),
            }
        }
    }

    /// Finds the longest path from the repair droid's current location by initially resetting its
    /// knowledge beyond its current location then crawling map until it is fully explored.
    ///
    /// Search is known to be finished when repair droid is back at origin (no breadcrumbs) and all
    /// tiles around the origin have been explored.
    pub fn find_longest_path_from_start(&mut self) -> u64 {
        self.reset_to_current_location();
        let mut longest_path_seen = 0;
        loop {
            // Check if the droid has returned to the starting location with no more tiles to explore
            if self.breadcrumbs.len() == 0 && self.check_all_around_explored() {
                return longest_path_seen as u64;
            }
            // Check if all directions have been exhausted from current location
            if self.check_all_around_explored() {
                self.rewind_move();
                continue;
            }
            // Check if the target location has been explored - don't need to try move.
            if self.is_target_location_explored() {
                self.rotate_direction();
                continue;
            }
            // Try to move the droid and check outcome of move
            let status = self.try_move();
            match status {
                STATUS_HIT_WALL => {
                    self.handle_bad_move();
                }
                STATUS_GOOD_MOVE => {
                    self.handle_successful_move(false);
                }
                STATUS_GOOD_MOVE_OXYGEN => {
                    self.handle_successful_move(true);
                }
                _ => panic!("Bad move status observed: {}", status),
            }
            // Check if we have seen a longer path than previously observed
            if self.breadcrumbs.len() > longest_path_seen {
                longest_path_seen = self.breadcrumbs.len();
            }
        }
    }
}

/// Calculates the solution for Day 15 Part 1 challenge.
pub fn solution_part_1(filename: String) -> u64 {
    let initial_memory: Vec<i64> = IntcodeMachine::extract_intcode_memory_from_filename(filename);
    let mut repair_droid = RepairDroid::new(initial_memory.clone());
    repair_droid.crawl_map_to_oxygen();
    return repair_droid.get_num_moves_from_origin();
}

/// Calculates the solution for Day 15 Part 2 challenge.
pub fn solution_part_2(filename: String) -> u64 {
    let initial_memory: Vec<i64> = IntcodeMachine::extract_intcode_memory_from_filename(filename);
    let mut repair_droid = RepairDroid::new(initial_memory.clone());
    // Move the droid to the location of the oxygen
    repair_droid.crawl_map_to_oxygen();
    // Find the longest path from the location of oxygen
    let longest_path = repair_droid.find_longest_path_from_start();
    return longest_path;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_d15_p1_solution() {
        let result = solution_part_1(String::from("./input/day_15/input.txt"));
        assert_eq!(208, result);
    }

    #[test]
    fn test_d15_p2_solution() {
        let result = solution_part_2(String::from("./input/day_15/input.txt"));
        assert_eq!(306, result);
    }
}
