// Maximum Path Sum II
// https://projecteuler.net/problem=67

use pe_lib::read_space_separated_matrix;

fn solve() -> u64 {
    let mut triangle = read_space_separated_matrix::<u64>("data/0067_triangle.txt")
        .expect("failed to read data/0067_triangle.txt");

    for i in (0..triangle.len() - 1).rev() {
        for j in 0..triangle[i].len() {
            triangle[i][j] += triangle[i + 1][j].max(triangle[i + 1][j + 1]);
        }
    }
    triangle[0][0]
}

pe_utils::pe_main!();
