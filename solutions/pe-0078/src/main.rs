// Coin Partitions
// https://projecteuler.net/problem=78

fn solve() -> u64 {
    const MOD: u64 = 1_000_000;
    let mut p = vec![0u64; 100001];
    p[0] = 1;
    
    for n in 1..=100000 {
        for k in n..=100000 {
            p[k] = (p[k] + p[k - n]) % MOD;
        }
        if p[n] == 0 {
            return n as u64;
        }
    }
    0
}

pe_utils::pe_main!();
