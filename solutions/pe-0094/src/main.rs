// Almost Equilateral Triangles
// https://projecteuler.net/problem=94

fn is_perfect_square(n: u64) -> bool {
    let root = (n as f64).sqrt() as u64;
    root * root == n
}

fn solve() -> u64 {
    let mut sum = 0u64;
    let limit = 1_000_000_000u64;

    for a in 1..limit / 3 {
        for b in &[a + 1, a - 1] {
            if *b == 0 {
                continue;
            }
            let perimeter = 2 * a + b;
            if perimeter > limit {
                break;
            }

            let s = (2 * a + b) / 2;
            let area_sq_16 = (2 * s - 2 * a) * (2 * s - 2 * a) * (2 * s - b);
            if is_perfect_square(area_sq_16) {
                sum += perimeter;
            }
        }
    }
    sum
}

pe_utils::pe_main!();
