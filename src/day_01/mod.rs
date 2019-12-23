use std::io::{BufRead, BufReader};

/// Calculates the solution to Day 01 Part 1 challenge.
pub fn solution_part_1(filename: String) {
    // Open up the file (read-only)
    let file = super::utils::fs::open_file(filename);
    // Created a buffered reader of the file
    let file = BufReader::new(file);
    // Calculate total fuel requirement
    let mut total_fuel_req = 0;
    for line in file.lines() {
        let module_mass = line.unwrap().parse::<i32>().unwrap();
        let fuel_req = calculate_fuel_req(module_mass);
        total_fuel_req += fuel_req;
    }
    // Display challenge solution
    println!("Day 01 Part 1 solution is: {}", total_fuel_req);
}

/// Calculates the solution to Day 01 Part 2 challenge.
pub fn solution_part_2(filename: String) {
    // Open up the file
    let file = super::utils::fs::open_file(filename);
    // Create a buffered reader
    let file = BufReader::new(file);
    // Calculate the total fuel requirement (recursive)
    let mut total_fuel_req = 0;
    for line in file.lines() {
        let module_mass = line.unwrap().parse::<i32>().unwrap();
        let fuel_req = calculate_fuel_req_recursive(module_mass);
        total_fuel_req += fuel_req;
    }
    // Display challenge solution
    println!("Day 01 Part 2 solution is: {}", total_fuel_req);
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

/// Calculates the total fuel requirement for given module mass, taking into
/// account the fuel required to accomodate the fuel mass itself.
fn calculate_fuel_req_recursive(module_mass: i32) -> i32 {
    let mut fuel_req = module_mass;
    let mut total_fuel_req = 0;
    loop {
        fuel_req = calculate_fuel_req(fuel_req);
        if fuel_req <= 0 {
            break;
        }
        total_fuel_req += fuel_req;
    }
    return total_fuel_req;
}
