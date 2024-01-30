// Even Fibonacci Numbers
// https://projecteuler.net/problem=2

struct EvenFibonacci {
    a: u64,
    b: u64,
}

impl EvenFibonacci {
    fn new() -> Self {
        EvenFibonacci { a: 2, b: 8 }
    }
}

impl Iterator for EvenFibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let next = 4 * self.b + self.a;

        self.a = self.b;
        self.b = next;
        
        Some(next)
    }
}

fn calculate_sum_of_even_fibo(limit: u64) -> u64 {
    let even_fibonacci = EvenFibonacci::new();

    even_fibonacci.take_while(|&x| x < limit).sum()
}

fn main() {
    let start = std::time::Instant::now();

    const UPPER_LIMIT: u64 = 4_000_000;
    let answer = calculate_sum_of_even_fibo(UPPER_LIMIT);

    let duration = start.elapsed();

    println!("\nProject Euler #2\nAnswer: {}", answer);
    println!("Elapsed time: {:?} milliseconds.\n", duration.as_millis());
}
