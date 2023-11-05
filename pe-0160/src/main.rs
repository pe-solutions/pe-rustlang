// Factorial Trailing Digits
// https://projecteuler.net/problem=160

fn f(n: u128) -> u128 {
    let mut result = 1u128;

    for i in 1..=n {
        result *= i;

        while result % 10 == 0 {
            result /= 10;
        }

        result %= 1_000_000_000_000;
    }

    result % 100_000
}

fn main() {
    let start = std::time::Instant::now();

    println!("\nProject Euler #160\nAnswer: {}", f(2_560_000));

    let duration = start.elapsed();
    println!("Elapsed time: {} milliseconds.", duration.as_millis());
}
