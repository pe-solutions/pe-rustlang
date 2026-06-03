use num_bigint::BigUint;
use num_traits::One;

pub fn factorial(n: u64) -> BigUint {
    (1..=n).fold(BigUint::one(), |a, b| a * b)
}

pub fn binomial_big(n: u32, k: u32) -> BigUint {
    if k > n {
        return BigUint::from(0u32);
    }
    if k == 0 || k == n {
        return BigUint::one();
    }

    let mut res = BigUint::one();
    for i in 0..k {
        res = &res * &BigUint::from(n - i);
        res = res / BigUint::from(i + 1);
    }
    res
}

pub fn count_partitions(n: usize) -> usize {
    let mut partitions = vec![0usize; n + 1];
    partitions[0] = 1;
    for num in 1..=n {
        for i in num..=n {
            partitions[i] += partitions[i - num];
        }
    }
    partitions[n]
}

pub fn combinations<T: Clone>(items: &[T], k: usize) -> Vec<Vec<T>> {
    let mut result = Vec::new();
    let mut combo = Vec::new();
    backtrack_combinations(0, items, k, &mut combo, &mut result);
    result
}

fn backtrack_combinations<T: Clone>(
    start: usize,
    items: &[T],
    k: usize,
    combo: &mut Vec<T>,
    result: &mut Vec<Vec<T>>,
) {
    if combo.len() == k {
        result.push(combo.clone());
        return;
    }
    for i in start..items.len() {
        combo.push(items[i].clone());
        backtrack_combinations(i + 1, items, k, combo, result);
        combo.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Factorial tests
    #[test]
    fn test_factorial_zero() {
        assert_eq!(factorial(0), BigUint::one());
    }

    #[test]
    fn test_factorial_one() {
        assert_eq!(factorial(1), BigUint::one());
    }

    #[test]
    fn test_factorial_small() {
        assert_eq!(factorial(5), BigUint::from(120u32));
        assert_eq!(factorial(10), BigUint::from(3_628_800u32));
    }

    #[test]
    fn test_factorial_larger() {
        // 20! = 2,432,902,008,176,640,000
        assert_eq!(factorial(20), BigUint::from(2_432_902_008_176_640_000u64));
    }

    #[test]
    fn test_factorial_growth() {
        let f5 = factorial(5);
        let f6 = factorial(6);
        assert_eq!(&f6 / &f5, BigUint::from(6u32));
    }

    // Binomial tests
    #[test]
    fn test_binomial_edge_cases() {
        assert_eq!(binomial_big(0, 0), BigUint::one());
        assert_eq!(binomial_big(5, 0), BigUint::one());
        assert_eq!(binomial_big(5, 5), BigUint::one());
    }

    #[test]
    fn test_binomial_k_greater_than_n() {
        assert_eq!(binomial_big(5, 10), BigUint::from(0u32));
    }

    #[test]
    fn test_binomial_small_values() {
        assert_eq!(binomial_big(5, 1), BigUint::from(5u32));
        assert_eq!(binomial_big(5, 2), BigUint::from(10u32));
        assert_eq!(binomial_big(6, 3), BigUint::from(20u32));
    }

    #[test]
    fn test_binomial_symmetry() {
        // C(n, k) = C(n, n-k)
        assert_eq!(binomial_big(10, 3), binomial_big(10, 7));
        assert_eq!(binomial_big(20, 5), binomial_big(20, 15));
    }

    #[test]
    fn test_binomial_pascal_triangle() {
        // C(n, k) + C(n, k+1) = C(n+1, k+1)
        let left = binomial_big(5, 2) + binomial_big(5, 3);
        let right = binomial_big(6, 3);
        assert_eq!(left, right);
    }

    // Partition tests
    #[test]
    fn test_partitions_edge_cases() {
        assert_eq!(count_partitions(0), 1);
        assert_eq!(count_partitions(1), 1);
        assert_eq!(count_partitions(2), 2);
    }

    #[test]
    fn test_partitions_known_values() {
        // OEIS A000041: 1, 1, 2, 3, 5, 7, 11, 15, 22, 30, ...
        assert_eq!(count_partitions(3), 3);
        assert_eq!(count_partitions(4), 5);
        assert_eq!(count_partitions(5), 7);
        assert_eq!(count_partitions(10), 42);
    }

    #[test]
    fn test_partitions_growth() {
        // Partition count should generally increase
        for n in 1..20 {
            assert!(count_partitions(n) < count_partitions(n + 1));
        }
    }

    #[test]
    fn test_combinations_basic() {
        let items = vec![1, 2, 3];
        let combos = combinations(&items, 2);
        assert_eq!(combos.len(), 3); // C(3,2) = 3
        assert!(combos.contains(&vec![1, 2]));
        assert!(combos.contains(&vec![1, 3]));
        assert!(combos.contains(&vec![2, 3]));
    }

    #[test]
    fn test_combinations_k_equals_n() {
        let items = vec![1, 2, 3];
        let combos = combinations(&items, 3);
        assert_eq!(combos.len(), 1);
        assert_eq!(combos[0], vec![1, 2, 3]);
    }

    #[test]
    fn test_combinations_k_equals_1() {
        let items = vec![1, 2, 3];
        let combos = combinations(&items, 1);
        assert_eq!(combos.len(), 3);
        assert!(combos.contains(&vec![1]));
        assert!(combos.contains(&vec![2]));
        assert!(combos.contains(&vec![3]));
    }

    #[test]
    fn test_combinations_generic_types() {
        let items = vec!['a', 'b', 'c', 'd'];
        let combos = combinations(&items, 2);
        assert_eq!(combos.len(), 6); // C(4,2) = 6
        assert!(combos.contains(&vec!['a', 'b']));
        assert!(combos.contains(&vec!['c', 'd']));
    }
}
