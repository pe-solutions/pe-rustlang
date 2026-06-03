// Poker Hands
// https://projecteuler.net/problem=54

use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Card {
    rank: u8,
    suit: char,
}

impl Card {
    fn from_str(s: &str) -> Self {
        let rank = match s.chars().next().unwrap() {
            '2'..='9' => s.chars().next().unwrap() as u8 - b'0',
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => 0,
        };
        Card {
            rank,
            suit: s.chars().nth(1).unwrap(),
        }
    }
}

fn get_rank_counts(cards: &[Card; 5]) -> [u8; 15] {
    let mut ranks = [0u8; 15];
    for card in cards {
        ranks[card.rank as usize] += 1;
    }
    ranks
}

fn is_flush(cards: &[Card; 5]) -> bool {
    cards.iter().all(|c| c.suit == cards[0].suit)
}

fn is_straight(cards: &[Card; 5], rank_counts: &[u8; 15]) -> bool {
    let sorted: Vec<_> = cards.iter().map(|c| c.rank).collect();
    let has_five_distinct = (0..=14).filter(|&i| rank_counts[i] > 0).count() == 5;
    sorted[4] - sorted[0] == 4 && has_five_distinct
}

fn determine_hand_type(rank_counts: &[u8; 15], is_straight: bool, is_flush: bool) -> u8 {
    let rank_pattern: Vec<u8> = (0..=14)
        .rev()
        .filter_map(|i| if rank_counts[i] > 0 { Some(rank_counts[i]) } else { None })
        .collect();

    match (&rank_pattern[..], is_straight, is_flush) {
        ([4, 1], _, _) => 7,           // Four of a kind
        ([3, 2], _, _) => 6,           // Full house
        (_, _, true) => 5,             // Flush
        (_, true, _) => 4,             // Straight
        ([3, 1, 1], _, _) => 3,        // Three of a kind
        ([2, 2, 1], _, _) => 2,        // Two pair
        ([2, 1, 1, 1], _, _) => 1,     // One pair
        _ => 0,                        // High card
    }
}

fn get_tiebreakers(rank_counts: &[u8; 15]) -> Vec<u8> {
    (0..=14)
        .rev()
        .filter_map(|i| if rank_counts[i] > 0 { Some(rank_counts[i] * 16 + i as u8) } else { None })
        .collect()
}

fn classify_hand(cards: &[Card; 5]) -> (u8, Vec<u8>) {
    let rank_counts = get_rank_counts(cards);
    let flush = is_flush(cards);
    let straight = is_straight(cards, &rank_counts);
    let hand_type = determine_hand_type(&rank_counts, straight, flush);
    let tiebreakers = get_tiebreakers(&rank_counts);

    (hand_type, tiebreakers)
}

fn solve() -> u64 {
    let content = fs::read_to_string("data/0054_poker.txt").unwrap();
    let mut player1_wins = 0;

    for line in content.lines() {
        let cards: Vec<&str> = line.split_whitespace().collect();
        let mut hand1 = [Card::from_str(""); 5];
        let mut hand2 = [Card::from_str(""); 5];

        for i in 0..5 {
            hand1[i] = Card::from_str(cards[i]);
            hand2[i] = Card::from_str(cards[i + 5]);
        }

        hand1.sort_by(|a, b| b.rank.cmp(&a.rank));
        hand2.sort_by(|a, b| b.rank.cmp(&a.rank));

        let (type1, tie1) = classify_hand(&hand1);
        let (type2, tie2) = classify_hand(&hand2);

        if type1 > type2 || (type1 == type2 && tie1 > tie2) {
            player1_wins += 1;
        }
    }

    player1_wins
}

pe_utils::pe_main!();
