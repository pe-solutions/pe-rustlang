// Pandigital Multiples
// https://projecteuler.net/problem=38

fn is_pandigital(num: i32) -> bool {
    let num_str = num.to_string();
    let mut sorted_chars: Vec<_> = num_str.chars().collect();
    sorted_chars.sort();
    sorted_chars.into_iter().collect::<String>() == "123456789"
}


fn main() {
    let start = std::time::Instant::now();
    
    for index in (1..=9876).rev() {
        let candidate = index * (100_002);

        if is_pandigital(candidate) {
            println!("\nProject Euler #38\nAnswer: {}", candidate);
            // 
            break;
        }
    }

    let duration = start.elapsed();

    println!("Elapsed time: {} milliseconds.\n", duration.as_millis());
}