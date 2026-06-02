// Pandigital Fibonacci ends
// https://projecteuler.net/problem=104

use num::bigint::BigInt;
use pe_lib::is_pandigital;

fn solve() -> u32 {
    let mut fib_prev = BigInt::from(0u32);
    let mut fib_curr = BigInt::from(1u32);
    let mut index = 1u32;
    loop {
        let fib_next: BigInt = &fib_prev + &fib_curr;
        fib_prev = fib_curr.clone();
        fib_curr = fib_next.clone();
        index += 1;
        if is_pandigital(&format!("{}", fib_next.clone() % 1_000_000_000u64)) {
            let fib_str = format!("{}", fib_next);
            if is_pandigital(&fib_str[0..9]) {
                return index;
            }
        }
    }
}

pe_utils::pe_main!();
