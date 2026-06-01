// 1000-digit Fibonacci number
// Link: https://projecteuler.net/problem=25
 
use num_bigint::BigUint;
use num_traits::One;

fn solve() -> u32 {
    let mut a = BigUint::one();
    let mut b = BigUint::one();
    let mut answer = 2;
    while b.to_string().len() < 1_000 {
        let temp = a + b.clone();
        a = b.clone();
        b = temp;
        answer += 1;
    }
    answer
}

fn main() {
    pe_utils::run(25, solve);
}
