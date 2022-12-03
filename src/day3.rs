//            aoc_2022, day3.rs, David Govorko, 2022-12-03, MIT Or Apache 2.0

///------------------------------ Day 3 Puzzle 1 Objective -------------------------------
/// Parse thru each rucksack line and split it into two equal size lines
/// Find the duplicate letter between the two lines
/// Each duplicate letter has a value where a-z = 1-26 & A-Z = 27-52
/// Get the sum of all of the duplicate letters' values
use std::fs;

///-------------------------------------- Attempt 1 --------------------------------------
/// Read the file line by line. (Single threaded approach)
/// Iterator magic
#[allow(unused_variables)]
pub fn day3_puzzle1_attempt1(input_file_path: &str) {
   // Open and Load whole file into 1 string
   let f = fs::read_to_string(input_file_path).unwrap();

   let total = f.split("\n");

   // .map(|rucksack| rucksack.as_bytes());

   println!("Total of all lines: [{:#?}]!", total);
}

#[test]
pub fn day3_puzzle1_attempt1_test() {
   let input_path = "src/inputs/day 3/puzzle.txt";
   day3_puzzle1_attempt1(input_path);
}
