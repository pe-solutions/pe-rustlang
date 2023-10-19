// https://projecteuler.net/problem=2

use std::time::Instant;

fn main() {
    let start = Instant::now();
    //
    let mut answer: u64 = 0;

    let mut a: u64 = 1;
    let mut b: u64 = 1;

    while b < 4_000_000 {
        if b % 2 == 0 {
            answer += b;
        }

        let next = a + b;
        
        a = b;
        b = next;
    }
    //
    let duration = start.elapsed();

    println!("\nProject Euler #2\nAnswer: {}", answer);
    println!("Elapsed time: {:?} milliseconds.\n", duration.as_millis());
}
