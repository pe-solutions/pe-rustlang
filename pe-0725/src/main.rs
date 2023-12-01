// Digit Sum Numbers
// https://projecteuler.net/problem=725

// Using Closed formula (cf. Pari-GP solution): 
//     [t1]$S(n) = \left(2\binom{n+8}{8} - \binom{10}{2}\right) \cdot (n-1) \cdot \frac{10^n-1}{9}$

use num_bigint::BigUint;
use num_traits::{Pow, One};

fn binomial(n: u32, k: u32) -> BigUint {
    let mut res: BigUint = BigUint::one();

    for i in 0..k {
        let n_minus_i: BigUint = (n - i).into();
        let i_plus_1: BigUint = (i + 1).into();

        res = &res * &n_minus_i / i_plus_1;
    }

    res
}

fn s(n: u32) -> BigUint {
    let binomial1 = binomial(n + 8, 8);
    let binomial2 = binomial(10, 2);

    let term1 = BigUint::from(2u32) * binomial1 - binomial2;
    let term2 = BigUint::from(n - 1);
    let term3 = BigUint::from(10u32).pow(n) - BigUint::from(1u32);

    &term1 * &term2 * term3 / BigUint::from(9u32)
}

fn calculate_digit_sum_modulo() -> BigUint {
    let s_2020 = s(2020);
    s_2020 % BigUint::from(10u32).pow(16u32)
}

fn main() {
    let start = std::time::Instant::now();

    let answer = calculate_digit_sum_modulo();

    let duration = start.elapsed();

    println!("\nProject Euler #725\nAnswer: {}", answer);
    println!("Elapsed time: {} milliseconds.\n", duration.as_millis());
}
