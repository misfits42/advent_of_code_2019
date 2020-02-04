// Day module declarations
pub mod day_01;
pub mod day_02;
pub mod day_03;
pub mod day_04;
pub mod day_05;
pub mod day_06;
pub mod day_07;
pub mod day_08;
pub mod day_09;
pub mod day_10;
pub mod day_11;
pub mod day_12;
pub mod day_13;
pub mod day_14;
pub mod day_15;
pub mod day_16;
// Other required declarations
pub mod utils;
// Declare itertools here with tag so its macros can be used in project
#[macro_use] extern crate itertools;
extern crate euclid;
extern crate num;
extern crate png;
extern crate regex;
extern crate primes;

fn main() {
    let solution = day_16::solution_part_1(String::from("./input/day_16/test/test_01.txt"));
    println!("Solution: {:?}", solution);
}
