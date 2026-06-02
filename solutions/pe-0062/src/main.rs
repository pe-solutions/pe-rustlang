// Cubic Permutations
// https://projecteuler.net/problem=62

use pe_lib::is_permutation;

fn solve() -> u64 {
    let mut cubes = Vec::new();
    for n in 1..10000u64 {
        cubes.push(n * n * n);
    }

    for &cube in &cubes {
        let mut count = 1;
        for &other in &cubes {
            if cube != other && is_permutation(cube, other) {
                count += 1;
                if count == 5 {
                    return cube;
                }
            }
        }
    }
    0
}

pe_utils::pe_main!();
