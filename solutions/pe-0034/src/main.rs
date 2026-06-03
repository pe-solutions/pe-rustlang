use pe_lib::digit_factorial_sum;

fn solve() -> u32 {
    let s_of_fac_pred = |n: u32| digit_factorial_sum(n as u64) as u32 == n;
    let mut answer = 0;
    for n in 3..10_000_000 {
        if s_of_fac_pred(n) { answer += n; }
    }
    answer
}

pe_utils::pe_main!();
