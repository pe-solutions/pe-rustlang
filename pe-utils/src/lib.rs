pub fn run<T: std::fmt::Display>(num: u16, f: impl FnOnce() -> T) {
    let start = std::time::Instant::now();
    let answer = f();
    println!("\nProject Euler #{}\nAnswer: {}", num, answer);
    println!("Elapsed time: {} milliseconds.\n", start.elapsed().as_millis());
}

/// Generates `fn main()` deriving the problem number from the crate name at
/// compile time (e.g. `pe-0042` → 42). Usage: `pe_utils::pe_main!();`
#[macro_export]
macro_rules! pe_main {
    () => {
        fn main() {
            pe_utils::run(
                env!("CARGO_PKG_NAME")
                    .strip_prefix("pe-")
                    .and_then(|s| s.parse::<u16>().ok())
                    .expect("crate name must be pe-NNNN"),
                solve,
            );
        }
    };
}
