pub fn run<T: std::fmt::Display>(num: u16, f: impl FnOnce() -> T) {
    let start = std::time::Instant::now();
    let answer = f();
    println!("\nProject Euler #{}\nAnswer: {}", num, answer);
    println!("Elapsed time: {} milliseconds.\n", start.elapsed().as_millis());
}
