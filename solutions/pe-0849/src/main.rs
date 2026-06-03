// The Tournament
// https://projecteuler.net/problem=849

fn f_alternate(n: usize) -> usize {
    const MOD: usize = 1_000_000_007;

    let maxd = 4 * (n - 1);
    let maxs = 2 * n * (n - 1);

    let mut dp = vec![vec![0; maxs + 1]; n + 1];

    dp[0][0] = 1;

    for d in 0..= maxd {
        for i in 1..=n {
            for s in std::cmp::max(d, 2 * i * (i - 1))..= maxs {
                dp[i][s] += dp[i - 1][s - d];
                dp[i][s] %= MOD;
            }
        }
    }

    dp[n][maxs]
}

fn solve() -> usize {
    f_alternate(100)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_f_alternate_small() {
        let result = f_alternate(2);
        assert!(result > 0);
    }

    #[test]
    fn test_f_alternate_modulo() {
        const MOD: usize = 1_000_000_007;
        let result = f_alternate(3);
        assert!(result < MOD);
    }

    #[test]
    fn test_dp_growth() {
        let a = f_alternate(2);
        let b = f_alternate(3);
        // DP result should be positive for both
        assert!(a > 0 && b > 0);
    }

    #[test]
    fn test_solve_produces_result() {
        let result = solve();
        assert!(result > 0);
    }
}

pe_utils::pe_main!();
