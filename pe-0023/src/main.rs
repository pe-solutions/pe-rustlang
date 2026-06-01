// Non-abundant sums
// https://projecteuler.net/problem=23

 use std::f64;
 
 fn sigma(n: i32) -> i32 {
     fn floor_sqrt(n: i32) -> i32 {
         (n as f64).sqrt() as i32
     }
 
     let mut sum = 1;
 
     for i in 2..=floor_sqrt(n) {
         if n % i == 0 {
             sum += i;

             if i != n / i {
                 sum += n / i;
             }
         }
     }
 
     sum
 }
 
 fn can_be_sum_of_two_ab_numb(ab: &Vec<i32>, n: i32) -> bool {
     let depth = (ab.len() as f64).log(2.0).ceil() as i32 + 1;
 
     for k in ab.iter() {
         if *k > n {
             break;
         }
 
         let s = n - k;
         let mut ll = 0;
         let mut rr = ab.len() as i32 - 1;
 
         for _ in 0..depth {
             let t = (ll + rr) / 2;

             if ab[t as usize] == s {
                 return true;
             } else if s < ab[t as usize] {
                 rr = t;
             } else {
                 ll = t;
             }
         }
     }
 
     false
 }
 
fn solve() -> i32 {
    let upper_limit = 28_123;
    let ab: Vec<i32> = (1..=upper_limit).filter(|&a| sigma(a) > a).collect();
    let mut answer = 0;
    for n in 1..upper_limit {
        if !can_be_sum_of_two_ab_numb(&ab, n) {
            answer += n;
        }
    }
    answer
}

pe_utils::pe_main!();
