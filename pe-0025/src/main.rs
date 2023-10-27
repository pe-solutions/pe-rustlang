// 1000-digit Fibonacci number
// Link: https://projecteuler.net/problem=25
 
use num_bigint::BigUint;
use num_traits::One;

 fn main() {
     let start_time = std::time::Instant::now();
 
     let mut a = BigUint::one();
     let mut b = BigUint::one();

     let mut answer = 2;
 
     while b.to_string().len() < 1_000 {
         let temp = a + b.clone();

         a = b.clone();
         b = temp;

         answer += 1;
     }

     let duration = start_time.elapsed();
    
     println!("\nProject Euler #25\nAnswer: {}", answer);
     println!("Elapsed time: {} milliseconds.\n", duration.as_millis());
 }
 