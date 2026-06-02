// Factorial Trailing Digits
// https://projecteuler.net/problem=160

fn f(n: u64) -> u64 {
    let mut result: u64 = 1u64;

    for i in 1..=n {
        result *= i;

        while result % 10 == 0 {
            result /= 10;
        }

        result %= 1_000_000_000_000;
    }

    result % 100_000
}

fn solve() -> u64 {
    f(2_560_000)
}

pe_utils::pe_main!();
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_f_9() {
        assert_eq!(f(9), 36288);
    }

    #[test]
    fn test_f_10() {
        assert_eq!(f(10), 36288);
    }

    #[test]
    fn test_f_20() {
        assert_eq!(f(20), 17664);
    }
}

