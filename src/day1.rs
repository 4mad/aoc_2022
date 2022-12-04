//            aoc_2022, day1.rs, David Govorko, 2022-12-01, MIT Or Apache 2.0

///------------------------------ Day 1 Puzzle 1 Objective -------------------------------
/// Find the Elf carrying the most Calories. How many total Calories is that Elf carrying?
/// Calories are found in the Puzzle_1.txt file.
/// Each Elf's calories are listed in order, separated by a blank line.
use std::fs;

///-------------------------------------- Attempt 1 --------------------------------------
/// Read the file line by line. (Single threaded approach)
/// If the line contains a value added it to var temp_total
/// If the line contains a blank, compare temp_total to total
/// If set total = the greater of the two values then keep going until the end of the file
///
/// Slow, too many mutable values, fat elves
pub fn day1_puzzle1_attempt1(input_file_path: &str) {
   // Open and Load whole file into 1 string
   let f = fs::read_to_string(input_file_path).unwrap();

   let mut total: usize = 0;
   let mut temp_total: usize = 0;

   // Loop thru each line
   for line in f.lines() {
      let txt = line;

      // Check if the Line has a value or not
      if !txt.is_empty() {
         // If there is a value, add it to our running temp_total
         temp_total += txt.parse::<usize>().unwrap();
      } else {
         // If the line is empty, compare temp_total to our greatest total so far.
         if temp_total > total {
            total = temp_total;
         }

         temp_total = 0; // Don't forget
      }

      // println!("{} {}", i + 1, txt); // DBG
   }

   // println!("Highest Caloric Density Elf: [{}]!", total);
}

#[test]
pub fn day1_puzzle1_attempt1_test() {
   let input_path = "src/inputs/day 1/puzzle.txt";
   day1_puzzle1_attempt1(input_path);
}

///-------------------------------------- Attempt 2 --------------------------------------
/// Same as attempt one, but using super sexy iterators
///
/// More or less the same in speed, possibly slightly slower
#[allow(unused_variables)]
pub fn day1_puzzle1_attempt2(input_file_path: &str) {
   // Open and Load whole file into 1 string
   let f = fs::read_to_string(input_file_path).unwrap();

   let total = f
      .split("\n\n")
      .map(|each_elf| {
         each_elf
            .lines()
            .map(|single_item| {
               single_item
                  .parse::<usize>()
                  .unwrap()
            })
            .sum::<usize>()
      })
      .max()
      .unwrap();

   // println!("Highest Caloric Density Elf: [{}]!", total);
}

#[test]
pub fn day1_puzzle1_attempt2_test() {
   let input_path = "src/inputs/day 1/puzzle.txt";
   day1_puzzle1_attempt2(input_path);
}

///------------------------------ Day 1 Puzzle 2 Objective -------------------------------
/// Find the 3 Elves carrying the most Calories. Get their total caloric values.

///-------------------------------------- Attempt 1 --------------------------------------
/// Same as first puzzle but have a tuple for the top 3 values and bit more complex if
/// greater than logic
///
/// Slow, too many mutable values, fat elves
pub fn day1_puzzle2_attempt1(input_file_path: &str) {
   // Open and Load whole file into 1 string
   let f = fs::read_to_string(input_file_path).unwrap();

   // This time we have a vec of the top 3 biggest Elves
   let mut total: Vec<usize> = vec![0; 3];
   let mut temp_total: usize = 0;

   // Loop thru each Line
   for line in f.lines() {
      let txt = line;

      // Check if the Line has a value or not
      if !txt.is_empty() {
         // If there is a value, add it to our running temp_total
         temp_total += txt.parse::<usize>().unwrap();
      } else {
         // If the Line is empty, add the temp_total to our current set of 3 totals.
         // Then sort them from greatest to least and pop() out the last (smallest) one
         total.push(temp_total);
         total.sort_by(|a, b| b.cmp(a));
         total.pop();

         temp_total = 0; // Don't forget
      }

      // println!("{} {}", i + 1, txt); //DBG
   }

   // println!("High density Elf over here: [{}]", total);
}

#[test]
pub fn day1_puzzle2_attempt1_test() {
   let input_path = "src/inputs/day 1/puzzle.txt";
   day1_puzzle2_attempt1(input_path);
}
