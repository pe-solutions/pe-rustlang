// Multiples of 3 or 5
// https://projecteuler.net/problem=1


fn multiples(factors: &[i64], limit: i64) ->i64 {
    (1..limit)
        .filter(|&n| factors.iter().any(|&m| n % m == 0))
        .sum()
}

fn main() {
    let start = std::time::Instant::now();
    //
    let answer = multiples(&[3, 5], 1000);
    //
    let duration = start.elapsed();
    
    println!("\nProject Euler #1\nAnswer: {}", answer);
    println!("Elapsed time: {:?} milliseconds.\n", duration.as_millis());
}
