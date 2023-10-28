// Large Non-Mersenne Prime
// https://projecteuler.net/problem=97
 
 fn main() {
    let timer = std::time::Instant::now();
     
    let mut answer: i64 = 1;
    let m: i64 = 10_000_000_000;
     
    for _i in 1..=7_830_457 {
        answer = (2 * answer) % m;
    }
     
    answer = (answer * 28_433) % m;
    answer = (answer + 1) % m;
    
    let duration = timer.elapsed();
     
    //
    println!("\nProject Euler #97\nAnswer: {}", answer);
    println!("Elapsed time: {} milliseconds.\n", duration.as_millis());
 }
 