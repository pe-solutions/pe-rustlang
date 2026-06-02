// Path Sum: Two Ways
// https://projecteuler.net/problem=81

use std::io;
use pe_lib::read_csv_matrix;

type MatrixError = io::Error;

fn min_path_sum(matrix: &[Vec<u32>]) -> u32 {
    let mut dp = matrix.to_owned();

    let rows = dp.len();
    let cols = dp[0].len();

    for j in 1..cols {
        dp[0][j] += dp[0][j - 1];
    }

    for i in 1..rows {
        dp[i][0] += dp[i - 1][0];
    }

    for i in 1..rows {
        for j in 1..cols {
            dp[i][j] += dp[i - 1][j].min(dp[i][j - 1]);
        }
    }

    dp[rows - 1][cols - 1]
}

fn solve() -> u32 {
    let matrix = read_csv_matrix("data/0081_matrix.txt").expect("failed to read data/0081_matrix.txt");
    min_path_sum(&matrix)
}

pe_utils::pe_main!();
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_data() {
        let result = read_data("data/test_matrix.txt");
        assert!(result.is_ok());
        let matrix = result.unwrap();
        assert_eq!(matrix, vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);
    }

    #[test]
    fn test_min_path_sum() {
        let matrix = vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]];
        let result = min_path_sum(&matrix);
        assert_eq!(result, 7);
    }
}

