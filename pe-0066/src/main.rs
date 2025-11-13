// Diophantine Equation
// https://projecteuler.net/problem=66

use num_bigint::BigInt;
use num_traits::{One, Zero};

fn is_square(n: i32) -> bool {
    let root = (n as f64).sqrt() as i32;
    
    root * root == n
}

fn find_minimal_solution(d: i32) -> (BigInt, BigInt) {
    let d_big = BigInt::from(d);
    let a0_big = BigInt::from((d as f64).sqrt() as i64);

    if &a0_big * &a0_big == d_big {
        return (BigInt::zero(), BigInt::zero());
    }

    if &a0_big * &a0_big - &d_big == BigInt::one() {
        return (a0_big, BigInt::one());
    }

    let mut m = BigInt::zero();
    let mut d_val = BigInt::one();
    let mut a = a0_big.clone();

    let mut h0 = BigInt::one();
    let mut h1 = a0_big.clone();
    let mut k0 = BigInt::zero();
    let mut k1 = BigInt::one();

    loop {
        m = &d_val * &a - &m;
        d_val = (&d_big - &m * &m) / &d_val;
        a = (&a0_big + &m) / &d_val;

        let h2 = &a * &h1 + &h0;
        let k2 = &a * &k1 + &k0;

        let lhs = &h2 * &h2;
        let rhs = &d_big * &k2 * &k2;
        
        if &lhs - &rhs == BigInt::one() {
            return (h2, k2);
        }

        h0 = h1;
        h1 = h2;
        k0 = k1;
        k1 = k2;
    }
}

fn main() {
    let start = std::time::Instant::now();
    //
    
    let mut max_x = BigInt::zero();
    let mut max_d = 0;

    for d in 2..=1000 {
        if is_square(d) {
            continue;
        }
        
        let (x, _y) = find_minimal_solution(d);
        
        if x > max_x {
            max_x = x;
            max_d = d;
        }
    }
    
    //
    let duration = start.elapsed();
    
    println!("\nProject Euler #66\nAnswer: {}", max_d);
    println!("Elapsed time: {:?} milliseconds.\n", duration.as_millis());

}