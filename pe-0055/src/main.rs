// Lychrel Numbers
// https://projecteuler.net/problem=55


fn reverse_digits(mut number: u128) -> u128 {
    let mut reversed = 0;
    
    while number > 0 {
        reversed = reversed * 10 + (number % 10);
        number /= 10;
    }
    
    reversed
}

fn is_palindrome(number: u128) -> bool {
    number == reverse_digits(number)
}

fn is_lychrel(mut number: u128) -> bool {
    for _ in 0..50 {
        number += reverse_digits(number);
        
        if is_palindrome(number) {
            return false;
        }
    }
    
    true
}

fn solve() -> u32 {
    let mut lychrel_total = 0;
    for candidate in 1..=10_000 {
        if is_lychrel(candidate as u128) { lychrel_total += 1; }
    }
    lychrel_total
}

pe_utils::pe_main!();
