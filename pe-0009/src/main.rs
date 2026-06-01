// Special Pythagorean Triplet
// https://projecteuler.net/problem=9

const PERIMETER: i32 = 1000;

fn find_pythagorean_triplet() -> Option<(i32, i32, i32)> {
    for a in 1..(PERIMETER / 3) {
        for b in (a + 1)..(PERIMETER / 2) {
            let c = PERIMETER - a - b;
            //
            if a.pow(2) + b.pow(2) == c.pow(2) {
                return Some((a, b, c));
            }
        }
    }
    
    None
}

fn solve() -> i32 {
    let (a, b, c) = find_pythagorean_triplet().expect("no triplet found");
    a * b * c
}

fn main() {
    pe_utils::run(9, solve);
}
