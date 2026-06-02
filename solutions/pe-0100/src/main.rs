// Arranged Probability
// https://projecteuler.net/problem=100

fn solve() -> u64 {
    let mut b = 3u64;
    let mut n = 4u64;

    let limit = 10u64.pow(12);

    while n <= limit {
        if 2 * b * (b - 1) == n * (n - 1) {
            if n > limit {
                return b;
            }
        }

        let new_b = 3 * b + 2 * n - 2;
        let new_n = 4 * b + 3 * n - 3;

        b = new_b;
        n = new_n;
    }

    b
}

pe_utils::pe_main!();
