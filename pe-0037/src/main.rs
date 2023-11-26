// Truncatable Primes
// https://projecteuler.net/problem=37

fn is_prime(t: usize) -> bool {
    if t < 2 {
        return false;
    }

    (2..=((t as f64).sqrt() as usize)).all(|i| t % i != 0)
}


fn is_truncatable(t: usize) -> bool {
    let num_str = t.to_string();

    (0..(num_str.len() - 1))
        .all(|i| {
            let left = &num_str[..=i];
            let right = &num_str[i + 1..];
            is_prime(left.parse().unwrap()) && is_prime(right.parse().unwrap())
        })
}

fn main() {
    let start = std::time::Instant::now();
    
    let answer = (10..)
        .filter(|&i| is_prime(i) && is_truncatable(i))
        .take(11)
        .fold(0, |acc, x| acc + x);
    
    let duration = start.elapsed();
    
    println!("\nProject Euler #37\nAnswer: {}", answer);
    println!("Elapsed time: {} milliseconds.\n", duration.as_millis());
}
