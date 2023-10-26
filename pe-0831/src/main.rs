// Triple Product
// https://projecteuler.net/problem=831

use num_bigint::BigInt;
use num_traits::{Num, Pow};

fn g_m(m: BigInt) -> BigInt {
    let numerator = BigInt::from(81) * m.clone().pow(5u32) + BigInt::from(153*5) * m.pow(4u32);
    let result = numerator / BigInt::from(40);
    
    result
}

fn pe0831() -> BigInt {
    let g_m_result = g_m(BigInt::from(142857));

    let g_str7 = g_m_result.to_str_radix(7);

    let g_digits = &g_str7.chars().collect::<Vec<char>>()[0..10];
    let g_string = g_digits.iter().collect::<String>();

    let result = BigInt::from_str_radix(&g_string, 10).unwrap();

    result
}

fn main() {
    let start = std::time::Instant::now();

    let answer = pe0831();

    println!("\nProject Euler #831\nAnswer: {}", answer);

    let duration = start.elapsed();

    println!("Elapsed time: {} milliseconds.\n", duration.as_millis());
}
