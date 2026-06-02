// Matrix Sum
// https://projecteuler.net/problem=345

use std::fs;

const N: usize = 15;
const N2: usize = 1 << N;

fn solve() -> i32 {
    let content = fs::read_to_string("data/0345_matrix.txt").expect("failed to read data/0345_matrix.txt");
    let a: Vec<Vec<i32>> = content.lines()
        .map(|line| line.split_whitespace().map(|n| n.parse().unwrap()).collect())
        .collect();
    let mut dp = vec![[0i32; N2]; N + 1];
    for n in 0..N {
        for c in 0..N2 {
            dp[n + 1][c] = dp[n][c];
            for x in 0..N {
                if (1 << x) & c != 0 {
                    let r = a[n][x] + dp[n][c - (1 << x)];
                    if dp[n + 1][c] < r { dp[n + 1][c] = r; }
                }
            }
        }
    }
    dp[N][N2 - 1]
}

pe_utils::pe_main!();
