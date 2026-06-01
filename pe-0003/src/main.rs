// Largest Prime Factor
// https://projecteuler.net/problem/3

fn largest_prime_factor(mut n: u64) -> u64 {
    let mut i: u64 = 2;
    while i * i <= n {
        while n % i == 0 {
            n /= i;
        }
        i += 1;
    }
    n
}

fn solve() -> u64 {
    largest_prime_factor(600_851_475_143)
}

fn main() {
    pe_utils::run(3, solve);
}
