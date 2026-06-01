// Digit fifth powers
// https://projecteuler.net/problem=30


fn solve() -> u32 {
    (2..10u32.pow(6))
        .filter(|&i| {
            let digits: Vec<u32> = (0..6).map(|pow| (i / 10u32.pow(pow)) % 10).collect();
            i == digits.iter().map(|&digit| digit.pow(5)).sum()
        })
        .sum()
}

fn main() {
    pe_utils::run(30, solve);
}
