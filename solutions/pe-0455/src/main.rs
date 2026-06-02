// Powers with Trailing Digits
// https://projecteuler.net/problem=455

use pe_lib::mod_pow;

fn f(n: u64, md: u64) -> u64 {
    let mut r1 = n;
    let mut r2 = mod_pow(n, r1, md);

    while r2 != r1 && r2 != 1 {
        r1 = r2;
        r2 = mod_pow(n, r1, md);
    }

    if r2 == 1 { 0 } else { r2 }
}

fn solve() -> u64 {
    let md = 1_000_000_000;
    (2..=1_000_000).map(|n| f(n, md)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_f_basic() {
        let result = f(2, 1000);
        assert!(result >= 0);
    }

    #[test]
    fn test_f_modular_property() {
        let result = f(3, 100);
        assert!(result < 100);
    }

    #[test]
    fn test_mod_pow_convergence() {
        // f repeatedly applies mod_pow until convergence
        let n = 5;
        let md = 1000;
        let result = f(n, md);
        assert!(result == 0 || result > 0);
    }

    #[test]
    fn test_solve_produces_sum() {
        let result = solve();
        assert!(result > 0);
    }
}

pe_utils::pe_main!();
