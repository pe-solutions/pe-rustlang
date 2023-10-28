// Number spiral diagonals
// https://projecteuler.net/problem=28


fn main() {
    let start = std::time::Instant::now();
    
    let mut answer = 1;

    answer += (3..=1_001)
        .step_by(2)
        .map(|a:i32| 4 * a.pow(2) - 6 * a + 6)
        .sum::<i32>();
    
    let duration = start.elapsed();

    println!("\nProject Euler #28\nAnswer: {}", answer);
    println!("Elapsed time: {} milliseconds.\n", duration.as_millis());
}
