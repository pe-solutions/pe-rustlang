// Counting Sundays
// https://projecteuler.net/problem=19

use itertools::iproduct;
use chrono::{Datelike, NaiveDate, Weekday};

fn pe0019() -> usize {
    // Count the number of Sundays on the first of the month
    iproduct!(1901..2001, 1..=12)
        .filter(|&(year, month)| {
            NaiveDate::from_ymd_opt(year, month, 1)
                .map(|date| date.weekday() == Weekday::Sun)
                .unwrap_or(false)
        })
        .count()
}

fn solve() -> usize {
    pe0019()
}

fn main() {
    pe_utils::run(19, solve);
}
