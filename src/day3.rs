//            aoc_2022, day3.rs, David Govorko, 2022-12-03, MIT Or Apache 2.0

use std::collections::HashSet;
use std::fs;
///------------------------------ Day 3 Puzzle 1 Objective -------------------------------
/// Parse thru each rucksack line and split it into two equal size lines
/// Find the duplicate letter between the two lines
/// Each duplicate letter has a value where a-z = 1-26 & A-Z = 27-52
/// Get the sum of all of the duplicate letters' values

///-------------------------------------- Attempt 1 --------------------------------------
/// Read the file line by line. (Single threaded approach)
/// Split line in half
/// Pass them into the intersection resolver
/// Sum up the results
/// Iterator magic and HashSets
#[allow(unused_variables)]
pub fn day3_puzzle1_attempt1(input_file_path: &str) {
   // Open and Load whole file into 1 string
   let f = fs::read_to_string(input_file_path).unwrap();

   let total = f
      .split('\n')
      .map(|line| get_intersection(line.split_at(line.len() / 2)))
      .sum::<usize>();

   // println!("Total of all lines: [{:#?}]!", total);
}

pub fn get_intersection(sets: (&str, &str)) -> usize {
   // Put each set of values into a HashSet
   let set1: HashSet<&u8> = HashSet::from_iter(sets.0.as_bytes());
   let set2: HashSet<&u8> = HashSet::from_iter(sets.1.as_bytes());
   // println!(
   //    "Aki: {} | 1: {} | 2: {} |",
   //    set1
   //       .intersection(&set2)
   //       .next()
   //       .unwrap()
   //       .to_owned(),
   //    sets.0,
   //    sets.1
   // ); // DGB

   // Get the intersection of the hash set
   let val = set1
      .intersection(&set2)
      .next()
      .unwrap()
      .to_owned();

   // Convert bytes into the problem's arbitrary point system
   match val.cmp(&96) {
      // A starts at 65 in ascii and is worth 26 more points than a
      std::cmp::Ordering::Less => val - 64 + 26,
      // On err
      std::cmp::Ordering::Equal => panic!("We cannot have non numerical characters!"),
      // a starts at 97 in ascii
      std::cmp::Ordering::Greater => val - 96,
   }
   .into()
}

#[test]
pub fn day3_puzzle1_attempt1_test() {
   let input_path = "src/inputs/day 3/puzzle.txt";
   day3_puzzle1_attempt1(input_path);
}

///------------------------------ Day 3 Puzzle 1 Objective -------------------------------
/// Similar to p1 but instead of one line split in 2 its the comparison of 3 lines at once

///-------------------------------------- Attempt 1 --------------------------------------
/// Read the file line by line. (Single threaded approach)
/// Get 3 lines at a time
/// Pass them into the intersection resolver
/// Sum up the results
/// Iterator magic and HashSets
#[allow(unused_variables)]

pub fn day3_puzzle2_attempt1(input_file_path: &str) {
   // Open and Load whole file into 1 string
   let f = fs::read_to_string(input_file_path).unwrap();

   let total: usize = f
      .lines()
      .array_chunks::<3>()
      .map(|three_stack| get_intersection_type2(three_stack.to_vec()))
      .sum();

   // println!("Total of all lines: [{:#?}]!", total);
}

// Runs intersection of all 3 and gets the value of the intersecting letter
pub fn get_intersection_type2(elves: Vec<&str>) -> usize {
   // Put each set of values into a HashSet
   let set1: HashSet<&u8> = HashSet::from_iter(elves[0].as_bytes());
   let set2: HashSet<&u8> = HashSet::from_iter(elves[1].as_bytes());
   let set3: HashSet<&u8> = HashSet::from_iter(elves[2].as_bytes());

   // Get the intersection of all 3 hash sets
   let val = set1
      .intersection(&set2)
      .map(|f| f.to_owned())
      .collect::<HashSet<&u8>>()
      .intersection(&set3)
      .next()
      .unwrap()
      .to_owned();

   // Convert bytes into the problem's arbitrary point system
   match val.cmp(&96) {
      // A starts at 65 in ascii and is worth 26 more points than a
      std::cmp::Ordering::Less => val - 64 + 26,
      // On err
      std::cmp::Ordering::Equal => panic!("We cannot have non numerical characters!"),
      // a starts at 97 in ascii
      std::cmp::Ordering::Greater => val - 96,
   }
   .into()
}

#[test]
pub fn day3_puzzle2_attempt1_test() {
   let input_path = "src/inputs/day 3/puzzle.txt";
   day3_puzzle2_attempt1(input_path);
}
