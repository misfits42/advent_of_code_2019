use super::utils::fs;
use super::utils::io;
use super::utils::maps::AsteroidMap;
use euclid::*;

/// Calculates the solution for Day 10 Part 1.
pub fn solution_part_1(filename: String) -> (i64, euclid::Point2D<i64, UnknownUnit>) {
    return get_optimal_location(filename);
}

fn get_optimal_location(filename: String) -> (i64, euclid::Point2D<i64, UnknownUnit>) {
    let mut file = fs::open_file(filename);
    let raw_input = io::read_file_to_string(&mut file);
    let asteroid_map = AsteroidMap::new(raw_input);
    let result = asteroid_map.find_optimal_station_location();
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1_ex_input_01() {
        let result = get_optimal_location(String::from("./input/day_10/test/test_01.txt"));
        assert_eq!(8, result.0);
        assert_eq!(3, result.1.x);
        assert_eq!(4, result.1.y);
    }

    #[test]
    fn test_p1_ex_input_02() {
        let result = get_optimal_location(String::from("./input/day_10/test/test_02.txt"));
        assert_eq!(33, result.0);
        assert_eq!(5, result.1.x);
        assert_eq!(8, result.1.y);
    }

    #[test]
    fn test_p1_ex_input_03() {
        let result = get_optimal_location(String::from("./input/day_10/test/test_03.txt"));
        assert_eq!(35, result.0);
        assert_eq!(1, result.1.x);
        assert_eq!(2, result.1.y);
    }

    #[test]
    fn test_p1_ex_input_04() {
        let result = get_optimal_location(String::from("./input/day_10/test/test_04.txt"));
        assert_eq!(41, result.0);
        assert_eq!(6, result.1.x);
        assert_eq!(3, result.1.y);
    }

    #[test]
    fn test_p1_ex_input_05() {
        let result = get_optimal_location(String::from("./input/day_10/test/test_05.txt"));
        assert_eq!(210, result.0);
        assert_eq!(11, result.1.x);
        assert_eq!(13, result.1.y);
    }
}
