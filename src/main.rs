// Day module declarations
pub mod day_01;
pub mod day_02;
pub mod day_03;
// Other required declarations
pub mod utils;
// Declare itertools here with tag so its macros can be used in project
#[macro_use] extern crate itertools;

fn main() {
    let solution = day_02::solution_part_1(String::from("./input/day_02/test/test_01.txt"));
    println!("Solution: {}", solution);
}
