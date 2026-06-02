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

fn solve() -> i32 {
    let mut max_val = 0;
    let mut d_max = 0;
    for i in 2..1_000 {
        let finder = RecurringCycleFinder::new(1, i);
        if let Ok(len) = finder.denominator_cycle_length() {
            if len > max_val {
                max_val = len;
                d_max = i;
            }
        }
    }
    d_max
}

pe_utils::pe_main!();
