// Special Pythagorean Triplet
// https://projecteuler.net/problem=9

fn find_triplet() -> Option<(u32, u32, u32)> {
    for a in 1..334 {
        for b in a + 1..(1000 - a) / 2 {
            let c = 1000 - a - b;
            //
            if a * a + b * b == c * c {
                return Some((a, b, c));
            }
        }
    }

    None
}

fn main() {
    let start = std::time::Instant::now();

    match find_triplet() {
        Some((a, b, c)) => {
            println!("\nProject Euler #9\nAnswer: {}", a*b*c);
        }
        None => {
            println!("No solution found.");
        }
    }

    let duration = start.elapsed();
    println!("Elapsed time: {:?} milliseconds.\n", duration.as_millis()); 
}
