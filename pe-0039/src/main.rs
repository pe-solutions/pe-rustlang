// Integer Right Triangles
// https://projecteuler.net/problem=39

fn main() {
    let start = std::time::Instant::now();
    
    let (max_perimeter, _max_count) = (12..=1000).fold((0, 0), |(max_p, max_count), p| {
        let count = (2..p / 2)
            .flat_map(|b| (1..b).filter(move |&a| a * a + b * b == (p - a - b) * (p - a - b)))
            .count();

        if count > max_count {
            (p, count)
        } else {
            (max_p, max_count)
        }
    });

    let duration = start.elapsed();
    
    let answer = max_perimeter;

    println!("\nProject Euler #39\nAnswer: {}", answer);
    println!("Elapsed time: {} milliseconds.\n", duration.as_millis());
}
