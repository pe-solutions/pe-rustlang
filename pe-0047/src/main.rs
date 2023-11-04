// Distinct Primes Factors
// https://projecteuler.net/problem=47

fn omega(mut n: u64) -> u64 {
    let mut count = 0;
    let mut factor = 2;
    
    while n > 1 {
        if n % factor == 0 {
            count += 1;

            while n % factor == 0 {
                n /= factor;
            }
        }

        factor += 1;
    }
    
    count
}

fn find_first_number() -> u64 {
    let mut n2 = 1;

    while omega(n2) < 2 || omega(n2 + 1) < 2 {
        n2 += 1;
    }
    
    let mut n3 = n2 + 1;

    while omega(n3) < 3 || omega(n3 + 1) < 3 || omega(n3 + 2) < 3 {
        n3 += 1;
    }
    
    let mut n4 = n3 + 2;

    while omega(n4) < 4 || omega(n4 + 1) < 4 || omega(n4 + 2) < 4 || omega(n4 + 3) < 4 {
        n4 += 1;
    }
    
    n4
}

fn main() {
    let start = std::time::Instant::now();
    
    let answer = find_first_number();

    let duration = start.elapsed();
 
    println!("\nProject Euler #48\nAnswer:{}", answer);
    println!("Elapsed time: {} milliseconds.", duration.as_millis());
}
