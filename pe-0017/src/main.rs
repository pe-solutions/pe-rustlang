// Number Letter Counts
// https://projecteuler.net/problem=17

use std::time::Instant;

fn number_to_english(n: u64) -> &'static str {
    match n {
        1 => "ONE",
        2 => "TWO",
        3 => "THREE",
        4 => "FOUR",
        5 => "FIVE",
        6 => "SIX",
        7 => "SEVEN",
        8 => "EIGHT",
        9 => "NINE",
        10 => "TEN",
        11 => "ELEVEN",
        12 => "TWELVE",
        13 => "THIRTEEN",
        14 => "FOURTEEN",
        15 => "FIFTEEN",
        16 => "SIXTEEN",
        17 => "SEVENTEEN",
        18 => "EIGHTEEN",
        19 => "NINETEEN",
        20 => "TWENTY",
        30 => "THIRTY",
        40 => "FORTY",
        50 => "FIFTY",
        60 => "SIXTY",
        70 => "SEVENTY",
        80 => "EIGHTY",
        90 => "NINETY",
        100 => "HUNDRED",
        1000 => "THOUSAND",
        1_000_000 => "MILLION",
        _ => "",
    }
}

fn main() {
    let start = Instant::now();
    // 

    let mut answer = 0;

    for num in 1..=1000 {
        let mut num = num;

        if num / 1000 > 0 {
            answer += number_to_english(num / 1000).len() + number_to_english(1000).len();
            num %= 1000;
        }

        if num / 100 > 0 {
            answer += number_to_english(num / 100).len() + number_to_english(100).len();
            num %= 100;

            if num % 100 != 0 {
                answer += "AND".len();
            }
        }

        if num >= 21 && num <= 99 {
            answer += number_to_english(num / 10 * 10).len();

            if num % 10 != 0 {
                answer += number_to_english(num % 10).len();
            }
        }

        if num >= 1 && num <= 20 {
            answer += number_to_english(num).len();
        }
    }

    let duration = start.elapsed();
    // 

    println!("\nProject Euler #17\nAnswer: {}", answer);

    println!("Elapsed time: {:?} milliseconds.\n", duration.as_millis());
}

