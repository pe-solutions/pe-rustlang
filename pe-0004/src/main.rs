// Largest Palindrome Product
// https://projecteuler.net/problem=4

fn is_palindrome(n: i32) -> bool {
    let s = n.to_string();
    s == s.chars().rev().collect::<String>()
}

fn find_largest_palindrome_product(range: std::ops::RangeInclusive<i32>) -> Option<i32> {
    let mut max_palindrome = 0;

    for a in range.clone() {
        for b in range.clone() {
            let product = a * b;
            if is_palindrome(product) && product > max_palindrome {
                max_palindrome = product;
            }
        }
    }

    if max_palindrome > 0 {
        Some(max_palindrome)
    } else {
        None
    }
}

fn main() {
    let start = std::time::Instant::now(); // let start = Instant::now();
    //
    
    let range = 899..=999;

    match find_largest_palindrome_product(range) {
        Some(ans) => {
            println!("\nProject Euler #4");
            println!("Answer: {}", ans);
        }
        None => {
            println!("No palindrome found in the specified range.");
        }
    }

    //
    let duration = start.elapsed();
    
    println!("Elapsed time: {:?} milliseconds.\n", duration.as_millis()); 
}
