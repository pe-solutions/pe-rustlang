// Largest Prime Factor
// https://projecteuler.net/problem=3

fn main() {
    let start = std::time::Instant::now();
    
    let mut answer: u64 = 600_851_475_143;

    let mut i: u64 = 2;
    while i * i < answer {
        while answer % i == 0 {
            answer /= i;
        }

        i += 1;
    }

    let duration = start.elapsed();

    println!("\nProject Euler #3\nAnswer: {}", answer);
    println!("Elapsed time: {:?} milliseconds.\n", duration.as_millis());
}
