// Largest Product in a Grid
// https://projecteuler.net/problem=11

use pe_lib::read_space_separated_matrix;

fn solve() -> i32 {
    let grid: Vec<Vec<i32>> = read_space_separated_matrix("data/0011_grid.txt").expect("failed to read data/0011_grid.txt");
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

pe_utils::pe_main!();
