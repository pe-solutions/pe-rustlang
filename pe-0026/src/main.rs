// Reciprocal Cycles
// https://projecteuler.net/problem=26

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]

// Special purpose bare Rational type
struct RecurringCycleFinder {
    numerator: i32,
    denominator: i32,
}

impl RecurringCycleFinder {
    fn new(numerator: i32, denominator: i32) -> Self {
        RecurringCycleFinder { numerator, denominator }
    }

    fn denominator_cycle_length (&self) -> Result<i32, &'static str> {
        if self.denominator == 0 {
            return Err("Denominator cannot be zero");
        }

        let mut remainders = HashMap::new();

        let (mut numerator, mut position) = (self.numerator, 0);

        while !remainders.contains_key(&numerator) && numerator != 0 {
            remainders.insert(numerator, position);
            numerator = (numerator % self.denominator) * 10;
            position += 1;
        }
        
        Ok(if numerator == 0 {0} else {position - remainders[&numerator]})
    }
}

fn main() {
    let start = std::time::Instant::now();
    
    // Length of the recurring cycle
    let mut max_val = 0;
    
    // Denominator with the longest recurring cycle
    let mut d_max = 0;

    for i in 2..1_000 {
        let finder = RecurringCycleFinder::new(1, i);

        match finder.denominator_cycle_length () {
            Ok(recurring_cycle_length) => {
                if recurring_cycle_length > max_val {
                    max_val = recurring_cycle_length;
                    d_max = i;
                }
            }
            
            Err(err) => println!("Error: {}", err),
        }
    }

    let answer = d_max;

    let duration = start.elapsed();

    println!("\nProject Euler #26\nAnswer: {}", answer);
    println!("Elapsed time: {} milliseconds.\n", duration.as_millis());
}
