// Power Digit Sum
// https://projecteuler.net/problem=16

use num_bigint::{ToBigUint};

fn calculate_sum_of_digits_of_power(exponent: u32) -> u32 {
    let base = 2u32.to_biguint().unwrap();
    let power = base.pow(exponent);

    let digits = power.to_string();
    
    let result: u32 = digits
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .sum();
    
    result
}

fn main() {
    let start = std::time::Instant::now();
    //

    let exponent = 1000;
    let answer = calculate_sum_of_digits_of_power(exponent);

    //
    let duration = start.elapsed();

    println!("\nProject Euler #16\nAnswer: {}", answer);
    println!("Elapsed time: {} milliseconds.\n", duration.as_millis());   
}
