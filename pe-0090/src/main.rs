// Cube Digit Pairs
// https://projecteuler.net/problem=90

use std::collections::HashSet;

fn can_represent_squares(die1: &[u8], die2: &[u8]) -> bool {
    let squares = vec![(0, 1), (0, 4), (0, 9), (1, 6), (2, 5), (3, 6), (4, 9), (6, 4), (8, 1)];
    
    for &(a, b) in &squares {
        if !((die1.contains(&a) && die2.contains(&b)) || (die1.contains(&b) && die2.contains(&a))) {
            return false;
        }
    }
    true
}

fn generate_combinations(digits: &[u8], k: usize) -> Vec<Vec<u8>> {
    let mut combinations = vec![];
    let mut combination = vec![];

    fn backtrack(start: usize, digits: &[u8], k: usize, combination: &mut Vec<u8>, combinations: &mut Vec<Vec<u8>>) {
        if combination.len() == k {
            combinations.push(combination.clone());
            return;
        }
        for i in start..digits.len() {
            combination.push(digits[i]);
            backtrack(i + 1, digits, k, combination, combinations);
            combination.pop();
        }
    }

    backtrack(0, digits, k, &mut combination, &mut combinations);
    combinations
}

fn count_valid_pairs() -> usize {
    let digits = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let combinations = generate_combinations(&digits, 6);
    let mut valid_pairs = HashSet::new();

    for i in 0..combinations.len() {
        for j in i..combinations.len() {
            let mut die1 = combinations[i].clone();
            let mut die2 = combinations[j].clone();

            // Handle 6/9 interchangeability
            if die1.contains(&6) && !die1.contains(&9) {
                die1.push(9);
            } else if die1.contains(&9) && !die1.contains(&6) {
                die1.push(6);
            }
            if die2.contains(&6) && !die2.contains(&9) {
                die2.push(9);
            } else if die2.contains(&9) && !die2.contains(&6) {
                die2.push(6);
            }

            // Sort the dice to ensure canonical order
            if die1 > die2 {
                std::mem::swap(&mut die1, &mut die2);
            }

            if can_represent_squares(&die1, &die2) {
                valid_pairs.insert((die1, die2));
            }
        }
    }

    valid_pairs.len()
}

fn main() {
    let start = std::time::Instant::now();
    //
    let result = count_valid_pairs();
    //
    let duration = start.elapsed();
    
    println!("\nProject Euler #90\nAnswer: {}", result);
    println!("Elapsed time: {:?} milliseconds.\n", duration.as_millis());
}
