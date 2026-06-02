// Non-abundant sums
// https://projecteuler.net/problem=23

use pe_lib::sum_proper_divisors;

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
    let ab: Vec<i32> = (1..=upper_limit).filter(|&a| sum_proper_divisors(a as u64) > a as u64).collect();
    let mut answer = 0;
    for n in 1..upper_limit {
        if !can_be_sum_of_two_ab_numb(&ab, n) {
            answer += n;
        }
    }
    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abundant_12() {
        // 12 is the smallest abundant number (divisors: 1,2,3,4,6 sum to 16 > 12)
        let sum = sum_proper_divisors(12);
        assert!(sum > 12);
    }

    #[test]
    fn test_sum_two_abundant_small() {
        let ab: Vec<i32> = (1..=100).filter(|&a| sum_proper_divisors(a as u64) > a as u64).collect();
        // Test that binary search finds sums correctly
        assert!(ab.len() > 0);
        // 24 = 12 + 12, both abundant
        assert!(can_be_sum_of_two_ab_numb(&ab, 24));
    }

    #[test]
    fn test_non_abundant_sum_behavior() {
        // Small non-abundant numbers shouldn't be expressible
        let ab: Vec<i32> = (1..=100).filter(|&a| sum_proper_divisors(a as u64) > a as u64).collect();
        let small = !can_be_sum_of_two_ab_numb(&ab, 5);
        assert!(small); // 5 is not sum of abundant numbers
    }

    #[test]
    fn test_solve_produces_positive() {
        let result = solve();
        assert!(result > 0);
    }
}

pe_utils::pe_main!();
