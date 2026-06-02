// Counting Summations
// https://projecteuler.net/problem=76

use pe_lib::count_partitions;

fn solve() -> usize {
    count_partitions(100) - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partitions_small_values() {
        assert_eq!(count_partitions(1), 1);
        assert_eq!(count_partitions(2), 2);
        assert_eq!(count_partitions(3), 3);
        assert_eq!(count_partitions(4), 5);
        assert_eq!(count_partitions(5), 7);
    }

    #[test]
    fn test_partitions_excluding_itself() {
        // We exclude 1 partition (the number itself as a single part)
        let partitions_100 = count_partitions(100);
        assert!(partitions_100 > 1);
    }

    #[test]
    fn test_solve_produces_output() {
        let result = solve();
        assert!(result > 0);
        // Partitions of 100 excluding itself should be very large
        assert!(result > 1_000_000);
    }

    #[test]
    fn test_partitions_monotonic_increase() {
        for n in 1..20 {
            assert!(count_partitions(n) < count_partitions(n + 1));
        }
    }
}

pe_utils::pe_main!();
