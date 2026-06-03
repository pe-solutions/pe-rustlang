// Roman Numerals
// https://projecteuler.net/problem=89

use pe_utils::read_lines;

fn roman_to_int(s: &str) -> i32 {
    let mut result = 0;
    let mut prev = 0;
    for c in s.chars().rev() {
        let val = match c {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0,
        };
        if val < prev {
            result -= val;
        } else {
            result += val;
        }
        prev = val;
    }
    result
}

fn int_to_roman(n: i32) -> String {
    let vals = [1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
    let syms = ["M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I"];
    let mut result = String::new();
    let mut num = n;
    for (i, &v) in vals.iter().enumerate() {
        while num >= v {
            result.push_str(syms[i]);
            num -= v;
        }
    }
    result
}

fn solve() -> u64 {
    let lines = read_lines("data/0089_roman.txt").unwrap_or_default();
    let mut total = 0;
    for line in lines {
        let original_len = line.len() as u64;
        let num = roman_to_int(&line);
        let optimized = int_to_roman(num);
        let optimized_len = optimized.len() as u64;
        total += original_len - optimized_len;
    }
    total
}

pe_utils::pe_main!();
