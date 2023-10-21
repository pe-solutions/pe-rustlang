// Longest Collatz Sequence
// https://projecteuler.net/problem=14

fn collatz(mut n: u64) -> u64 {
    let mut count = 1;
    
    while n > 1 {
        if n % 2 == 0 {
            n /= 2;
        } else {
            n = 3 * n + 1;
        }
        
        count += 1;
    }
    
    count
}

fn find_max_start() -> u64 {
    let mut max_count = 0;
    let mut max_start = 0;

    for i in 1..1_000_000 {
        let count = collatz(i);

        if count > max_count {
            max_count = count;
            max_start = i;
        }
    }

    max_start
}

fn main() {
    let start = std::time::Instant::now();

    let answer = find_max_start();

    let duration = start.elapsed();

    println!("\nProject Euler #14\nAnswer: {}", answer);
    println!("Elapsed time: {:?} milliseconds.\n", duration.as_millis());
}
