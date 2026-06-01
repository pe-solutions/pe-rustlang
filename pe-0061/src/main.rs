// Cyclical Figurate Numbers
// https://projecteuler.net/problem=61

struct NumberFunctions;

impl NumberFunctions {
    fn triangle_number(n: i32) -> i32 { n * (n + 1) / 2 }
    fn square_number(n: i32) -> i32 { n * n }
    fn pentagonal_number(n: i32) -> i32 { n * (3 * n - 1) / 2 }
    fn hexagonal_number(n: i32) -> i32 { n * (2 * n - 1) }
    fn heptagonal_number(n: i32) -> i32 { n * (5 * n - 3) / 2 }
    fn octagonal_number(n: i32) -> i32 { n * (3 * n - 2) }
    fn first_two_digits(n: i32) -> i32 { n / 100 }
    fn last_two_digits(n: i32) -> i32 { n % 100 }

    fn find_cyclic_numbers(
        s: &mut Vec<i32>,
        used_flags: &mut [bool; 6],
        sequences: [&Vec<i32>; 6],
        min_limit: i32,
        max_limit: i32,
        max_sequence_length: usize,
    ) -> Option<i32> {
        if s.len() >= max_sequence_length {
            if NumberFunctions::first_two_digits(s[0]) == NumberFunctions::last_two_digits(s[s.len() - 1]) {
                return Some(s.iter().sum());
            }
            return None;
        }

        if s.is_empty() {
            for &number in sequences[0] {
                if min_limit <= number && number <= max_limit {
                    s.push(number);
                    used_flags[0] = true;
                    if let Some(result) = NumberFunctions::find_cyclic_numbers(s, used_flags, sequences, min_limit, max_limit, max_sequence_length) {
                        used_flags[0] = false;
                        s.pop();
                        return Some(result);
                    }
                    used_flags[0] = false;
                    s.pop();
                }
            }
        } else {
            let target = NumberFunctions::last_two_digits(s[s.len() - 1]);
            for index in 0..6 {
                if !used_flags[index] {
                    let mut flags_copy = *used_flags;
                    for &number in sequences[index] {
                        if NumberFunctions::first_two_digits(number) == target
                            && min_limit <= number && number <= max_limit
                        {
                            s.push(number);
                            flags_copy[index] = true;
                            if let Some(result) = NumberFunctions::find_cyclic_numbers(s, &mut flags_copy, sequences, min_limit, max_limit, max_sequence_length) {
                                flags_copy[index] = false;
                                s.pop();
                                return Some(result);
                            }
                            flags_copy[index] = false;
                            s.pop();
                        }
                    }
                }
            }
        }
        None
    }
}

fn solve() -> i32 {
    let min_limit = 1000;
    let max_limit = 9999;
    let max_sequence_length = 6;

    let mut triangle_numbers: Vec<i32> = Vec::new();
    let mut square_numbers: Vec<i32> = Vec::new();
    let mut pentagonal_numbers: Vec<i32> = Vec::new();
    let mut hexagonal_numbers: Vec<i32> = Vec::new();
    let mut heptagonal_numbers: Vec<i32> = Vec::new();
    let mut octagonal_numbers: Vec<i32> = Vec::new();

    for n in 1..=200 {
        let t3 = NumberFunctions::triangle_number(n);
        if min_limit <= t3 && t3 <= max_limit { triangle_numbers.push(t3); }
        let s4 = NumberFunctions::square_number(n);
        if min_limit <= s4 && s4 <= max_limit { square_numbers.push(s4); }
        let p5 = NumberFunctions::pentagonal_number(n);
        if min_limit <= p5 && p5 <= max_limit { pentagonal_numbers.push(p5); }
        let h6 = NumberFunctions::hexagonal_number(n);
        if min_limit <= h6 && h6 <= max_limit { hexagonal_numbers.push(h6); }
        let he7 = NumberFunctions::heptagonal_number(n);
        if min_limit <= he7 && he7 <= max_limit { heptagonal_numbers.push(he7); }
        let o8 = NumberFunctions::octagonal_number(n);
        if min_limit <= o8 && o8 <= max_limit { octagonal_numbers.push(o8); }
    }

    let mut sequence: Vec<i32> = Vec::new();
    let mut used_flags: [bool; 6] = [false; 6];
    let sequences: [&Vec<i32>; 6] = [
        &triangle_numbers, &square_numbers, &pentagonal_numbers,
        &hexagonal_numbers, &heptagonal_numbers, &octagonal_numbers,
    ];

    NumberFunctions::find_cyclic_numbers(
        &mut sequence, &mut used_flags, sequences,
        min_limit, max_limit, max_sequence_length,
    ).expect("no cyclic sequence found")
}

pe_utils::pe_main!();
