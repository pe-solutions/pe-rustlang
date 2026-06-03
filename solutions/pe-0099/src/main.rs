// Largest Exponential
// https://projecteuler.net/problem=99

use pe_utils::read_lines;

fn solve() -> u64 {
    let lines = read_lines("data/0099_base_exp.txt").unwrap_or_default();
    let mut max_log = 0.0;
    let mut max_line = 0u64;

    for (idx, line) in lines.iter().enumerate() {
        let parts: Vec<&str> = line.split(',').map(|s| s.trim()).collect();
        if parts.len() == 2 {
            let base: f64 = parts[0].parse().unwrap_or(0.0);
            let exp: f64 = parts[1].parse().unwrap_or(0.0);
            let log_val = exp * base.ln();

            if log_val > max_log {
                max_log = log_val;
                max_line = (idx + 1) as u64;
            }
        }
    }

    max_line
}

pe_utils::pe_main!();
