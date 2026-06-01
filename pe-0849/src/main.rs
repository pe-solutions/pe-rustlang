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

pe_utils::pe_main!();
