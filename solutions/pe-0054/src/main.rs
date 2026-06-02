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

fn classify_hand(cards: &[Card; 5]) -> (u8, Vec<u8>) {
    let mut ranks = [0u8; 15];
    for card in cards {
        ranks[card.rank as usize] += 1;
    }

    let mut counts: Vec<u8> = ranks.iter().filter(|&&c| c > 0).copied().collect();
    counts.sort_by(|a, b| b.cmp(a));

    let is_flush = cards.iter().all(|c| c.suit == cards[0].suit);
    let is_straight = {
        let sorted: Vec<_> = cards.iter().map(|c| c.rank).collect();
        sorted[4] - sorted[0] == 4 && counts.len() == 5
    };

    let rank_counts: Vec<u8> = (0..=14)
        .rev()
        .filter_map(|i| if ranks[i] > 0 { Some(ranks[i]) } else { None })
        .collect();

    let hand_type = match (&rank_counts[..], is_straight, is_flush) {
        ([4, 1], _, _) => 7,           // Four of a kind
        ([3, 2], _, _) => 6,           // Full house
        (_, _, true) => 5,             // Flush
        (_, true, _) => 4,             // Straight
        ([3, 1, 1], _, _) => 3,        // Three of a kind
        ([2, 2, 1], _, _) => 2,        // Two pair
        ([2, 1, 1, 1], _, _) => 1,     // One pair
        _ => 0,                        // High card
    };

    let tiebreakers: Vec<u8> = (0..=14)
        .rev()
        .filter_map(|i| if ranks[i] > 0 { Some(ranks[i] * 16 + i as u8) } else { None })
        .collect();

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
