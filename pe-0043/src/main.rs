// Sub-string divisibility
// https://projecteuler.net/problem=43

fn solve() -> u64 {
    let mut used = [false; 10];
    let mut total = 0u64;
    let d = [2, 3, 5, 7, 11, 13, 17];
    fn enumerate(used: &mut [bool; 10], total: &mut u64, d: &[usize], s: String) {
        if s.len() > 3 && s[s.len()-3..].parse::<usize>().unwrap() % d[s.len()-4] != 0 {
            return;
        }
        if s.len() == 10 {
            *total += s.parse::<u64>().unwrap();
        }
        for i in 0..10 {
            let digit = (i as u8 + b'0') as char;
            if !used[i] {
                used[i] = true;
                enumerate(used, total, d, s.clone() + &digit.to_string());
                used[i] = false;
            }
        }
    }
    enumerate(&mut used, &mut total, &d, String::new());
    total
}

fn main() {
    pe_utils::run(43, solve);
}
