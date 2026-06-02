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
 
use pe_lib::is_prime;
 
 fn is_composite(num: i32) -> bool {
     if num <= 1 {
         return true;
     }
     !is_prime(num as u64)
 }
 
 fn is_goldbach_busted(num: i32) -> bool {
     let mut busted = true;
 
     for s in 1..=sqrtint(num / 2) {
         if is_prime((num - 2 * s * s) as u64) {
             busted = false;
             break;
         }
     }
 
     busted
 }
 
fn solve() -> i32 {
    (35..6_000).step_by(2)
        .filter(|&x| is_composite(x) && is_goldbach_busted(x))
        .min()
        .expect("no solution found")
}

pe_utils::pe_main!();
