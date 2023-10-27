// Digit fifth powers
// https://projecteuler.net/problem=30


fn main() {
    let start = std::time::Instant::now();
    
    let answer: u32 = (2..10u32.pow(6))
        .filter(|&i| {
            let digits: Vec<u32> = (0..6).map(|pow| (i / 10u32.pow(pow)) % 10).collect();
            i == digits.iter().map(|&digit| digit.pow(5)).sum()
        })
        .sum();

    let duration = start.elapsed();

    println!("\nProject Euler #30\nAnswer: {}", answer);
    println!("Elapsed time: {} milliseconds.\n", duration.as_millis());}