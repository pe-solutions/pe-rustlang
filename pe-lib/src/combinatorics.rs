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
