use super::utils::fs;
use super::utils::io;
use super::utils::math;
use itertools::Itertools;
use regex::Regex;
use std::fmt;
use std::collections::HashSet;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct SpaceObject {
    pos_x: i64,
    pos_y: i64,
    pos_z: i64,
    vel_x: i64,
    vel_y: i64,
    vel_z: i64,
}

impl fmt::Debug for SpaceObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "pos=<x= {}, y= {}, z={}>, vel=<x={}, y={}, z={}>",
            self.pos_x, self.pos_y, self.pos_z, self.vel_x, self.vel_y, self.vel_z
        )
    }
}

impl SpaceObject {
    pub fn new(init_pos_x: i64, init_pos_y: i64, init_pos_z: i64) -> Self {
        Self {
            pos_x: init_pos_x,
            pos_y: init_pos_y,
            pos_z: init_pos_z,
            vel_x: 0,
            vel_y: 0,
            vel_z: 0,
        }
    }

    /// Calculates the hash of the SpaceObject's position and velocity in the x-axis.
    pub fn calculate_x_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.pos_x.hash(&mut hasher);
        self.vel_x.hash(&mut hasher);
        return hasher.finish();
    }

    /// Calculates the hash of the SpaceObject's position and velocity in the y-axis.
    pub fn calculate_y_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.pos_y.hash(&mut hasher);
        self.vel_y.hash(&mut hasher);
        return hasher.finish();
    }

    /// Calculates the hash of the SpaceObject's position and velocity in the z-axis.
    pub fn calculate_z_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.pos_z.hash(&mut hasher);
        self.vel_z.hash(&mut hasher);
        return hasher.finish();
    }

    /// Calculates the SpaceObject's potential energy using the formula introduced in 2019 Day 12.
    pub fn get_potential_energy(&self) -> i64 {
        return self.pos_x.abs() + self.pos_y.abs() + self.pos_z.abs();
    }

    /// Calculates the SpaceObject's kinetic energy using the formula introduced in 2019 Day 12.
    pub fn get_kinetic_energy(&self) -> i64 {
        return self.vel_x.abs() + self.vel_y.abs() + self.vel_z.abs();
    }

    /// Calculates the total energy of the SpaceObject as the sum of its potential and kinetic
    /// energy.
    pub fn get_total_energy(&self) -> i64 {
        return self.get_potential_energy() * self.get_kinetic_energy();
    }

    pub fn add_velocity(&mut self, vel_delta: VelocityDelta) {
        self.vel_x += vel_delta.delta_x;
        self.vel_y += vel_delta.delta_y;
        self.vel_z += vel_delta.delta_z;
    }

    pub fn move_moon(&mut self) {
        self.pos_x += self.vel_x;
        self.pos_y += self.vel_y;
        self.pos_z += self.vel_z;
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct VelocityDelta {
    delta_x: i64,
    delta_y: i64,
    delta_z: i64,
}

impl VelocityDelta {
    pub fn new() -> Self {
        Self {
            delta_x: 0,
            delta_y: 0,
            delta_z: 0,
        }
    }
}

/// Calculates solution to Day 12 Part 1 challenge.
pub fn solution_part_1(filename: String) -> i64 {
    let mut moons = get_moon_data(filename);
    return calculate_total_energy(&mut moons, 1000);
}

/// Calculates the hashes for the state of the x-, y- and z-axes for all the moons taken together.
/// 
/// Returned tuple: (x_hash, y_hash, z_hash)
fn get_moon_xyz_hashes(moons: &Vec<SpaceObject>) -> (u64, u64, u64) {
    let mut x_hasher = DefaultHasher::new();
    let mut y_hasher = DefaultHasher::new();
    let mut z_hasher = DefaultHasher::new();
    for i in 0..4 {
        moons[i].calculate_x_hash().hash(&mut x_hasher);
        moons[i].calculate_y_hash().hash(&mut y_hasher);
        moons[i].calculate_z_hash().hash(&mut z_hasher);
    }
    let x_hash = x_hasher.finish();
    let y_hash = y_hasher.finish();
    let z_hash = z_hasher.finish();
    return (x_hash, y_hash, z_hash);
}

/// Calculates solution for Day 12 Part 2 challenge.
pub fn solution_part_2(filename: String) -> u128 {
    let mut moons = get_moon_data(filename);
    let mut steps: u64 = 0;
    // We are looking at each axis individually, so need to keep hashes for each axis
    let mut x_hashes = HashSet::<u64>::new();
    let mut y_hashes = HashSet::<u64>::new();
    let mut z_hashes = HashSet::<u64>::new();
    let mut x_repeat_steps: u64 = 0;
    let mut y_repeat_steps: u64 = 0;
    let mut z_repeat_steps: u64 = 0;
    // Calculate and store hashes for initial state
    let (x_hash_init, y_hash_init, z_hash_init) = get_moon_xyz_hashes(&moons);
    x_hashes.insert(x_hash_init);
    y_hashes.insert(y_hash_init);
    z_hashes.insert(z_hash_init);
    loop {
        do_moon_step(&mut moons);
        steps += 1;
        if steps % 10000 == 0 {
            println!("Conducted {} steps...", steps);
        }
        let (x_hash, y_hash, z_hash) = get_moon_xyz_hashes(&moons);
        // Check x hash
        if x_hashes.contains(&x_hash) && x_repeat_steps == 0 {
            x_repeat_steps = steps;
        } else {
            x_hashes.insert(x_hash);
        }
        // Check y hash
        if y_hashes.contains(&y_hash) && y_repeat_steps == 0 {
            y_repeat_steps = steps;
        } else {
            y_hashes.insert(y_hash);
        }
        // Check z hash
        if z_hashes.contains(&z_hash) && z_repeat_steps == 0 {
            z_repeat_steps = steps;
        } else {
            z_hashes.insert(z_hash);
        }
        // Check if we have seen a repeat on all axes
        if x_repeat_steps > 0 && y_repeat_steps > 0 && z_repeat_steps > 0 {
            break;
        }
    }
    let repeat_steps = vec![x_repeat_steps, y_repeat_steps, z_repeat_steps];
    let lcm = math::calculate_lcm(repeat_steps);
    return lcm;
}

/// Parses the given file and returns a vector containing the moons specified in file.
fn get_moon_data(filename: String) -> Vec<SpaceObject> {
    // Open file and initialise space objects
    let mut file = fs::open_file(filename);
    let raw_input = io::read_file_to_string(&mut file);
    // Read the input data using a regex and insert objects into array
    let moon_regex = Regex::new(r"<x=(-?\d+), y=(-?\d+), z=(-?\d+)>").unwrap();
    let mut moons: Vec<SpaceObject> = vec![];
    for capture in moon_regex.captures_iter(&raw_input) {
        let x_pos = capture[1].parse().unwrap();
        let y_pos = capture[2].parse().unwrap();
        let z_pos = capture[3].parse().unwrap();
        let moon = SpaceObject::new(x_pos, y_pos, z_pos);
        moons.push(moon);
    }
    return moons;
}

fn do_moon_step(moons: &mut Vec<SpaceObject>) {
    // Initialise velocity delta structs with all zeroes
    let mut velocity_delta: Vec<VelocityDelta> = vec![];
    for _ in 0..4 {
        velocity_delta.push(VelocityDelta::new());
    }
    // For each pair of moons, calculate required velocity deltas
    let moon_pairs = (0..4).combinations(2);
    for pair in moon_pairs {
        let moon_a_index = pair[0];
        let moon_b_index = pair[1];
        let (moon_a_delta, moon_b_delta) =
            calculate_velocity_deltas(moons[moon_a_index], moons[moon_b_index]);
        increment_velocity_delta(&mut velocity_delta[moon_a_index], moon_a_delta);
        increment_velocity_delta(&mut velocity_delta[moon_b_index], moon_b_delta);
    }
    // Apply velocity deltas to moon velocities
    for moon_index in 0..4 {
        moons[moon_index].add_velocity(velocity_delta[moon_index]);
        moons[moon_index].move_moon();
    }
}

fn calculate_total_energy(moons: &mut Vec<SpaceObject>, steps: u64) -> i64 {
    for _ in 0..steps {
        do_moon_step(moons);
    }
    // Calculate total energy of system (all four moons together) after 1000 steps
    let mut system_total_energy = 0;
    for moon_index in 0..4 {
        system_total_energy += moons[moon_index].get_total_energy();
    }
    return system_total_energy;
}

fn increment_velocity_delta(original: &mut VelocityDelta, delta: VelocityDelta) {
    original.delta_x += delta.delta_x;
    original.delta_y += delta.delta_y;
    original.delta_z += delta.delta_z;
}

fn calculate_velocity_deltas(
    moon_a: SpaceObject,
    moon_b: SpaceObject,
) -> (VelocityDelta, VelocityDelta) {
    let mut moon_a_vel_delta = VelocityDelta::new();
    let mut moon_b_vel_delta = VelocityDelta::new();
    // Look at x-axis
    if moon_a.pos_x < moon_b.pos_x {
        moon_a_vel_delta.delta_x = 1;
        moon_b_vel_delta.delta_x = -1;
    } else if moon_a.pos_x > moon_b.pos_x {
        moon_a_vel_delta.delta_x = -1;
        moon_b_vel_delta.delta_x = 1;
    }
    // Look at y-axis
    if moon_a.pos_y < moon_b.pos_y {
        moon_a_vel_delta.delta_y = 1;
        moon_b_vel_delta.delta_y = -1;
    } else if moon_a.pos_y > moon_b.pos_y {
        moon_a_vel_delta.delta_y = -1;
        moon_b_vel_delta.delta_y = 1;
    }
    // Look at z-axis
    if moon_a.pos_z < moon_b.pos_z {
        moon_a_vel_delta.delta_z = 1;
        moon_b_vel_delta.delta_z = -1;
    } else if moon_a.pos_z > moon_b.pos_z {
        moon_a_vel_delta.delta_z = -1;
        moon_b_vel_delta.delta_z = 1;
    }
    return (moon_a_vel_delta, moon_b_vel_delta);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1_ex_input_01() {
        let mut moons = get_moon_data(String::from("./input/day_12/test/test_01.txt"));
        let total_energy = calculate_total_energy(&mut moons, 10);
        assert_eq!(179, total_energy);
    }

    #[test]
    fn test_p1_ex_input_02() {
        let mut moons = get_moon_data(String::from("./input/day_12/test/test_02.txt"));
        let total_energy = calculate_total_energy(&mut moons, 100);
        assert_eq!(1940, total_energy);
    }

    #[test]
    fn test_p1_solution() {
        let result = solution_part_1(String::from("./input/day_12/input.txt"));
        assert_eq!(7179, result);
    }

    #[test]
    fn test_p2_ex_input_01() {
        let result = solution_part_2(String::from("./input/day_12/test/test_01.txt"));
        assert_eq!(2772, result);
    }

    #[test]
    fn test_p2_ex_input_02() {
        let result = solution_part_2(String::from("./input/day_12/test/test_02.txt"));
        assert_eq!(4686774924, result);
    }

    #[test]
    fn test_p2_solution() {
        let result = solution_part_2(String::from("./input/day_12/input.txt"));
        assert_eq!(428576638953552, result);
    }
}
