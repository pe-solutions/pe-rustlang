// Digit Factorial Chains
// https://projecteuler.net/problem=74

fn digit_factorial_sum(n: u64) -> u64 {
    let fact = [1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880];
    let mut sum = 0;
    let mut m = n;
    while m > 0 {
        sum += fact[(m % 10) as usize];
        m /= 10;
    }
    sum
}

fn solve() -> u64 {
    let mut count = 0;
    for n in 1..1_000_000 {
        let mut seen = std::collections::HashSet::new();
        let mut m = n;
        while !seen.contains(&m) && m != 169 {
            seen.insert(m);
            m = digit_factorial_sum(m);
        }
        if seen.len() == 60 {
            count += 1;
        }
    }
    count
}

pe_utils::pe_main!();
