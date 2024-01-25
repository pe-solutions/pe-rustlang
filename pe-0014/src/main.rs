// Longest Collatz Sequence
// https://projecteuler.net/problem=14

fn collatz_length(mut n: u64) -> u64 {
    let mut count = 1;
    while n > 1 {
        count += 1;
        n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
    }
    count
}

fn find_max_start(limit: u64) -> u64 {
    let mut max_count = 0;
    let mut max_start = 0;
    for i in 1..limit {
        let count = collatz_length(i);
        if count > max_count {
            max_count = count;
            max_start = i;
        }
    }
    max_start
}

fn main() {
    const LIMIT: u64 = 1_000_000;
    
    let start = std::time::Instant::now();
    let answer = find_max_start(LIMIT);
    let duration = start.elapsed();

    println!("\nProject Euler #14\nAnswer: {}", answer);
    println!("Elapsed time: {:?} milliseconds.\n", duration.as_millis());
}
