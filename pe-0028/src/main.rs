// Number spiral diagonals
// https://projecteuler.net/problem=28

// Closed form
fn sum_of_spiral_diagonals(n: i64) -> i64 {
    (2 * n.pow(3) + n.pow(2) + 4 * n - 3) / 3
}

fn main() {
    let start = std::time::Instant::now();
    
    let answer = sum_of_spiral_diagonals(1_001);
    
    let duration = start.elapsed();

    println!("\nProject Euler #28\nAnswer: {}", answer);
    println!("Elapsed time: {} milliseconds.\n", duration.as_millis());
}
