// Pandigital Prime
// https://projecteuler.net/problem=41

extern crate itertools;

use itertools::Itertools;

fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true}

fn main() {
    let start = std::time::Instant::now();
    
    let digits = vec!['7', '6', '5', '4', '3', '2', '1'];
    let permutations = digits.iter().permutations(digits.len());

    let largest_prime = permutations
        .map(|perm| {
            let perm_str: String = perm.into_iter().collect();
            let perm_num = perm_str.parse::<u64>().unwrap();
            if is_prime(perm_num) {
                Some(perm_num)
            } else {
                None
            }
        })
        .filter(|x| x.is_some())
        .max();

    match largest_prime {
        Some(prime) => {
            println!("\nProject Euler #41\nAnswer: {}", prime.unwrap());
        }
        None => {
            println!("No prime number found among permutations.");
        }
    }

    let duration = start.elapsed();

    println!("Elapsed time: {} milliseconds.\n", duration.as_millis());
}

