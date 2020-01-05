// Day module declarations
pub mod day_01;
pub mod day_02;
pub mod day_03;
pub mod day_04;
pub mod day_05;
pub mod day_06;
pub mod day_07;
pub mod day_08;
// Other required declarations
pub mod utils;
// Declare itertools here with tag so its macros can be used in project
#[macro_use] extern crate itertools;

fn main() {
    let solution = day_08::solution_part_1(String::from("./input/day_08/input.txt"));
    println!("Solution: {:?}", solution);
}
