// Counting Sundays
// https://projecteuler.net/problem=19

extern crate chrono;

use chrono::{Datelike, NaiveDate, Weekday};

fn main() {
    let start = std::time::Instant::now();
    //
    
    let mut answer = 0u32;

    for year in 1901..=2000 {
        for month in 1..=12 {
            if let Some(date) = NaiveDate::from_ymd_opt(year, month, 1) {
                if date.weekday() == Weekday::Sun {
                    answer += 1;
                }
            }
        }
    }

    // 
    let duration = start.elapsed();

    println!("\nProject Euler #19\nAnswer: {}", answer);
    println!("Elapsed time: {:?} milliseconds.\n", duration.as_millis());

}
