pub fn digit_sum(n: u64) -> u64 {
    let mut sum = 0;
    let mut n = n;
    while n > 0 {
        sum += n % 10;
        n /= 10;
    }
    sum
}

pub fn digit_sum_sq(n: u64) -> u64 {
    let mut sum = 0;
    let mut n = n;
    while n > 0 {
        let digit = n % 10;
        sum += digit * digit;
        n /= 10;
    }
    sum
}

pub fn reverse_digits(mut n: u64) -> u64 {
    let mut reversed = 0;
    while n > 0 {
        reversed = reversed * 10 + (n % 10);
        n /= 10;
    }
    reversed
}

pub fn is_palindrome_num(n: u64) -> bool {
    n == reverse_digits(n)
}

pub fn is_palindrome_str(s: &str) -> bool {
    let bytes = s.as_bytes();
    let mut left = 0;
    let mut right = bytes.len();
    while left < right {
        right -= 1;
        if bytes[left] != bytes[right] {
            return false;
        }
        left += 1;
    }
    true
}

pub fn is_pandigital(s: &str) -> bool {
    if s.len() != 9 {
        return false;
    }
    let mut chars: Vec<char> = s.chars().collect();
    chars.sort_unstable();
    chars.iter().collect::<String>() == "123456789"
}

pub fn is_permutation(a: u64, b: u64) -> bool {
    let mut a_digits: Vec<char> = a.to_string().chars().collect();
    let mut b_digits: Vec<char> = b.to_string().chars().collect();
    a_digits.sort_unstable();
    b_digits.sort_unstable();
    a_digits == b_digits
}

pub fn digits(n: u64) -> Vec<u32> {
    let mut result = Vec::new();
    let mut n = n;
    while n > 0 {
        result.push((n % 10) as u32);
        n /= 10;
    }
    result.reverse();
    result
}

pub fn digit_factorial_sum(n: u64) -> u64 {
    const FACTORIALS: [u64; 10] = [1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880];
    let mut sum = 0u64;
    let mut m = n;
    while m > 0 {
        sum += FACTORIALS[(m % 10) as usize];
        m /= 10;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digit_sum() {
        assert_eq!(digit_sum(123), 6);
        assert_eq!(digit_sum(0), 0);
        assert_eq!(digit_sum(999), 27);
    }

    #[test]
    fn test_reverse_digits() {
        assert_eq!(reverse_digits(123), 321);
        assert_eq!(reverse_digits(100), 1);
    }

    #[test]
    fn test_is_palindrome_num() {
        assert!(is_palindrome_num(121));
        assert!(is_palindrome_num(9));
        assert!(!is_palindrome_num(123));
    }

    #[test]
    fn test_is_pandigital() {
        assert!(is_pandigital("123456789"));
        assert!(!is_pandigital("123456788"));
        assert!(!is_pandigital("12345678"));
    }

    #[test]
    fn test_is_permutation() {
        assert!(is_permutation(123, 321));
        assert!(!is_permutation(123, 124));
    }

    #[test]
    fn test_digit_factorial_sum() {
        assert_eq!(digit_factorial_sum(1), 1);
        assert_eq!(digit_factorial_sum(2), 2);
        assert_eq!(digit_factorial_sum(145), 1 + 24 + 120); // 1! + 4! + 5!
        assert_eq!(digit_factorial_sum(40585), 24 + 1 + 120 + 40320 + 120);
    }
}
