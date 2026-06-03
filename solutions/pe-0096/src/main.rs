// Su Doku
// https://projecteuler.net/problem=96

use pe_utils::read_lines;

fn is_valid(board: &[[u8; 9]; 9], row: usize, col: usize, num: u8) -> bool {
    for i in 0..9 {
        if board[row][i] == num || board[i][col] == num {
            return false;
        }
    }
    let box_row = (row / 3) * 3;
    let box_col = (col / 3) * 3;
    for i in box_row..box_row + 3 {
        for j in box_col..box_col + 3 {
            if board[i][j] == num {
                return false;
            }
        }
    }
    true
}

fn solve_sudoku(board: &mut [[u8; 9]; 9]) -> bool {
    for row in 0..9 {
        for col in 0..9 {
            if board[row][col] == 0 {
                for num in 1..=9 {
                    if is_valid(board, row, col, num) {
                        board[row][col] = num;
                        if solve_sudoku(board) {
                            return true;
                        }
                        board[row][col] = 0;
                    }
                }
                return false;
            }
        }
    }
    true
}

fn solve() -> u64 {
    let lines = read_lines("data/0096_sudoku.txt").unwrap_or_default();
    let mut sum = 0u64;
    let mut i = 0;
    while i < lines.len() {
        if lines[i].starts_with("Grid") {
            i += 1;
            let mut board = [[0u8; 9]; 9];
            for row in 0..9 {
                for (col, c) in lines[i].chars().enumerate() {
                    if col < 9 {
                        board[row][col] = (c as u8 - b'0') as u8;
                    }
                }
                i += 1;
            }
            solve_sudoku(&mut board);
            let first_three = (board[0][0] as u64) * 100 + (board[0][1] as u64) * 10 + (board[0][2] as u64);
            sum += first_three;
        } else {
            i += 1;
        }
    }
    sum
}

pe_utils::pe_main!();
