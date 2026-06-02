pub fn sum_proper_divisors(n: u64) -> u64 {
    if n <= 1 {
        return 0;
    }
    let mut sum = 1;
    let sqrt_n = (n as f64).sqrt() as u64;
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
    let sqrt_n = (n as f64).sqrt() as u64;
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

pub fn prime_factors(mut n: u64) -> Vec<(u64, u32)> {
    let mut factors = Vec::new();
    let mut divisor = 2;
    while divisor * divisor <= n {
        let mut count = 0;
        while n % divisor == 0 {
            n /= divisor;
            count += 1;
        }
        if count > 0 {
            factors.push((divisor, count));
        }
        divisor += 1;
    }
    if n > 1 {
        factors.push((n, 1));
    }
    factors
}

pub fn largest_prime_factor(mut n: u64) -> u64 {
    let mut largest = 1u64;
    let mut i: u64 = 2;
    while i * i <= n {
        while n % i == 0 {
            largest = i;
            n /= i;
        }
        i += 1;
    }
    if n > 1 {
        largest = n;
    }
    largest
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

    // prime_factors tests
    #[test]
    fn test_prime_factors_small() {
        assert_eq!(prime_factors(2), vec![(2, 1)]);
        assert_eq!(prime_factors(6), vec![(2, 1), (3, 1)]);
        assert_eq!(prime_factors(12), vec![(2, 2), (3, 1)]);
    }

    #[test]
    fn test_prime_factors_powers() {
        assert_eq!(prime_factors(8), vec![(2, 3)]);
        assert_eq!(prime_factors(27), vec![(3, 3)]);
        assert_eq!(prime_factors(32), vec![(2, 5)]);
    }

    #[test]
    fn test_prime_factors_composite() {
        assert_eq!(prime_factors(30), vec![(2, 1), (3, 1), (5, 1)]);
        assert_eq!(prime_factors(60), vec![(2, 2), (3, 1), (5, 1)]);
    }

    #[test]
    fn test_prime_factors_reconstruction() {
        // Verify that multiplying prime factors reconstructs the number
        for n in [6, 12, 30, 60, 100, 120] {
            let factors = prime_factors(n);
            let product: u64 = factors.iter().map(|(p, e)| p.pow(*e)).product();
            assert_eq!(product, n);
        }
    }

    // largest_prime_factor tests
    #[test]
    fn test_largest_prime_factor_prime() {
        assert_eq!(largest_prime_factor(7), 7);
        assert_eq!(largest_prime_factor(13), 13);
        assert_eq!(largest_prime_factor(17), 17);
    }

    #[test]
    fn test_largest_prime_factor_composite() {
        assert_eq!(largest_prime_factor(6), 3);
        assert_eq!(largest_prime_factor(12), 3);
        assert_eq!(largest_prime_factor(30), 5);
        assert_eq!(largest_prime_factor(15), 5);
    }

    #[test]
    fn test_largest_prime_factor_small_powers() {
        assert_eq!(largest_prime_factor(4), 2);
        assert_eq!(largest_prime_factor(8), 2);
        assert_eq!(largest_prime_factor(9), 3);
        assert_eq!(largest_prime_factor(25), 5);
    }
}
