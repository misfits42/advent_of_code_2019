use std::collections::HashMap;
use std::collections::HashSet;
use super::utils::fs;
use super::utils::io;
use petgraph::graphmap::GraphMap;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    /// Creates a new point with the given co-ordinates.
    pub fn new(x: i64, y: i64) -> Self {
        Self {
            x: x,
            y: y,
        }
    }

    /// Calculates the points surrounding the current point.
    pub fn get_surrounding_points(&self) -> Vec<Point> {
        let mut surr: Vec<Point> = vec![];
        for i in 0..4 {
            let mut new_point = self.clone();
            match i {
                0 => new_point.x -= 1,
                1 => new_point.x += 1,
                2 => new_point.y -= 1,
                3 => new_point.y += 1,
                _ => (),
            }
            surr.push(new_point);
        }
        return surr;
    }
}

pub fn solution_part_1(filename: String) -> u64 {
    // Read contents of input file
    let mut file = fs::open_file(filename);
    let mut raw_input = io::read_file_to_string(&mut file);
    raw_input = raw_input.trim().to_owned();
    // Store map state read from input file
    let mut vault_map: HashMap<Point, char> = HashMap::new();
    let mut location = Point::new(0, 0);
    let mut key_and_door_locations: HashMap<Point, char> = HashMap::new();
    // Keep track of the keys not yet held and the doors still locked
    let mut keys_outstanding: HashSet<char> = HashSet::new();
    let mut doors_locked: HashSet<char> = HashSet::new();
    // Load up map state and record locations of keys, doors and start
    for line in raw_input.lines() {
        let line = line.trim(); // Remove any trailing whitespace to remove spurious characters
        for c in line.chars() {
            if c == '@' || c.is_ascii_alphabetic() {
                key_and_door_locations.insert(location.clone(), c);
                if c.is_ascii_lowercase() {
                    keys_outstanding.insert(c);
                } else if c.is_ascii_uppercase() {
                    doors_locked.insert(c);
                }
            }
            vault_map.insert(location.clone(), c);
            location.x += 1;
        }
        location.y += 1;
        location.x = 0;
    }
    let mut vault_graph: GraphMap<char, u64, petgraph::Undirected> = GraphMap::new();
    // Look at the start, key and door locations
    for loc_of_interest in key_and_door_locations.keys() {
        // Check if the graph already has the current interesting spot as a node
        let curr_spot = *key_and_door_locations.get(loc_of_interest).unwrap();
        if !vault_graph.contains_node(curr_spot) {
            vault_graph.add_node(curr_spot);
        }
        // Keep track of locations explored from interesting spot
        let mut explored_locations: HashSet<Point> = HashSet::new();
        explored_locations.insert(*loc_of_interest);
        let mut path_heads: HashSet<Point> = HashSet::new();
        // Find the initial heads
        let mut steps_taken = 1;
        let init_surround = loc_of_interest.get_surrounding_points();
        for adj in init_surround {
            let spot = *vault_map.get(&adj).unwrap();
            if spot.is_ascii_alphabetic() { // adjacent to a key or door
                if !vault_graph.contains_node(spot) {
                    vault_graph.add_node(spot);
                }
                if !vault_graph.contains_edge(spot, curr_spot) {
                    vault_graph.add_edge(spot, curr_spot, steps_taken);
                }
            } else if spot == '.' { // Found new tile to explore
                path_heads.insert(adj);
                explored_locations.insert(adj);
            }
        }
        // Keep moving out from the current start spot until no more places to move.
        steps_taken += 1;
        loop {
            let mut new_heads: HashSet<Point> = HashSet::new();
            // Try to explore from the head location of each search path
            for head in path_heads.iter() {
                let surr_points = head.get_surrounding_points();
                for adj in surr_points {
                    let next_spot = *vault_map.get(&adj).unwrap();
                    if explored_locations.contains(&adj) {
                        continue;
                    }
                    if next_spot.is_ascii_alphabetic() {
                        if !vault_graph.contains_node(next_spot) {
                            vault_graph.add_node(next_spot);
                        }
                        if !vault_graph.contains_edge(next_spot, curr_spot) && next_spot != curr_spot {
                            vault_graph.add_edge(next_spot, curr_spot, steps_taken);
                        }
                    } else if next_spot == '.' {
                        new_heads.insert(adj);
                        explored_locations.insert(adj);
                    }
                }
            }
            steps_taken += 1;
            // Set path heads to newly discovered heads
            path_heads.clear();
            path_heads = new_heads.clone();
            // All connections from current node have been found when path heads are exhausted.
            if path_heads.is_empty() {
                break;
            }
        }
    }
    // TODO: implement algorithm to find min. steps to collect all keys
    unimplemented!();
}
