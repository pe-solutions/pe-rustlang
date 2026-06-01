// Double-base Palindromes
// https://projecteuler.net/problem=36


fn is_palindrome(s: &str) -> bool {
    let s_rev: String = s.chars().rev().collect();
    s == s_rev
}

fn solve() -> i32 {
    let mut answer = 0;
    for i in 1..1_000_000 {
        let decimal_str = i.to_string();
        let binary_str = format!("{:b}", i);
        if is_palindrome(&decimal_str) && is_palindrome(&binary_str) {
            answer += i;
        }
    }
    answer
}

pe_utils::pe_main!();
