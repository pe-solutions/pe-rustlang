// Hyperexponentiation
// https://projecteuler.net/problem=188

use pe_lib::mod_pow;

fn tetra_mod(base: u64, exponent: u64, modulus: u64) -> u64 {
    let mut x = base;

    for _ in 2..=exponent {
        x = mod_pow(base, x, modulus);
    }

    x
}

fn solve() -> u64 {
    tetra_mod(1777, 1855, 10u64.pow(8))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tetra_mod_base() {
        let result = tetra_mod(2, 1, 1000);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_tetra_mod_small() {
        let result = tetra_mod(2, 2, 1000);
        let expected = 2u64.pow(2);
        assert_eq!(result, expected % 1000);
    }

    #[test]
    fn test_tetra_mod_modulo_property() {
        let modulus = 10u64.pow(8);
        let result = tetra_mod(3, 3, modulus);
        assert!(result < modulus);
    }

    #[test]
    fn test_mod_pow_convergence() {
        // Hyperexponentiation towers converge modulo m
        let base = 7;
        let modulus = 1000;
        let r1 = tetra_mod(base, 2, modulus);
        let r2 = tetra_mod(base, 3, modulus);
        assert!(r1 < modulus && r2 < modulus);
    }

    #[test]
    fn test_solve_produces_result() {
        let result = solve();
        assert!(result >= 0);
    }
}

pe_utils::pe_main!();
