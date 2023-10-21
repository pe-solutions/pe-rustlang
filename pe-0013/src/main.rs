// Large sum
// https://projecteuler.net/problem=13

extern crate num_bigint;
use num_bigint::BigUint;

use std::fs;
use std::io;

fn large_sum(arr: Vec<String>) -> Result<BigUint, num_bigint::ParseBigIntError> {
    let mut sum = BigUint::from(0 as u32);

    for line in arr {
        let num = line.parse::<BigUint>()?;
        sum += num;
    }

    Ok(sum)
}

fn main() -> io::Result<()> {
    let timer = std::time::Instant::now();

    let content = fs::read_to_string("data/numbers.txt")?;
    let arr: Vec<String> = content.lines().map(String::from).collect();

    let sum = large_sum(arr);

    match sum {
        Ok(sum) => {
            let answer = sum.to_string()[..10].parse::<u64>().unwrap();

            println!("\nProject Euler #13\nAnswer: {}", answer);
        }

        Err(err) => {
            eprintln!("Error parsing input: {:?}", err);
        }
    }

    let duration = timer.elapsed();
    println!("Elapsed time: {:?} milliseconds.\n", duration.as_millis());

    Ok(())
}
