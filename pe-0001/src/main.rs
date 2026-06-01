// Multiples of 3 or 5
// https://projecteuler.net/problem=1


fn multiples(factors: &[i64], limit: i64) ->i64 {
    (1..limit)
        .filter(|&n| factors.iter().any(|&m| n % m == 0))
        .sum()
}

fn solve() -> i64 {
    multiples(&[3, 5], 1000)
}

fn main() {
    pe_utils::run(1, solve);
}
