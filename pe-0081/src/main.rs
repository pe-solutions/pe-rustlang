// Path Sum: Two Ways
// https://projecteuler.net/problem=81

use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
enum MatrixError {
    IoError(io::Error),
    ParseError(std::num::ParseIntError),
}

impl fmt::Display for MatrixError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MatrixError::IoError(err) => write!(f, "IO error: {}", err),
            MatrixError::ParseError(err) => write!(f, "Parse error: {}", err),
        }
    }
}

impl Error for MatrixError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            MatrixError::IoError(err) => Some(err),
            MatrixError::ParseError(err) => Some(err),
        }
    }
}

impl From<io::Error> for MatrixError {
    fn from(err: io::Error) -> Self {
        MatrixError::IoError(err)
    }
}

impl From<std::num::ParseIntError> for MatrixError {
    fn from(err: std::num::ParseIntError) -> Self {
        MatrixError::ParseError(err)
    }
}

fn read_data(file_path: &str) -> Result<Vec<Vec<u32>>, MatrixError> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut matrix = Vec::new();

    for line in reader.lines() {
        let row: Result<Vec<u32>, _> = line?
            .trim()
            .split(',')
            .map(|s| s.parse())
            .collect();
        matrix.push(row.map_err(MatrixError::from)?);
    }

    Ok(matrix)
}

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

fn main() {
    let start = std::time::Instant::now();
    
    let file_path = "data/matrix.txt";
    match read_data(file_path) {
        Ok(matrix) => {
            
            
            let answer = min_path_sum(&matrix);
            let duration = start.elapsed();

            println!("\nProject Euler #81\nAnswer: {}", answer);
            println!("Elapsed time: {} milliseconds.\n", duration.as_millis());
        }
        Err(e) => eprintln!("Error reading matrix: {}", e),
    }
}

// Test
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
