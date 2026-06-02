// Monopoly Odds
// https://projecteuler.net/problem=84

fn solve() -> u64 {
    let mut pos: usize = 0;
    let mut visits = vec![0; 40];
    let mut rng_state = 123456u64;

    for _ in 0..1_000_000 {
        for _ in 0..3 {
            let mut state = rng_state;
            state = state.wrapping_mul(1103515245).wrapping_add(12345);
            let die1 = 1 + ((state / 65536) % 6);
            state = state.wrapping_mul(1103515245).wrapping_add(12345);
            let die2 = 1 + ((state / 65536) % 6);
            rng_state = state;

            pos = (pos + (die1 + die2) as usize) % 40;

            if die1 != die2 {
                break;
            }
        }
        visits[pos] += 1;
    }

    let mut top3 = vec![(0, 0); 3];
    for (i, &v) in visits.iter().enumerate() {
        if v > top3[0].0 {
            top3[0] = (v, i);
            top3.sort_by(|a, b| b.0.cmp(&a.0));
        }
    }

    (top3[0].1 as u64) * 100 + (top3[1].1 as u64) * 10 + (top3[2].1 as u64)
}

pe_utils::pe_main!();
