// Highly Divisible Triangular Number
// https://projecteuler.net/problem=12

fn calculate_highly_composite_number(target_divisors: u64) -> u64 {
    let mut result = 0;
    let mut divisors_count = 0;
    let mut number_to_check = 0;

    while divisors_count <= target_divisors {
        divisors_count = 0;
        number_to_check += 1;
        result += number_to_check;

        let upper_bound = (result as f64).sqrt() as u64;
        
        for n in 1..=upper_bound {
            if result % n == 0 {
                divisors_count += 2;
            }
        }
    }

    result
}

fn solve() -> u64 {
    calculate_highly_composite_number(500)
}

fn main() {
    pe_utils::run(12, solve);
}
