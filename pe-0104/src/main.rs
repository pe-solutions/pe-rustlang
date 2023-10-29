// Pandigital Fibonacci ends
// https://projecteuler.net/problem=104

use num::bigint::BigInt;

fn is_pandigital(s: &str) -> bool {
    if s.len() != 9 {
        return false;
    }

    let mut chars: Vec<char> = s.chars().collect();

    chars.sort();
    
    let sorted_str: String = chars.iter().collect();

    sorted_str == "123456789"
}

fn main() {
    let start = std::time::Instant::now();

    let mut fib_prev = BigInt::from(0u32);
    let mut fib_curr = BigInt::from(1u32);

    let mut index = 1u32;

    loop {
        let fib_next: BigInt = &fib_prev + &fib_curr;

        fib_prev = fib_curr.clone();
        fib_curr = fib_next.clone();

        index += 1;

        if is_pandigital(&format!("{}", fib_next.clone() % 1_000_000_000)) {
            let fib_str = format!("{}", fib_next);

            if is_pandigital(&fib_str[0..9]) {
                let answer = index;
                println!("\nProject Euler #124\nAnswer: {}", answer);

                break;
            }
        }
    }

    let duration = start.elapsed();
    
    println!("Elapsed time: {} milliseconds.\n", duration.as_millis());
}
