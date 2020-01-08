use ::num::integer::*;
use euclid::*;

// Expected map characters
const MAP_CHAR_EMPTY: char = '.';
const MAP_CHAR_ASTRD: char = '#';

/// This struct is used to represent the state of an asteroid map, as introduced in Day 10 (2019).
pub struct AsteroidMap {
    map_data: Vec<Vec<char>>,
    asteroid_locations: Vec<euclid::Point2D<i64, UnknownUnit>>,
    map_width: i64,
    map_height: i64,
}

/// Used to represent a quadrant on the X-Y plane. Values should either be +1 or -1.
///         +y
///         |
///   x:-1  |  x:+1
///   y:+1  |  y:+1
///         |
/// --------|-------- +x
///         |
///   x:-1  |  x:+1
///   y:-1  |  y:-1
///         |
struct QuadrantMultiplier {
    x: i64,
    y: i64,
}

impl AsteroidMap {
    /// Creates a new AsteroidMap from the given raw data. Raw data is must consist of lines of
    /// equal length and contain only characters '.' (no asteroid) or '#' (asteroid).
    pub fn new(raw_data: String) -> Self {
        let mut asteroid_locations: Vec<euclid::Point2D<i64, UnknownUnit>> = vec![];
        let mut map_data: Vec<Vec<char>> = vec![vec![]];
        let mut map_width: i64 = 0;
        let mut map_height: i64 = 0;
        // Split raw data into lines
        for line in raw_data.lines() {
            let mut x_loc: i64 = 0;
            let line = line.trim();
            map_data.push(vec![]);
            for c in line.chars() {
                // Check if current square contains asteroid
                if c == MAP_CHAR_ASTRD {
                    asteroid_locations.push(euclid::Point2D::new(x_loc, map_height));
                } else if c != MAP_CHAR_EMPTY {
                    panic!("Bad map char: {}", c);
                }
                map_data[map_height as usize].push(c);
                x_loc += 1;
            }
            // Set the map width data
            if map_height == 0 {
                map_width = x_loc;
            } else {
                if x_loc > map_width {
                    panic!("Poorly formed map - unequal row widths.");
                }
            }
            // Go up a row
            map_height += 1;
        }

        // Create the instance with the processed data
        Self {
            map_data: map_data,
            asteroid_locations: asteroid_locations,
            map_width: map_width + 1,
            map_height: map_height + 1,
        }
    }

    /// Checks if the given location contains an asteroid.
    pub fn contains_asteroid(&self, x: i64, y: i64) -> Result<bool, String> {
        if x < 0 || x >= self.map_width || y < 0 || y >= self.map_height {
            return Err(format!(
                "Provided co-ordinates ({}, {}) exceed map size ({}, {}).",
                x, y, self.map_width, self.map_height
            ));
        }
        let content = self.map_data[y as usize][x as usize];
        return Ok(content == MAP_CHAR_ASTRD);
    }

    /// Looks through all of the asteroids and determines which one is able to see the most of its
    /// counterparts. Return value is tuple consisting of max. asteroids seen (index 0) and location
    /// of the optimal asteroid (index 1).
    pub fn find_optimal_station_location(&self) -> (i64, Point2D<i64, UnknownUnit>) {
        let mut max_asteroids_seen: i64 = 0;
        let mut optimal_location = Point2D::<i64, UnknownUnit>::new(0, 0);
        // Iterate over each asteroid location
        for i in 0..self.asteroid_locations.len() {
            let current_asteroid = self.asteroid_locations[i];
            let mut asteroids_seen = 0;
            for j in 0..self.asteroid_locations.len() {
                let target_asteroid = self.asteroid_locations[j];
                if current_asteroid == target_asteroid {
                    continue;
                }
                // Calculate delta-X and delta-Y
                let dx = (current_asteroid.x - target_asteroid.x).abs();
                let dy = (current_asteroid.y - target_asteroid.y).abs();
                // Reduce both deltas by GCD
                let gcd = gcd(dx, dy);
                let dx = dx / gcd;
                let dy = dy / gcd;
                // Work out what quadrant we are searching in (relative to current asteroid)
                let mut q_mult = QuadrantMultiplier { x: 0, y: 0 };
                if target_asteroid.x >= current_asteroid.x {
                    q_mult.x = 1;
                } else {
                    q_mult.x = -1;
                }
                if target_asteroid.y >= current_asteroid.y {
                    q_mult.y = 1;
                } else {
                    q_mult.y = -1;
                }
                // Check the line of sight
                let mut curr_loc: euclid::Point2D<i64, UnknownUnit> = euclid::Point2D::new(
                    current_asteroid.x + dx * q_mult.x,
                    current_asteroid.y + dy * q_mult.y,
                );
                let mut los_blocked = false;
                while curr_loc != target_asteroid {
                    // Check if current check location is occupied by asteroid
                    if self.contains_asteroid(curr_loc.x, curr_loc.y).unwrap() {
                        los_blocked = true;
                        break;
                    }
                    // Move on to next location
                    curr_loc.x += dx * q_mult.x;
                    curr_loc.y += dy * q_mult.y;
                }
                if !los_blocked {
                    asteroids_seen += 1;
                }
            }
            // Check if we have found a more optimal location
            if asteroids_seen > max_asteroids_seen {
                max_asteroids_seen = asteroids_seen;
                optimal_location = current_asteroid.clone();
            }
        }
        return (max_asteroids_seen, optimal_location);
    }
}
