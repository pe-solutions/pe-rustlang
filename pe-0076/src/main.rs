// Counting Summations
// https://projecteuler.net/problem=76

fn compute_partitions(n: usize) -> usize {
    let mut partitions = vec![0usize; n + 1];
    partitions[0] = 1;

    for num in 1..=n {
        for i in num..=n {
            partitions[i] += partitions[i - num];
        }
    }

    partitions[n]
}

fn n_partitions(n: usize) -> usize {
    compute_partitions(n)-1
}    

fn solve() -> usize {
    n_partitions(100)
}

pe_utils::pe_main!();
