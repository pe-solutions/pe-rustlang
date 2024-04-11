// Special Pythagorean Triplet
// https://projecteuler.net/problem=9

const PERIMETER: i32 = 1000;

fn find_pythagorean_triplet() -> Option<(i32, i32, i32)> {
    for a in 1..(PERIMETER / 3) {
        for b in (a + 1)..(PERIMETER / 2) {
            let c = PERIMETER - a - b;
            //
            if a.pow(2) + b.pow(2) == c.pow(2) {
                return Some((a, b, c));
            }
        }
    }
    
    None
}

fn main() {
    let start = std::time::Instant::now();
    
    match find_pythagorean_triplet() {
        Some((a, b, c)) => println!("\nProject Euler #9\nAnswer: {}", a * b * c),
        None => println!("No triplet found!"),
    }
    
    let duration = start.elapsed();
    
    println!("Elapsed time: {:?} milliseconds.\n", duration.as_millis()); 
}
