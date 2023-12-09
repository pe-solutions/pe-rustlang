// Hyperexponentiation
// https://projecteuler.net/problem=188

fn tetra_mod(base: u64, exponent: u64, modulus: u64) -> u64 {
    fn modpow(mut base: u64, mut exponent: u64, modulus: u64) -> u64 {
        let mut result = 1;

        base %= modulus;

        while exponent > 0 {
            if exponent % 2 == 1 {
                result = (result * base) % modulus;
            }

            base = (base * base) % modulus;
            exponent /= 2;
        }

        result
    }

    let mut x = base;

    for _ in 2..=exponent {
        x = modpow(base, x, modulus);
    }

    x
}

fn main() {
    let start = std::time::Instant::now();
    let answer = tetra_mod(1777, 1855, 10u64.pow(8));
    let duration = start.elapsed();

    println!("\nProject Euler #188\nAnswer: {}", answer);
    println!("Elapsed time: {} milliseconds.\n", duration.as_millis());
}
