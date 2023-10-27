// Amicable Numbers
// https://projecteuler.net/problem=21

 fn sum_of_proper_divisors(n: i32) -> i32 {
     (1..=(n + 1) / 2)
         .filter(|x| n % x == 0 && n != *x)
         .sum()
 }
 
 fn main() {
     let start = std::time::Instant::now();
 
     const RANGE_MAX: i32 = 10_000;
 
     let mut answer = 0;
 
     for n in 1..RANGE_MAX {
         let sum_n = sum_of_proper_divisors(n);
         
         if n < sum_n && sum_n <= RANGE_MAX && sum_of_proper_divisors(sum_n) == n {
             answer += n + sum_n;
         }
     }
 
     let duration = start.elapsed();
    
     println!("\nProject Euler #21\nAnswer: {}", answer);
     println!("Elapsed time: {} milliseconds.\n", duration.as_millis());
  }
 