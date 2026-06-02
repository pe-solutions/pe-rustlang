// XOR Decryption
// https://projecteuler.net/problem=59

use pe_lib::read_file_to_string;

fn solve() -> u64 {
    let content = read_file_to_string("data/0059_cipher.txt").unwrap();
    let bytes: Vec<u8> = content
        .trim()
        .split(',')
        .filter_map(|s| s.parse::<u8>().ok())
        .collect();

    for a in b'a'..=b'z' {
        for b in b'a'..=b'z' {
            for c in b'a'..=b'z' {
                let key = [a, b, c];
                let decrypted: Vec<u8> = bytes
                    .iter()
                    .enumerate()
                    .map(|(i, &byte)| byte ^ key[i % 3])
                    .collect();

                if let Ok(text) = String::from_utf8(decrypted.clone()) {
                    if text.contains("the") && text.to_lowercase().contains("and") {
                        return decrypted.iter().map(|&b| b as u64).sum();
                    }
                }
            }
        }
    }
    0
}

pe_utils::pe_main!();
