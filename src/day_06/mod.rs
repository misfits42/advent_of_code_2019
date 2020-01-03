use super::utils::fs;
use std::io::BufReader;
use std::io::BufRead;
use std::io::Lines;
use std::fs::File;
use std::collections::HashMap;

use super::utils::orbit::OrbitNode;

/// Calculates the solution for Day 06 Part 1.
pub fn solution_part_1(filename: String) -> i32 {
    return get_number_of_orbits(filename);
}

/// Calculates the solution for Day 06 Part 2.
pub fn solution_part_2(filename: String) -> i32 {
    return get_number_of_orbit_transfers(filename, String::from("YOU"), String::from("SAN"));
}

/// Calculates the number of orbit transfers required to get from the start node
/// to the end node, using the map file included in the given map file.
fn get_number_of_orbit_transfers(filename: String, start_id: String, end_id: String) -> i32 {
    let file = fs::open_file(filename);
    let lines = BufReader::new(file).lines();
    let nodes = process_orbit_map(lines);
    let mut start_id_parent_id = nodes.get(&start_id).unwrap().get_parent_id();
    let mut end_id_parent_id = nodes.get(&end_id).unwrap().get_parent_id();
    let mut start_id_parent_chain = Vec::<String>::new();
    let mut end_id_parent_chain = Vec::<String>::new();
    // Generate the chain of parents from the start node
    loop {
        // Check if we have reached most senior parent of start node
        if start_id_parent_id.is_empty() {
            break;
        }
        let new_parent_id = nodes.get(&start_id_parent_id).unwrap().get_parent_id();
        start_id_parent_chain.push(start_id_parent_id);
        start_id_parent_id = new_parent_id;
    }
    // Generate the chain of parents from the end node
    loop {
        // Check if we have reached most senior parent
        if end_id_parent_id.is_empty() {
            break;
        }
        let new_parent_id = nodes.get(&end_id_parent_id).unwrap().get_parent_id();
        end_id_parent_chain.push(end_id_parent_id);
        end_id_parent_id = new_parent_id;
    }
    start_id_parent_chain.reverse();
    end_id_parent_chain.reverse();
    // Find the node that overlaps in the parent
    let mut last_common_parent = String::from("");
    for i in 0..start_id_parent_chain.len() {
        // Check if we have reached end of end_id parent chain
        if i == end_id_parent_chain.len() {
            break;
        }
        // Still going - we have another common chain
        if start_id_parent_chain[i] == end_id_parent_chain[i] {
            last_common_parent = start_id_parent_chain[i].clone();
        } else { // Parents differ, so end loop
            break;
        }
    }
    // Calculate number of parents
    let num_parents_start_id = get_number_of_parents(&start_id, &nodes);
    let num_parents_end_id = get_number_of_parents(&end_id, &nodes);
    let num_parents_common_parent = get_number_of_parents(&last_common_parent, &nodes);
    // Calculate result - subtract two to extract first jump from start and end ids
    let result = num_parents_start_id + num_parents_end_id - 2*num_parents_common_parent - 2;
    return result;
}

/// Gets the total number of direct and indirect orbits in the given map file. Panics
/// if a bad map file is given.
fn get_number_of_orbits(filename: String) -> i32 {
    let file = fs::open_file(filename);
    let lines: Lines<BufReader<File>> = BufReader::new(file).lines();
    // Keep track of each node in the orbit map
    let nodes = process_orbit_map(lines);
    // Now we need to check how many parents each node has and add these up
    let mut num_orbits = 0;
    for node_name in nodes.keys() {
        num_orbits += get_number_of_parents(node_name, &nodes);
    }
    return num_orbits;
}

/// Takes the lines from an orbit map and constructs a HashMap consisting of each
/// node specified somewhere in the file connected to each of its parent and children
/// nodes.
fn process_orbit_map(raw_input: Lines<BufReader<File>>) -> HashMap<String, OrbitNode> {
    let mut nodes = HashMap::<String, OrbitNode>::new();
    for raw_line in raw_input {
        let line = raw_line.unwrap();
        let args: Vec<&str> = line.split(")").collect();
        let centre_of_mass = String::from(args[0]);
        let orbiter = String::from(args[1]);
        // Add the new orbit relationships to the node map
        if !nodes.contains_key(&centre_of_mass) && !nodes.contains_key(&orbiter) {
            // Create new node for COM and orbiter
            let mut com_node = OrbitNode::new(centre_of_mass.clone());
            let mut orbit_node = OrbitNode::new(orbiter.clone());
            com_node.add_child_id(orbiter.clone());
            orbit_node.add_parent_id(centre_of_mass.clone());
            nodes.insert(centre_of_mass, com_node);
            nodes.insert(orbiter, orbit_node);
        } else if nodes.contains_key(&centre_of_mass) && !nodes.contains_key(&orbiter) {
            // Retrieve existing COM and create new node for orbiter
            if let Some(com_node) = nodes.get_mut(&centre_of_mass) {
                com_node.add_child_id(orbiter.clone());
            }
            let mut orbit_node = OrbitNode::new(orbiter.clone());
            orbit_node.add_parent_id(centre_of_mass.clone());
            nodes.insert(orbiter, orbit_node);
        } else if !nodes.contains_key(&centre_of_mass) && nodes.contains_key(&orbiter) {
            // Create new node for COM and retrieve existing node for orbiter
            let mut com_node = OrbitNode::new(centre_of_mass.clone());
            com_node.add_child_id(orbiter.clone());
            if let Some(orbit_node) = nodes.get_mut(&orbiter) {
                orbit_node.add_parent_id(centre_of_mass.clone());
            }
            nodes.insert(centre_of_mass, com_node);
        } else {
            // Retrieve existing nodes for COM and orbiter
            if let Some(com_node) = nodes.get_mut(&centre_of_mass) {
                com_node.add_child_id(orbiter.clone());
            }
            if let Some(orbit_node) = nodes.get_mut(&orbiter) {
                orbit_node.add_parent_id(centre_of_mass.clone());
            }
        }
    }
    return nodes;
}

/// Gets the number of parents the given OrbitNode has. 
fn get_number_of_parents(node_name: &String, nodes: &HashMap<String, OrbitNode>) -> i32 {
    let node = nodes.get(node_name).unwrap();
    if !node.has_parent() {
        return 0;
    } else {
        let parent_id: String = node.get_parent_id();
        return 1 + get_number_of_parents(&parent_id, nodes);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1_example_input() {
        let result = get_number_of_orbits(String::from("./input/day_06/test/test_01.txt"));
        assert_eq!(42, result);
    }

    #[test]
    fn test_p2_example_input() {
        let result = get_number_of_orbit_transfers(String::from("./input/day_06/test/test_02.txt"), String::from("YOU"), String::from("SAN"));
        assert_eq!(4, result);
    }
}