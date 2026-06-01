// Nth Digit of Reciprocals
// https://projecteuler.net/problem=820

fn modular_exp(n: u64, p: u64, modulo: u64) -> u64 {
    if p == 0 {
        return 1;
    }

    if p % 2 == 1 {
        return (modular_exp(n, p - 1, modulo) * n) % modulo;
    }

    let tmp = modular_exp(n, p / 2, modulo) % modulo;
    (tmp * tmp) % modulo
}

fn calculate_nth_digit_sum(n: u64) -> u64 {
    (1..=n)
        .map(|i| modular_exp(10, n - 1, i) * 10 / i)
        .sum()
}

fn solve() -> u64 {
    calculate_nth_digit_sum(10_000_000)
}

pe_utils::pe_main!();
