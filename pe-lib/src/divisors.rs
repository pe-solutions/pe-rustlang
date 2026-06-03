pub fn sum_proper_divisors(n: u64) -> u64 {
    if n <= 1 {
        return 0;
    }
    let mut sum = 1;
    let sqrt_n = crate::isqrt::isqrt(n);
    for i in 2..=sqrt_n {
        if n % i == 0 {
            sum += i;
            if i != n / i {
                sum += n / i;
            }
        }
    }
    sum
}

pub fn sum_divisors(n: u64) -> u64 {
    sum_proper_divisors(n) + n
}

pub fn count_divisors(n: u64) -> u64 {
    let mut count = 0;
    let sqrt_n = crate::isqrt::isqrt(n);
    for i in 1..=sqrt_n {
        if n % i == 0 {
            if i * i == n {
                count += 1;
            } else {
                count += 2;
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    // sum_proper_divisors tests
    #[test]
    fn test_sum_proper_divisors_edge_cases() {
        assert_eq!(sum_proper_divisors(0), 0);
        assert_eq!(sum_proper_divisors(1), 0);
        assert_eq!(sum_proper_divisors(2), 1); // Only proper divisor is 1
    }

    #[test]
    fn test_sum_proper_divisors_small() {
        assert_eq!(sum_proper_divisors(6), 6); // 1+2+3
        assert_eq!(sum_proper_divisors(12), 16); // 1+2+3+4+6
        assert_eq!(sum_proper_divisors(28), 28); // Perfect number
    }

    #[test]
    fn test_sum_proper_divisors_prime() {
        assert_eq!(sum_proper_divisors(7), 1);
        assert_eq!(sum_proper_divisors(11), 1);
        assert_eq!(sum_proper_divisors(13), 1);
    }

    #[test]
    fn test_sum_proper_divisors_power_of_prime() {
        assert_eq!(sum_proper_divisors(4), 3); // 1+2
        assert_eq!(sum_proper_divisors(8), 7); // 1+2+4
        assert_eq!(sum_proper_divisors(9), 4); // 1+3
    }

    // sum_divisors tests
    #[test]
    fn test_sum_divisors() {
        assert_eq!(sum_divisors(6), 12); // 1+2+3+6
        assert_eq!(sum_divisors(12), 28); // 1+2+3+4+6+12
        assert_eq!(sum_divisors(1), 1);
    }

    // count_divisors tests
    #[test]
    fn test_count_divisors_small() {
        assert_eq!(count_divisors(1), 1);
        assert_eq!(count_divisors(2), 2);
        assert_eq!(count_divisors(6), 4);
        assert_eq!(count_divisors(12), 6);
    }

    #[test]
    fn test_count_divisors_perfect_square() {
        assert_eq!(count_divisors(4), 3); // 1,2,4
        assert_eq!(count_divisors(9), 3); // 1,3,9
        assert_eq!(count_divisors(16), 5); // 1,2,4,8,16
    }

    #[test]
    fn test_count_divisors_prime_powers() {
        assert_eq!(count_divisors(2), 2); // p: 2 divisors
        assert_eq!(count_divisors(4), 3); // p^2: 3 divisors
        assert_eq!(count_divisors(8), 4); // p^3: 4 divisors
    }
}
