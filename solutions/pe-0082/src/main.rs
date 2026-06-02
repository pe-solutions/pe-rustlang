// Path Sum: Two Ways
// https://projecteuler.net/problem=82

use pe_lib::read_space_separated_matrix;

fn solve() -> u64 {
    let matrix = read_space_separated_matrix::<u64>("data/0082_matrix.txt")
        .expect("failed to read matrix");
    let rows = matrix.len();
    let cols = matrix[0].len();
    
    let mut dp = vec![vec![u64::MAX; cols]; rows];
    for i in 0..rows {
        dp[i][0] = matrix[i][0];
    }
    
    for j in 1..cols {
        for i in 0..rows {
            dp[i][j] = matrix[i][j] + dp[i][j - 1].min(if i > 0 { dp[i - 1][j] } else { u64::MAX });
        }
        for i in (0..rows - 1).rev() {
            dp[i][j] = dp[i][j].min(matrix[i][j] + dp[i + 1][j]);
        }
    }
    
    *dp.iter().map(|r| r[cols - 1]).collect::<Vec<_>>().iter().min().unwrap_or(&0)
}

pe_utils::pe_main!();
