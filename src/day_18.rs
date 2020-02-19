use std::collections::HashMap;
use std::collections::HashSet;
use super::utils::fs;
use super::utils::io;
use petgraph::graphmap::GraphMap;
use itertools::Itertools;
// use std::thread;
use crossbeam::thread;

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
    let mut keys_outstanding: Vec<char> = vec![];
    let mut doors_locked: HashSet<char> = HashSet::new();
    // Load up map state and record locations of keys, doors and start
    for line in raw_input.lines() {
        let line = line.trim(); // Remove any trailing whitespace to remove spurious characters
        for c in line.chars() {
            if c == '@' || c.is_ascii_alphabetic() {
                key_and_door_locations.insert(location.clone(), c);
                if c.is_ascii_lowercase() {
                    keys_outstanding.push(c);
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
    let mut key_orders = keys_outstanding.iter().permutations(keys_outstanding.len());
    let mut min_steps_seen = u64::max_value();
    let mut count = 0;
    loop {
        count += 20;
        if count % 10000 == 0 {
            println!("Up to key order {}", count);
        }
        let mut orders_finished = false;
        thread::scope(|s| {
            let mut handles: Vec<thread::ScopedJoinHandle<Option<u64>>> = vec![];
            for _ in 0..20 {
                let order = key_orders.next();
                match order {
                    Some(v) => {
                        let handle = s.spawn(|_| {
                            get_steps_for_key_order(&vault_graph.clone(), v)
                        });
                        handles.push(handle);
                    },
                    None => {
                        orders_finished = true;
                        break;
                    }
                }
            }
            let mut min_steps_batch = u64::max_value();
            for handle in handles {
                let result = handle.join().unwrap();
                match result {
                    None => continue,
                    Some(steps_taken) => {
                        if steps_taken < min_steps_batch {
                            min_steps_batch = steps_taken;
                        }
                    },
                }
            }
            if min_steps_batch < min_steps_seen {
                min_steps_seen = min_steps_batch;
            }
        }).unwrap();
        if orders_finished {
            break;
        }
    }
    return min_steps_seen;
}

fn get_steps_for_key_order(vault_graph: &GraphMap<char, u64, petgraph::Undirected>, key_order: Vec<&char>) -> Option<u64> {
    let mut current_node = '@';
    let mut steps_taken = 0;
    let mut keys_remaining = key_order.clone();
    let mut vault_map_copy = vault_graph.clone();
    for target_key in key_order {
        if !vault_map_copy.contains_edge(current_node, *target_key) {
            return None;
        }
        // Remove current node
        steps_taken += remove_node_and_update_edges(&mut vault_map_copy, current_node, *target_key);
        // Move to the nearest key
        current_node = *target_key;
        keys_remaining.retain(|x| *x != target_key);
        // Open corresponding door
        let door = target_key.to_ascii_uppercase();
        remove_node_and_update_edges(&mut vault_map_copy, door, *target_key);
        // Check if we have picked up all the keys
        if keys_remaining.is_empty() {
            break;
        }
    }
    return Some(steps_taken);
}

fn remove_node_and_update_edges(vault_graph: &mut GraphMap<char, u64, petgraph::Undirected>, current_node: char, nearest_key: char) -> u64 {
    // Make new neighbour connections if new path through current node is shorter
    let neighbours: Vec<char> = vault_graph.neighbors(current_node).map(|x| x).collect();
    let neighbour_pairs = neighbours.iter().permutations(2);
    for pair in neighbour_pairs {
        // Calculate what the new path length would be if the neighbours were connected
        let first_steps = *vault_graph.edge_weight(current_node, *pair[0]).unwrap();
        let second_steps = *vault_graph.edge_weight(current_node, *pair[1]).unwrap();
        let new_steps = first_steps + second_steps;
        // Check if new edge should be added or updated (if exists and old steps less than new)
        if !vault_graph.contains_edge(*pair[0], *pair[1]) {
            vault_graph.add_edge(*pair[0], *pair[1], new_steps);
        } else if *vault_graph.edge_weight(*pair[0], *pair[1]).unwrap() > new_steps {
            vault_graph.add_edge(*pair[0], *pair[1], new_steps);
        }
    }
    // Remove current node from vault graph
    let mut steps_taken: u64 = 0;
    for neighbour in neighbours.iter() {
        if *neighbour == nearest_key {
            steps_taken = *vault_graph.edge_weight(current_node, *neighbour).unwrap();
        }
        vault_graph.remove_edge(current_node, *neighbour);
    }
    vault_graph.remove_node(current_node);
    return steps_taken;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_d18_example_01() {
        let result = solution_part_1(String::from("./input/day_18/test/test_01.txt"));
        assert_eq!(8, result);
    }

    #[test]
    fn test_d18_example_02() {
        let result = solution_part_1(String::from("./input/day_18/test/test_02.txt"));
        assert_eq!(86, result);
    }

    #[test]
    fn test_d18_example_03() {
        let result = solution_part_1(String::from("./input/day_18/test/test_03.txt"));
        assert_eq!(132, result);
    }

    #[test]
    fn test_d18_example_04() {
        let result = solution_part_1(String::from("./input/day_18/test/test_04.txt"));
        assert_eq!(136, result);
    }

    #[test]
    fn test_d18_example_05() {
        let result = solution_part_1(String::from("./input/day_18/test/test_05.txt"));
        assert_eq!(81, result);
    }
}
