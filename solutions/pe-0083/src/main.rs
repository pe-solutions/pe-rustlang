// Path Sum: Four Ways
// https://projecteuler.net/problem=83

use pe_utils::read_space_separated_matrix;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn solve() -> u64 {
    let matrix = read_space_separated_matrix::<u64>("data/0083_matrix.txt")
        .expect("failed to read matrix");
    let n = matrix.len();
    
    let mut dist = vec![vec![u64::MAX; n]; n];
    dist[0][0] = matrix[0][0];
    
    let mut pq = BinaryHeap::new();
    pq.push(Reverse((dist[0][0], 0, 0)));
    
    while let Some(Reverse((d, r, c))) = pq.pop() {
        if d > dist[r][c] { continue; }
        
        for (dr, dc) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let nr = (r as i32 + dr) as usize;
            let nc = (c as i32 + dc) as usize;
            if nr < n && nc < n {
                let nd = d + matrix[nr][nc];
                if nd < dist[nr][nc] {
                    dist[nr][nc] = nd;
                    pq.push(Reverse((nd, nr, nc)));
                }
            }
        }
    }
    
    dist[n - 1][n - 1]
}

pe_utils::pe_main!();
