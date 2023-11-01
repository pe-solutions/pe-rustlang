// Pentagon Numbers
// https://projecteuler.net/problem=44

fn is_pentagonal(n: u32) -> bool {
    fn int_sqrt(num: u32) -> u32 {
        let mut x = num >> 1;

        while x != (x + num / x) >> 1 {
            x = (x + num / x) >> 1;
        }

        x
    }

    let num = 24 * n + 1;
    let sqrt_num = int_sqrt(num);

    sqrt_num * sqrt_num == num && (sqrt_num + 1) % 6 == 0
}

fn main() {
    let start = std::time::Instant::now();

    let mut a: u32;
    let mut b: u32;

    for i in 1..2500 {
        for j in 1..i {
            a = i * (3 * i - 1) / 2;
            b = j * (3 * j - 1) / 2;

            if is_pentagonal(a - b) && is_pentagonal(a + b) {
                let duration = start.elapsed();

                println!("\nProject Euler #44\nAnswer:{}", a - b);
                println!("Elapsed time: {} milliseconds.\n", duration.as_millis());

                return;
            }
        }
    }
}
