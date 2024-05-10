// Coin Sums
// https://projecteuler.net/problem=31

const COINS: [usize; 8] = [1, 2, 5, 10, 20, 50, 100, 200];
const TARGET: usize = 200;

fn main() {
    let start = std::time::Instant::now();
    
    let mut ways = [0; TARGET + 1];
    
    ways[0] = 1;
    
    COINS.iter().for_each(|&coin| {
        (coin..=TARGET).for_each(|amount| ways[amount] += ways[amount - coin])
    });
    
    let answer = ways[TARGET];
    
    println!("\nProject Euler #31\nAnswer: {}", answer);

    let duration = start.elapsed();
    println!("Elapsed time: {} milliseconds.\n", duration.as_millis());
}
