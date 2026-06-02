// Reversible Prime Squares
// https://projecteuler.net/problem=808

use pe_lib::is_prime;
 
 fn reverse(n: u64) -> u64 {
     let mut r = 0;
     let mut n = n;
     while n > 0 {
         r *= 10;
         r += n % 10;
         n /= 10;
     }
     r
 }
 
 fn sum_of_rev_prime(mut c: u32) -> u64 {
     let mut s;
     let mut rev;
     let mut z = 0;
     let mut p10 = 10;
     let mut p100 = 100;
     let mut q;
     let mut i = 0;
     let mut r;
     let mut n = 13;
 
     while c > 0 {
         if n > p100 {
             p10 *= 10;
             p100 *= 10;
         }
 
         q = n / p10;
         if q == 1 || q == 3 {
             if is_prime(n) {
                 s = (n as u64) * (n as u64);
                 rev = reverse(s);
 
                 if rev != s {
                     r = (rev as f64).sqrt() as u64;
 
                     if r * r == rev && r % 2 > 0 && r % 3 > 0 && is_prime(r) {
                         c -= 1;
                         z += s;
                     }
                 }
             }
         }
 
         n += 0x2A8A >> ((i & 3) << 2) & 15;
         i += 1;
     }
 
     z
 }
 
fn solve() -> u64 {
    sum_of_rev_prime(50)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_basic() {
        assert_eq!(reverse(123), 321);
        assert_eq!(reverse(1000), 1);
        assert_eq!(reverse(121), 121);
    }

    #[test]
    fn test_reverse_single_digit() {
        assert_eq!(reverse(5), 5);
        assert_eq!(reverse(9), 9);
    }

    #[test]
    fn test_reverse_property() {
        let num = 12345;
        let rev = reverse(num);
        let rev_rev = reverse(rev);
        assert_eq!(rev_rev, num);
    }

    #[test]
    fn test_is_prime_check() {
        assert!(is_prime(13));
        assert!(is_prime(17));
        assert!(!is_prime(15));
    }

    #[test]
    fn test_solve_produces_result() {
        let result = solve();
        assert!(result > 0);
    }
}

pe_utils::pe_main!();
