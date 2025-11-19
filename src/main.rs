// Two-Dimensional Recurrence
// https://projecteuler.net/problem=940

const MOD: i64 = 1_123_581_313;
const MAX_K: usize = 50;

/// Returns the nth Fibonacci number
fn fibonacci(n: usize) -> u64 {
    let (mut a, mut b) = (0, 1);
    
    for _ in 0..n {
        let next = a + b;
        a = b;
        b = next;
    }
    
    a
}

/// Computes the nth term of a linear recurrence:
/// f[n] = a * f[n-1] + b * f[n-2], given f(0) and f(1)
fn solve_recurrence(initial0: i64, initial1: i64, a: i64, b: i64, mut n: u64) -> i64 {
    let mut coeff0 = 1;
    let mut coeff1 = 0;
    let (mut base0, mut base1) = (0, 1);

    while n > 0 {
        if n % 2 == 1 {
            let new_coeff0 = (coeff0 * base0 + b * coeff1 * base1) % MOD;
            let new_coeff1 = (coeff0 * base1 + coeff1 * base0 + a * coeff1 * base1) % MOD;
            coeff0 = new_coeff0;
            coeff1 = new_coeff1;
        }

        let new_base0 = (base0 * base0 + b * base1 * base1) % MOD;
        let new_base1 = (2 * base0 * base1 + a * base1 * base1) % MOD;
        
        base0 = new_base0;
        base1 = new_base1;

        n /= 2;
    }

    (initial0 * coeff0 + initial1 * coeff1) % MOD
}

/// Computes A(n, m) based on nested linear recurrences
fn compute_a(n: u64, m: u64) -> i64 {
    let f0 = solve_recurrence(0, 1, 3, 1, n);
    let f1 = solve_recurrence(1, 2, 3, 1, n);
    
    solve_recurrence(f0, f1, 1, 3, m)
}

fn main() {
    let start = std::time::Instant::now();
    
    // >>>
    
    let mut total = 0;

    for i in 2..=MAX_K {
        let fi = fibonacci(i);
        
        for j in 2..=MAX_K {
            let fj = fibonacci(j);
            
            total = (total + compute_a(fi, fj)) % MOD;
        }
    }
    
    // <<<
    
    let duration = start.elapsed();

    println!("\nProject Euler #940\nAnswer: {}", total);
    println!("Elapsed time: {} milliseconds.\n", duration.as_millis()); 
}
