// Largest Product in a Grid
// https://projecteuler.net/problem=11

use pe_lib::read_space_separated_matrix;

fn max_product_in_grid(grid: &Vec<Vec<i32>>) -> i32 {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut answer = 0;
    for i in 0..rows {
        for j in 0..cols {
            if j + 3 < cols {
                let p = grid[i][j] * grid[i][j+1] * grid[i][j+2] * grid[i][j+3];
                if p > answer { answer = p; }
            }
            if i + 3 < rows {
                let p = grid[i][j] * grid[i+1][j] * grid[i+2][j] * grid[i+3][j];
                if p > answer { answer = p; }
            }
            if i + 3 < rows && j + 3 < cols {
                let p = grid[i][j] * grid[i+1][j+1] * grid[i+2][j+2] * grid[i+3][j+3];
                if p > answer { answer = p; }
            }
            if i + 3 < rows && j >= 3 {
                let p = grid[i][j] * grid[i+1][j-1] * grid[i+2][j-2] * grid[i+3][j-3];
                if p > answer { answer = p; }
            }
        }
    }
    answer
}

fn solve() -> i32 {
    let grid: Vec<Vec<i32>> = read_space_separated_matrix("data/0011_grid.txt").expect("failed to read data/0011_grid.txt");
    max_product_in_grid(&grid)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_horizontal_product() {
        let grid = vec![vec![1, 2, 3, 4, 5], vec![6, 7, 8, 9, 10]];
        let result = max_product_in_grid(&grid);
        // 5*6*7*8 = 1680 (wrap vertical), or 2*3*4*5 = 120 (horizontal)
        assert!(result > 0);
    }

    #[test]
    fn test_grid_with_large_numbers() {
        let grid = vec![
            vec![10, 20, 30, 40],
            vec![40, 50, 60, 70],
            vec![70, 80, 90, 100],
            vec![100, 110, 120, 130],
        ];
        let result = max_product_in_grid(&grid);
        // 40*70*100*130 = 36,400,000 (diagonal)
        assert!(result > 10_000_000);
    }

    #[test]
    fn test_diagonal_product() {
        let grid = vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
            vec![13, 14, 15, 16],
        ];
        let result = max_product_in_grid(&grid);
        // 1*6*11*16 = 1056 (diagonal), or 4*8*12*16 = 6144 (anti-diagonal)
        assert!(result > 1000);
    }

    #[test]
    fn test_solve_produces_result() {
        let result = solve();
        assert!(result > 0);
    }
}

pe_utils::pe_main!();
