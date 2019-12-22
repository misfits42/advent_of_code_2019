use std::io::{BufRead, BufReader};
extern crate colored;

/// Calculates the solution to Day 01 Part 1 challenge.
pub fn day_01_solution_p1(filename: String) {
    // Open up the file (read-only)
    let file = super::utils::fs::open_file(filename);
    // Created a buffered reader of the file
    let file = BufReader::new(file);
    // Calculate total fuel requirement
    let mut total_fuel_req = 0;
    for line in file.lines() {
        let module_mass = line.unwrap().parse::<i32>();
        let fuel_req = calculate_fuel_req(module_mass.unwrap());
        total_fuel_req += fuel_req;
    }
    // Display challenge solution
    println!("Day 01 Part 1 solution is: {}", total_fuel_req);
}

/// Calculates the fuel requirement for a single given module mass.
/// 
/// Fuel requirement is calculated by taking module mass and dividing by 3
/// (rounded-down), then subtracting 2. If result of calculation is less than
/// 0, then 0 is returned (negative fuel requirement doesn't make sense!).
fn calculate_fuel_req(module_mass: i32) -> i32 {
    let result = (module_mass / 3) - 2;
    if result < 0 {
        return 0;
    }
    return result;
}
