// Combinatoric Selections
// https://projecteuler.net/problem=53

fn main() {
    let start = std::time::Instant::now();
    
    //
    
    const THRESHOLD: u128 = 1_000_000;
    
    let count = (1u32..=100)
        .filter_map(|n| {
            let mut binom = 1u128;
            
            (0..=n / 2)
                .find_map(|r| {
                    if binom > THRESHOLD {
                        // Calculate the count of valid coefficients
                        let valid_count = if n % 2 == 0 && r == n / 2 {
                            1  // Middle term when n is even
                        } else {
                            n - 2 * r + 1  // All terms from r to n-r
                        };
                        
                        Some(valid_count)
                    } else {
                        // Calculate next binomial coefficient: C(n, r+1) = C(n, r) * (n - r) / (r + 1)
                        binom = binom * (n - r) as u128 / (r + 1) as u128;
                        
                        None
                    }
                })
        })
        .sum::<u32>();
        
    //
        
    let duration = start.elapsed();
    
    println!("\nProject Euler #53\nAnswer: {}", count);
    println!("Elapsed time: {:?} milliseconds.\n", duration.as_millis());
}