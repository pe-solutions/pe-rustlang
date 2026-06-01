// Pandigital Multiples
// https://projecteuler.net/problem=38

fn is_pandigital(num: i32) -> bool {
    let num_str = num.to_string();
    let mut sorted_chars: Vec<_> = num_str.chars().collect();
    sorted_chars.sort();
    sorted_chars.into_iter().collect::<String>() == "123456789"
}


fn solve() -> i32 {
    for index in (1..=9876).rev() {
        let candidate = index * 100_002;
        if is_pandigital(candidate) {
            return candidate;
        }
    }
    panic!("no pandigital found")
}

fn main() {
    pe_utils::run(38, solve);
}
