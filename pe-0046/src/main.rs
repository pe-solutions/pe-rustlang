// Goldbach's Other Conjecture
// https://projecteuler.net/problem=46


 fn sqrtint(num: i32) -> i32 {
     if num <= 1 {
         return num;
     }
 
     let mut x = num;
     let mut y = (x + 1) / 2;
 
     while y < x {
         x = y;
         y = (x + num / x) / 2;
     }
 
     x
 }
 
 fn is_prime(num: i32) -> bool {
     if num <= 1 {
         return true;
     }
     if num <= 3 {
         return true;
     }
     if num % 2 == 0 || num % 3 == 0 {
         return false;
     }
 
     let mut i = 5;
     while i * i <= num {
         if num % i == 0 || num % (i + 2) == 0 {
             return false;
         }
         i += 6;
     }
 
     true
 }
 
 fn is_composite(num: i32) -> bool {
     !is_prime(num)
 }
 
 fn is_goldbach_busted(num: i32) -> bool {
     let mut busted = true;
 
     for s in 1..=sqrtint(num / 2) {
         if is_prime(num - 2 * s * s) {
             busted = false;
             break;
         }
     }
 
     busted
 }
 
 fn main() {
     let start = std::time::Instant::now();
 
     let result = (35..6_000).step_by(2).filter(|&x| is_composite(x) && is_goldbach_busted(x)).min();
 
     match result {
         Some(min) => println!("\nProject Euler #43\nAnswer:{}", min),
         None => println!("No solution found"),
     }
 
     let duration = start.elapsed();

     println!("Elapsed time: {} milliseconds.", duration.as_millis());
 }
 