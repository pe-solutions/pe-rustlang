// Sum Square Difference
// https://projecteuler.net/problem/6


fn sum_square_difference(n: i64) -> i64 {
    let sum_of_squares: i64 = (1..=n).map(|x| x.pow(2)).sum();
    let square_of_sums: i64 = (1..=n).sum::<i64>().pow(2);
    
    square_of_sums - sum_of_squares
}

fn solve() -> i64 {
    sum_square_difference(100)
}

pe_utils::pe_main!();
