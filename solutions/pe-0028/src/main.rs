// Number spiral diagonals
// https://projecteuler.net/problem=28

// Closed form
fn sum_of_spiral_diagonals(n: i64) -> i64 {
    (2 * n.pow(3) + n.pow(2) + 4 * n - 3) / 3
}

fn solve() -> i64 {
    sum_of_spiral_diagonals(1_001)
}

pe_utils::pe_main!();
