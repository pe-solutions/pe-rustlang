// Special Pythagorean Triplet
// https://projecteuler.net/problem=9

const P: u32 = 1_000u32;

fn find_sp_triplet() -> Option<(u32, u32, u32)> {
    for a in 1..P / 3 {
        let n = P.pow(2) - 2 * P * a;
        let d = 2 * P - 2 * a;
        
        if d != 0 && n % d == 0 {
            let b = n / d;
            let c = P - a - b;
            
            return Some((a, b, c));
        }
    }
    
    None
}

fn main() {
    let start = std::time::Instant::now();

    match find_sp_triplet() {
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
