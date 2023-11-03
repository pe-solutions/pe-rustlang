// Nth Digit of Reciprocals
// https://projecteuler.net/problem=820

fn modular_exp(n: u64, p: u64, modulo: u64) -> u64 {
    if p == 0 {
        return 1;
    }

    if p % 2 == 1 {
        return (modular_exp(n, p - 1, modulo) * n) % modulo;
    }

    let tmp = modular_exp(n, p / 2, modulo) % modulo;

    (tmp * tmp) % modulo
}

fn main() {
    let start = std::time::Instant::now();
    
    let n: u64 = 10_000_000;
    let mut answer: u64 = 0;

    for i in 1..=n {
        answer += modular_exp(10, n - 1, i) * 10 / i;
    }

    let duration = start.elapsed();

    println!("\nProject Euler #820\nAnswer: {}", answer);
    println!("Elapsed time: {} milliseconds.\n", duration.as_millis()); 
}
