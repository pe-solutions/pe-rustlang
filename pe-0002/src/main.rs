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
        let next_value = self.a;
        let new_b = self.a + 4*self.b;
        self.a = self.b;
        self.b = new_b;
        Some(next_value)
    }
}

fn sum_of_even_fibo(limit: u64) -> u64 {
    EvenFibonacci::new().take_while(|&x| x < limit).sum()
}

fn main() {
    let start = std::time::Instant::now();    

    // Calculate and Print the sum
    println!("\nProject Euler #2\nAnswer: {}", sum_of_even_fibo(4_000_000));

    let duration = start.elapsed();

    // Print the Elapsed time    
    println!("Elapsed time: {:?} milliseconds.\n", duration.as_millis());
}
