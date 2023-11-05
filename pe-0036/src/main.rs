// Double-base Palindromes
// https://projecteuler.net/problem=36


fn is_palindrome(s: &str) -> bool {
    let s_rev: String = s.chars().rev().collect();
    s == s_rev
}

fn main() {
    let start = std::time::Instant::now();
    
    let limit = 1_000_000;
    let mut answer = 0;

    for i in 1..limit {
        let decimal_str = i.to_string();
        let binary_str = format!("{:b}", i);

        if is_palindrome(&decimal_str) && is_palindrome(&binary_str) {
            answer += i;
        }
    }

    let duration = start.elapsed();
    println!("\nProject Euler #36\nAnswer: {}", answer);
    println!("Elapsed time: {} milliseconds.\n", duration.as_millis());

    // println!("The sum of palindromic numbers in both base 10 and base 2 below 1,000,000 is: {}", sum);
}


