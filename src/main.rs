pub mod day_01;
pub mod day_02;
pub mod utils;

#[macro_use] extern crate itertools;

fn main() {
    day_02::day_02_solution_p2(String::from("./input/day_02/input.txt"));
}
