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

fn main() {
    let start = std::time::Instant::now();
    
    const N: usize = 100;
    let answer = n_partitions(N);

    assert_eq!(n_partitions(5), 6); // Test
        
    println!("\nProject Euler #31\nAnswer: {}", answer);

    let duration = start.elapsed();
    println!("Elapsed time: {} milliseconds.\n", duration.as_millis());
}
