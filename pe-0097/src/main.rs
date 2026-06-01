// Large Non-Mersenne Prime
// https://projecteuler.net/problem=97
 
fn solve() -> i64 {
    let mut answer: i64 = 1;
    let m: i64 = 10_000_000_000;
    for _i in 1..=7_830_457 {
        answer = (2 * answer) % m;
    }
    answer = (answer * 28_433) % m;
    answer = (answer + 1) % m;
    answer
}

pe_utils::pe_main!();
