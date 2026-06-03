// Passcode Derivation
// https://projecteuler.net/problem=79

use pe_utils::read_file_to_string;

fn solve() -> u64 {
    let content = read_file_to_string("data/0079_keylog.txt").unwrap();
    let attempts: Vec<&str> = content.lines().collect();
    
    for num in 123_456_789u64..=987_654_321u64 {
        let s = num.to_string();
        if s.chars().collect::<std::collections::HashSet<_>>().len() == 9 {
            let valid = attempts.iter().all(|attempt| {
                let mut pos = 0;
                for c in attempt.chars() {
                    if let Some(new_pos) = s[pos..].find(c) {
                        pos += new_pos + 1;
                    } else {
                        return false;
                    }
                }
                true
            });
            
            if valid {
                return num;
            }
        }
    }
    0
}

pe_utils::pe_main!();
