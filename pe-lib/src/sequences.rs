pub struct Fibonacci {
    a: u64,
    b: u64,
}

impl Fibonacci {
    pub fn new() -> Self {
        Fibonacci { a: 0, b: 1 }
    }
}

impl Default for Fibonacci {
    fn default() -> Self {
        Self::new()
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let next_value = self.a;
        self.a = self.b;
        self.b = next_value + self.b;
        Some(next_value)
    }
}

pub fn triangular(n: u64) -> u64 {
    n * (n + 1) / 2
}

pub fn pentagonal(n: u64) -> u64 {
    n * (3 * n - 1) / 2
}

pub fn hexagonal(n: u64) -> u64 {
    n * (2 * n - 1)
}

pub fn heptagonal(n: u64) -> u64 {
    n * (5 * n - 3) / 2
}

pub fn octagonal(n: u64) -> u64 {
    n * (3 * n - 2)
}

pub fn is_triangular(n: u64) -> bool {
    let discriminant = 1 + 8 * n;
    let sqrt_disc = (discriminant as f64).sqrt() as u64;
    if sqrt_disc * sqrt_disc != discriminant {
        return false;
    }
    (sqrt_disc + 1) % 2 == 0 && (sqrt_disc - 1) / 2 > 0
}

pub fn is_pentagonal(n: u64) -> bool {
    let num = 24 * n + 1;
    let sqrt_num = (num as f64).sqrt() as u64;
    sqrt_num * sqrt_num == num && (sqrt_num + 1) % 6 == 0
}

pub fn is_hexagonal(n: u64) -> bool {
    let num = 8 * n + 1;
    let sqrt_num = (num as f64).sqrt() as u64;
    sqrt_num * sqrt_num == num && (sqrt_num + 1) % 4 == 0
}
