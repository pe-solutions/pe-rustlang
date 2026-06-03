// Magic 5-gon Ring
// https://projecteuler.net/problem=68

fn is_valid_ring(outer: &[i32], inner: &[i32]) -> bool {
    let line1 = outer[0] as u64 + inner[0] as u64 + inner[4] as u64;
    (outer[1] as u64 + inner[0] as u64 + inner[1] as u64 == line1)
        && (outer[2] as u64 + inner[1] as u64 + inner[2] as u64 == line1)
        && (outer[3] as u64 + inner[2] as u64 + inner[3] as u64 == line1)
        && (outer[4] as u64 + inner[3] as u64 + inner[4] as u64 == line1)
}

fn format_ring_string(outer: &[i32], inner: &[i32]) -> String {
    let mut s = String::new();
    for i in 0..5 {
        s.push_str(&outer[i].to_string());
        s.push_str(&inner[i].to_string());
        s.push_str(&inner[(i + 1) % 5].to_string());
    }
    s
}

fn solve() -> u64 {
    let mut max_str = String::new();

    let mut perm = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    loop {
        let outer = &perm[0..5];
        let inner = &perm[5..10];

        if is_valid_ring(outer, inner) {
            let s = format_ring_string(outer, inner);
            if s.len() == 16 && (max_str.is_empty() || s > max_str) {
                max_str = s;
            }
        }

        if !next_permutation(&mut perm) {
            break;
        }
    }

    max_str.parse().unwrap_or(0)
}

fn next_permutation(arr: &mut [i32]) -> bool {
    let len = arr.len();
    let mut i = len - 1;

    while i > 0 && arr[i - 1] >= arr[i] {
        i -= 1;
    }

    if i == 0 {
        return false;
    }

    let mut j = len - 1;
    while j > i - 1 && arr[j] <= arr[i - 1] {
        j -= 1;
    }

    arr.swap(i - 1, j);
    arr[i..].reverse();
    true
}

pe_utils::pe_main!();
