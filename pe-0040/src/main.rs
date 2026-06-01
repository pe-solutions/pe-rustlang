// Champernowne's Constant
// https://projecteuler.net/problem=40

fn solve() -> u64 {
    let c: Vec<u64> = (1..=250000)
        .flat_map(|x: u64| x.to_string().chars().map(|c| c.to_digit(10).unwrap() as u64).collect::<Vec<_>>())
        .collect();
    c[0] * c[9] * c[99] * c[999] * c[9999] * c[99999] * c[999999]
}

fn main() {
    pe_utils::run(40, solve);
}
