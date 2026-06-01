// Integer Right Triangles
// https://projecteuler.net/problem=39

fn solve() -> usize {
    let (max_perimeter, _) = (12..=1000).fold((0, 0), |(max_p, max_count), p| {
        let count = (2..p / 2)
            .flat_map(|b| (1..b).filter(move |&a| a * a + b * b == (p - a - b) * (p - a - b)))
            .count();
        if count > max_count { (p, count) } else { (max_p, max_count) }
    });
    max_perimeter
}

pe_utils::pe_main!();
