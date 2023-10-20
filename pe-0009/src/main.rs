// Special Pythagorean Triplet
// https://projecteuler.net/problem=9

use std::time::Instant;

fn find_special_pythagorean_triplet() -> Option<u32> {
    for a in 1..=1000 {
        for b in a + 1..=1000 {
            let c = 1000 - a - b;

            if a * a + b * b == c * c {
                return Some(a * b * c); // a * b * c;
            } else if a * a + b * b > c * c {
                break;
            }
        }
    }

    None
}

fn main() {
    let start = Instant::now();
    //
    
    match find_special_pythagorean_triplet() {
        Some(answer) => {
            let duration = start.elapsed();
            
            println!("\nProject Euler #9\nAnswer: {}", answer);
            println!("Elapsed time: {:?} milliseconds.\n", duration.as_millis()); 
        }
        None => {
            let duration = start.elapsed();

            println!("No special Pythagorean triplet found.");
            println!("Elapsed time: {:?} milliseconds.\n", duration.as_millis()); 
        }
    }
    
}
