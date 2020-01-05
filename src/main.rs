// Day module declarations
pub mod day_01;
pub mod day_02;
pub mod day_03;
pub mod day_04;
pub mod day_05;
pub mod day_06;
pub mod day_07;
// Other required declarations
pub mod utils;
// Declare itertools here with tag so its macros can be used in project
#[macro_use] extern crate itertools;
extern crate queues;

fn main() {
    let solution = day_07::solution_part_2(String::from("./input/day_07/input.txt"));
    println!("Solution: {:?}", solution);
}
