// Coin Sums
// https://projecteuler.net/problem=31

fn count_ways(amount: i32) -> i32 {
    
    const COINS: [i32; 8] = [1, 2, 5, 10, 20, 50, 100, 200];
    
    let mut ways: Vec<i32> = vec![0; (amount + 1) as usize];

    ways[0] = 1;

    for coin in COINS.iter() {
        for i in *coin..=amount {
            ways[i as usize] += ways[(i - coin) as usize];
        }
    }

    ways[amount as usize]
}

fn main() {
    let start = std::time::Instant::now();

    let answer = count_ways(200);

    println!("\nProject Euler #31\nAnswer: {}", answer);

    let duration = start.elapsed();
    println!("Elapsed time: {} milliseconds.\n", duration.as_millis());
}
