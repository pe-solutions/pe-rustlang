// Self powers
// https://projecteuler.net/problem=48

 use num_bigint::{BigUint, ToBigUint};
 
 fn pow(x: &BigUint, y: &BigUint) -> BigUint {
     if y == &BigUint::from(0u32) {
         return BigUint::from(1u32);
     }
     //
     if y == &BigUint::from(1u32) {
         return x.clone();
     }
     //
     if y % 2u32.to_biguint().unwrap() == BigUint::from(0u32) {
         let x_squared = x.clone() * x.clone();
         return pow(&x_squared, &(y.clone() / 2u32.to_biguint().unwrap()));
     } else {
         let x_squared = x.clone() * x.clone();
         return x.clone() * pow(&x_squared, &(&(y.clone() - 1u32.to_biguint().unwrap()) / 2u32.to_biguint().unwrap()));
     }
 }
 
 fn main() {
     let start = std::time::Instant::now();
 
     let answer = (1u32..=1_000u32)
         .map(|a| pow(&a.to_biguint().unwrap(), &a.to_biguint().unwrap()))
         .fold(BigUint::from(0u32), |acc, x| acc + x);
 
     let answer_str = answer.to_str_radix(10);
     let last_10_digits = &answer_str[answer_str.len() - 10..];

     let duration = start.elapsed();
 
     println!("\nProject Euler #48\nAnswer:{}", last_10_digits); // println!("{}", last_10_digits);
 
     println!("Elapsed time: {} milliseconds.", duration.as_millis());
 }
 