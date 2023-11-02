// Digit Power Sum
// https://projecteuler.net/problem=119


fn main() {
    let start = std::time::Instant::now();
    
    let mut res: Vec<u64> = Vec::new();

    for base in 2u64..=70 {
        for power in 2u32..=10 {
            if base == base.pow(power).to_string().chars()
                .map(|c| c.to_digit(10).unwrap() as u64)
                .sum() {
                res.push(base.pow(power));
            }
        }
    }

    res.sort();

    let duration = start.elapsed();

    let answer = res[30 - 1];
    println!("\nProject Euler #119\nAnswer: {}", answer);

    println!("Elapsed time: {} milliseconds.\n", duration.as_millis());
}

