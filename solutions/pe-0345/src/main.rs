// Matrix Sum
// https://projecteuler.net/problem=345

use pe_lib::read_space_separated_matrix;

const N: usize = 15;
const N2: usize = 1 << N;

fn solve() -> i32 {
    let a: Vec<Vec<i32>> = read_space_separated_matrix("data/0345_matrix.txt").expect("failed to read data/0345_matrix.txt");
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
