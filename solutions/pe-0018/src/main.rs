// Maximum Path Sum I
// https://projecteuler.net/problem=18

use pe_utils::read_space_separated_matrix;

fn solve() -> u64 {
    let mut triangle = read_space_separated_matrix::<u64>("data/0018_triangle.txt")
        .expect("failed to read data/0018_triangle.txt");

    for i in (0..triangle.len() - 1).rev() {
        for j in 0..triangle[i].len() {
            triangle[i][j] += triangle[i + 1][j].max(triangle[i + 1][j + 1]);
        }
    }
    triangle[0][0]
}

pe_utils::pe_main!();
