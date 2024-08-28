// Powers with Trailing Digits
// https://projecteuler.net/problem=455

fn f(n: u64, md: u64) -> u64 {
    let mut r1 = n;
    let mut r2 = mod_exp(n, r1, md);
    
    while r2 != r1 && r2 != 1 {
        r1 = r2;
        r2 = mod_exp(n, r1, md);
    }
    
    if r2 == 1 { 0 } else { r2 }
}

#[inline]
fn mod_exp(base: u64, mut exp: u64, modulus: u64) -> u64 {
    let mut result: u64 = 1;
    
    let mut b: u64 = base % modulus;

    while exp > 0 {
        if exp % 2 == 1 {
            result = result.wrapping_mul(b) % modulus;
        }
        
        exp /= 2;
        
        b = b.wrapping_mul(b) % modulus;
    }
    
    result
}

fn main() {
    let start = std::time::Instant::now();
    
    let md = 1_000_000_000;
    let sum: u64 = (2..=1_000_000).map(|n| f(n, md)).sum();
    
    let duration = start.elapsed();

    println!("\nProject Euler #800\nAnswer: {}", sum);
    println!("Elapsed time: {} milliseconds.\n", duration.as_millis());
}
