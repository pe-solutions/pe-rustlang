// Right Triangles with Integer Coordinates
// https://projecteuler.net/problem=91

use itertools::iproduct;

fn is_valid_tuple(k1: i32, k2: i32, l1: i32, l2: i32) -> bool {
    (k1 != 0 || k2 != 0) && k1 * l2 != k2 * l1
        && (k1 * l1 + k2 * l2 == 0 || k1 * k1 + k2 * k2 == k1 * l1 + k2 * l2 || l1 * l1 + l2 * l2 == k1 * l1 + k2 * l2)
}

fn count_triangles(n: i32) -> i32 {
    iproduct!(0..=n, 0..=n, 0..=n, 0..=n)
        .filter(|&(k1, k2, l1, l2)| is_valid_tuple(k1, k2, l1, l2))
        .count() as i32 / 2
}

fn main() {
    let start = std::time::Instant::now();
	
	let answer = count_triangles(50);
	
	let duration = start.elapsed();
	
    println!("\nProject Euler #91\nAnswer: {}", answer);
    println!("Elapsed time: {:?} milliseconds.\n", duration.as_millis());
}
