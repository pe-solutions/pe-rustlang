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
    let mut i: u64 = 2;
    while i * i <= n {
        while n % i == 0 {
            n /= i;
        }
        i += 1;
    }
    n
}
