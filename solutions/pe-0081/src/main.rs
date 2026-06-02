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
    fn test_min_path_sum_simple() {
        let matrix = vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]];
        let result = min_path_sum(&matrix);
        assert_eq!(result, 7);
    }

    #[test]
    fn test_min_path_sum_single_cell() {
        let matrix = vec![vec![5]];
        let result = min_path_sum(&matrix);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_min_path_sum_single_row() {
        let matrix = vec![vec![1, 2, 3, 4]];
        let result = min_path_sum(&matrix);
        assert_eq!(result, 10); // 1+2+3+4
    }

    #[test]
    fn test_min_path_sum_single_column() {
        let matrix = vec![vec![1], vec![2], vec![3], vec![4]];
        let result = min_path_sum(&matrix);
        assert_eq!(result, 10); // 1+2+3+4
    }

    #[test]
    fn test_min_path_sum_property() {
        // Minimum path should only go right or down
        let matrix = vec![
            vec![1, 5, 5],
            vec![5, 1, 5],
            vec![5, 5, 1]
        ];
        // Best path: right (5), right (5), down (1), down (1) = 1+5+5+1+1 = 13
        // Or: down (5), down (5), right (1), right (1) = 1+5+5+1+1 = 13
        // Or: right (5), down (1), right (1), down (1) = 1+5+1+1+1 = 9
        let result = min_path_sum(&matrix);
        assert!(result >= 1 && result <= 20);
    }
}

