//            aoc_2022, day2.rs, David Govorko, 2022-12-02, MIT Or Apache 2.0

///------------------------------ Day 2 Puzzle 1 Objective -------------------------------
/// Calculate the total score of rock paper scissors.
/// Winning a round nets you 6 points, 3 on a draw, and 0 on a loss.
/// Additional points are rewarded based on the hand you played.
/// 3 points for scissors (Z\C), 2 points for paper (Y\B), and 1 point for rock (X\A)
use std::fs;

///-------------------------------------- Attempt 1 --------------------------------------
/// Read the file line by line. (Single threaded approach)
/// Get first and second value and run the "simulation".
/// Add that score to the total tally.
///
/// Slow, probably can be done faster with some iterator mapping magic
#[allow(unused_variables)]
pub fn day2_puzzle1_attempt1(input_file_path: &str) {
   // Open and Load whole file into 1 string
   let f = fs::read_to_string(input_file_path).unwrap();

   let mut total: usize = 0;

   // Run simulation for each line
   for line in f.lines() {
      let txt = line;

      // The simulation function will return the game results
      total += rps_simulation(txt.split_at(1));
   }

   //    println!("The final score would be: [{}]!", total);
}

// Game logic baked into a match
#[allow(clippy::identity_op)]
fn rps_simulation(hands: (&str, &str)) -> usize {
   match hands {
      ("A", " X") => 3 + 1,
      ("A", " Y") => 6 + 2,
      ("A", " Z") => 0 + 3,
      ("B", " X") => 0 + 1,
      ("B", " Y") => 3 + 2,
      ("B", " Z") => 6 + 3,
      ("C", " X") => 6 + 1,
      ("C", " Y") => 0 + 2,
      ("C", " Z") => 3 + 3,
      // Error line
      (_, _) => panic!(
         "Not sure how to resolve a hand of: ([{}], [{}])!",
         hands.0, hands.1
      ),
   }
}

#[test]
pub fn day2_puzzle1_attempt1_test() {
   let input_path = "src/inputs/day 2/puzzle.txt";
   day2_puzzle1_attempt1(input_path);
}

///------------------------------ Day 2 Puzzle 2 Objective -------------------------------
/// Same game as puzzle 1 but now the X stands for LOSE, Y for DRAW, and Z for WIN.

///-------------------------------------- Attempt 1 --------------------------------------
/// Read the file line by line. (Single threaded approach)
/// Get first and second value and run the "simulation".
/// Add that score to the total tally.
///
/// Slow, probably can be done faster with some iterator mapping magic
/// Also the "simulation is dumb atm"
#[allow(unused_variables)]
pub fn day2_puzzle2_attempt1(input_file_path: &str) {
   // Open and Load whole file into 1 string
   let f = fs::read_to_string(input_file_path).unwrap();

   let mut total: usize = 0;

   // Run simulation for each line
   for line in f.lines() {
      let txt = line;

      // The simulation function will return the game results
      total += rps_simulation_type2(txt.split_at(1));
   }

   //    println!("The final score would be: [{}]!", total);
}

// Game logic (for part 2) baked into a match
#[allow(clippy::identity_op)]
fn rps_simulation_type2(hands: (&str, &str)) -> usize {
   match hands {
      ("A", " X") => 0 + 3,
      ("A", " Y") => 3 + 1,
      ("A", " Z") => 6 + 2,
      ("B", " X") => 0 + 1,
      ("B", " Y") => 3 + 2,
      ("B", " Z") => 6 + 3,
      ("C", " X") => 0 + 2,
      ("C", " Y") => 3 + 3,
      ("C", " Z") => 6 + 1,
      // Error line
      (_, _) => panic!(
         "Not sure how to resolve a hand of: ([{}], [{}])!",
         hands.0, hands.1
      ),
   }
}

#[test]
pub fn day2_puzzle2_attempt1_test() {
   let input_path = "src/inputs/day 2/puzzle.txt";
   day2_puzzle2_attempt1(input_path);
}

///-------------------------------------- Attempt 2 --------------------------------------
/// Same as Attempt 1 but...
/// Create a table of strings that are "A X", "A Y", ... indexed according to their scores
/// Then match them to get the results
///
/// This attempt is actually slightly slower than attempt 1
#[allow(unused_variables)]
pub fn day2_puzzle2_attempt2(input_file_path: &str) {
   // Open and Load whole file into 1 string
   let f = fs::read_to_string(input_file_path).unwrap();

   let mut total: usize = 0;

   // Each potential line is sorted into this vec at the index = to their points won
   let score = vec![
      "B X", "C X", "A X", "A Y", "B Y", "C Y", "C Z", "A Z", "B Z",
   ];

   // Run simulation for each line
   for line in f.lines() {
      let txt = line;

      // Iterate thru the vec, get its matching index, then add one to account for 0 base
      total += score
         .iter()
         .position(|p| p == &txt)
         .unwrap()
         + 1;
   }

   //    println!("The final score would be: [{}]!", total);
}

#[test]
pub fn day2_puzzle2_attempt2_test() {
   let input_path = "src/inputs/day 2/puzzle.txt";
   day2_puzzle2_attempt2(input_path);
}
