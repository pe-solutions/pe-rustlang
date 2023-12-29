// Self powers
// https://projecteuler.net/problem=48

use num_bigint::BigInt;
use num_traits::pow;

fn main() {
    let start = std::time::Instant::now();
    
    let answer = powmod(
        (1..=1000)
            .map(|i| BigInt::from(i).pow(i as u32))
            .sum::<BigInt>(),
        BigInt::from(1),
        pow(BigInt::from(10), 10),
    );
    
    let duration = start.elapsed();
 
    println!("\nProject Euler #48\nAnswer: {}", answer);
    println!("Elapsed time: {} milliseconds.", duration.as_millis());
}

fn powmod(base: BigInt, exponent: BigInt, modulus: BigInt) -> BigInt {
    base.modpow(&exponent, &modulus)
}
