// Day module declarations
pub mod day_01;
pub mod day_02;
pub mod day_03;
// Other required declarations
pub mod utils;
// Declare itertools here with tag so its macros can be used in project
#[macro_use] extern crate itertools;

fn main() {
    day_03::day_03_solution_p1(String::from("./input/day_03/input.txt"));
}
