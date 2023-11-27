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

fn main() {
    let start = std::time::Instant::now();
    
    const N: usize = 100;
    let answer = f_alternate(N);

    let duration = start.elapsed();

    println!("\nProject Euler #849\nAnswer: {}", answer);
    println!("Elapsed time: {} milliseconds.\n", duration.as_millis()); 
}
