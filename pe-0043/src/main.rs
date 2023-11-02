// Sub-string divisibility
// https://projecteuler.net/problem=43

 fn main() {
    let mut used = [false; 10];
    let mut total = 0u64;

    let d = [2, 3, 5, 7, 11, 13, 17];

    fn enumerate(used: &mut [bool; 10], total: &mut u64, d: &[usize], s: String) {
        if s.len() > 3 && s[s.len() - 3..].parse::<usize>().unwrap() % d[s.len() - 4] != 0 {
            return;
        }

        if s.len() == 10 {
            *total += s.parse::<u64>().unwrap();
        }

        for i in 0..10 {
            let digit = (i as u8 + '0' as u8) as char;

            if !used[i] {
                used[i] = true;
                enumerate(used, total, d, s.clone() + &digit.to_string());
                used[i] = false;
            }
        }
    }

    let start = std::time::Instant::now();

    enumerate(&mut used, &mut total, &d, String::from(""));
    
    let duration = start.elapsed();

    println!("\nProject Euler #43\nAnswer:{}", total);
    println!("Elapsed time: {} milliseconds.", duration.as_millis());
}
