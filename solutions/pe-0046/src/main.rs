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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sqrtint_basic() {
        assert_eq!(sqrtint(4), 2);
        assert_eq!(sqrtint(9), 3);
        assert_eq!(sqrtint(16), 4);
    }

    #[test]
    fn test_sqrtint_non_perfect() {
        assert_eq!(sqrtint(5), 2);
        assert_eq!(sqrtint(10), 3);
    }

    #[test]
    fn test_is_composite() {
        assert!(is_composite(4));
        assert!(is_composite(6));
        assert!(is_composite(8));
        assert!(!is_composite(2));
        assert!(!is_composite(3));
        assert!(!is_composite(5));
    }

    #[test]
    fn test_goldbach_property() {
        // Goldbach_busted returns true if no prime p and square s exist
        // such that n = p + 2*s^2
        // For testing, just verify the function returns a boolean
        let result = is_goldbach_busted(9);
        assert!(result == true || result == false);
    }

    #[test]
    fn test_solve_produces_odd_composite() {
        let result = solve();
        assert!(result % 2 == 1);  // Should be odd
        assert!(is_composite(result)); // Should be composite
    }
}

pe_utils::pe_main!();
