fn s_of_fac_digits(n: u32) -> u32 {
    let f = [1u32, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880];
    
    let mut s = 0;
    let mut n = n;

    while n != 0 {
        s += f[(n % 10) as usize];
        n /= 10;
    }

    s
}

fn main() {
    let start = std::time::Instant::now();
 
    let s_of_fac_pred = |n: u32| s_of_fac_digits(n) == n;

    let mut answer = 0;
    for n in 3..10_000_000 {
        if s_of_fac_pred(n) {
            answer += n;
        }
    }

    let duration = start.elapsed();

    println!("\nProject Euler #34\nAnswer: {}", answer);
    println!("Elapsed time: {} milliseconds.\n", duration.as_millis());
}
