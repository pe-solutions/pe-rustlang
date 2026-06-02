// Singular Integer Right Triangles
// https://projecteuler.net/problem=75

use pe_lib::gcd;

fn solve() -> u64 {
    let mut counts = vec![0; 1_500_001];
    
    for m in 2..1415 {
        for n in 1..m {
            if (m - n) % 2 == 1 && gcd(m as u64, n as u64) == 1 {
                let a = m * m - n * n;
                let b = 2 * m * n;
                let c = m * m + n * n;
                let p = a + b + c;
                
                if p <= 1_500_000 {
                    for k in (1..).take_while(|k| k * p <= 1_500_000) {
                        counts[k * p] += 1;
                    }
                }
            }
        }
    }
    
    counts.iter().filter(|&&c| c == 1).count() as u64
}

pe_utils::pe_main!();
