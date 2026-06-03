// Matrix Sum
// https://projecteuler.net/problem=345

use pe_utils::read_space_separated_matrix;

const N: usize = 15;
const N2: usize = 1 << N;

fn solve() -> i32 {
    let a: Vec<Vec<i32>> = read_space_separated_matrix("data/0345_matrix.txt").expect("failed to read data/0345_matrix.txt");
    let mut dp = vec![[0i32; N2]; N + 1];
    for n in 0..N {
        for c in 0..N2 {
            dp[n + 1][c] = dp[n][c];
            for x in 0..N {
                if (1 << x) & c != 0 {
                    let r = a[n][x] + dp[n][c - (1 << x)];
                    if dp[n + 1][c] < r { dp[n + 1][c] = r; }
                }
            }
        }
    }
    dp[N][N2 - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitmask_operations() {
        // Test bit shifting and masking
        let mask = 1 << 3; // bit 3 set
        assert_eq!(mask, 8);
        assert_eq!((1 << 3) & mask, 8);
        assert_eq!((1 << 2) & mask, 0);
    }

    #[test]
    fn test_bit_manipulation() {
        let c = 0b1010u32; // bits 1 and 3 are set
        let mut set_bits = Vec::new();
        for x in 0..4 {
            if (1 << x) & c != 0 {
                set_bits.push(x);
            }
        }
        assert_eq!(set_bits, vec![1, 3]);
    }

    #[test]
    fn test_solve_produces_result() {
        let result = solve();
        assert!(result > 0);
    }
}

pe_utils::pe_main!();
