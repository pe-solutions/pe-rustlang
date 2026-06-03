// Number Letter Counts
// https://projecteuler.net/problem=17


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

fn word_length_for_number(n: u64) -> usize {
    let mut length = 0;
    let mut num = n;

    if num / 1000 > 0 {
        length += number_to_english(num / 1000).len() + number_to_english(1000).len();
        num %= 1000;
    }
    if num / 100 > 0 {
        length += number_to_english(num / 100).len() + number_to_english(100).len();
        num %= 100;
        if num % 100 != 0 {
            length += "AND".len();
        }
    }
    if num >= 21 && num <= 99 {
        length += number_to_english(num / 10 * 10).len();
        if num % 10 != 0 {
            length += number_to_english(num % 10).len();
        }
    }
    if num >= 1 && num <= 20 {
        length += number_to_english(num).len();
    }
    length
}

fn solve() -> usize {
    (1..=1000).map(word_length_for_number).sum()
}

pe_utils::pe_main!();
