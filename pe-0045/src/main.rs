// Triangular, Pentagonal, and Hexagonal
// https://projecteuler.net/problem=45

fn find_next_tph() -> i32 {
    let mut p = 5;
    let mut p_inc = 7;

    let mut h = 6;
    let mut h_inc = 9;

    let mut equal_amount_found = 0;

    while equal_amount_found <= 1 {
        if p <= h {
            p += p_inc;
            p_inc += 3;
        } else {
            h += h_inc;
            h_inc += 4;
        }

        if p == h {
            equal_amount_found += 1;
        }
    }

    return h;
}

fn main() {
    let start = std::time::Instant::now();
     
    let answer = find_next_tph();

    let duration = start.elapsed();

    println!("\nProject Euler #45\nAnswer:{}", answer);
    println!("Elapsed time: {} milliseconds.\n", duration.as_millis());
}
