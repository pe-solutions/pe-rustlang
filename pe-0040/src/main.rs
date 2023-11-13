// Champernowne's Constant
// https://projecteuler.net/problem=40

fn main() {
    let start = std::time::Instant::now();
    
    let c: Vec<u64> = (1..=250000)
        .map(|x| x.to_string().chars().map(|c| c.to_digit(10).unwrap() as u64).collect::<Vec<_>>())
        .flatten()
        .collect();
    
    let answer = c[0] * c[9] * c[99] * c[999] * c[9999] * c[99999] * c[999999];

    let duration = start.elapsed();
    
    println!("\nProject Euler #40\nAnswer: {}", answer);
    println!("Elapsed time: {} milliseconds.\n", duration.as_millis());
}
