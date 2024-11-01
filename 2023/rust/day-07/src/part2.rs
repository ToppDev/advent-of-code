use std::{collections::HashMap, usize};

use itertools::Itertools;

use crate::custom_error::AocError;

const CARDS: [char; 13] = [
    'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
];

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Eq)]
struct Game {
    hand: Vec<Card>,
    hand_type: HandType,
    bid: u32,
}

#[derive(Debug, PartialEq, Eq)]
struct Card(char);

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Game {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }

        self.hand.cmp(&other.hand)
    }
}

impl PartialOrd for Game {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Game {
    fn eq(&self, other: &Self) -> bool {
        self.hand == other.hand
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let idx = CARDS.iter().position(|&x| x == self.0).unwrap();
        let idx_other = CARDS.iter().position(|&x| x == other.0).unwrap();
        idx.cmp(&idx_other)
    }
}

fn determine_type(hand: &str) -> HandType {
    let mut card_duplicates = HashMap::new();

    for card in hand.chars() {
        *card_duplicates.entry(card).or_insert(0) += 1;
    }
    let mut cards: Vec<(&char, i32)> = card_duplicates
        .iter()
        .sorted_by(|a, b| {
            match Ord::cmp(&b.1, &a.1) {
                core::cmp::Ordering::Equal => {}
                ord => return ord,
            }

            Ord::cmp(&Card(*b.0), &Card(*a.0))
        })
        .map(|x| (x.0, *x.1))
        .collect_vec();

    // Add jokers to highest card and remove them
    if let Some(i) = cards.iter().position(|&x| *x.0 == 'J') {
        if cards.len() == 1 {
            return HandType::FiveOfAKind; // JJJJJ
        }

        if *cards[0].0 == 'J' {
            cards[1].1 += cards[0].1;
            cards.remove(0);
        } else {
            cards[0].1 += cards[i].1;
            cards.remove(i);
        }
    }

    match cards.len() {
        1 => HandType::FiveOfAKind,
        2 => match (cards[0].1, cards[1].1) {
            (4, 1) => HandType::FourOfAKind,
            (3, 2) => HandType::FullHouse,
            _ => panic!("All combinations with sum 5 should be covered"),
        },
        3 => match (cards[0].1, cards[1].1, cards[2].1) {
            (3, 1, 1) => HandType::ThreeOfAKind,
            (2, 2, 1) => HandType::TwoPair,
            _ => panic!("All combinations with sum 5 should be covered"),
        },
        4 => HandType::OnePair,
        _ => HandType::HighCard,
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let games = input
        .lines()
        .map(|l| {
            let (hand, bid) = l.split_once(' ').unwrap();
            Game {
                hand: hand.chars().map(Card).collect_vec(),
                hand_type: determine_type(hand),
                bid: bid.parse::<u32>().unwrap(),
            }
        })
        .sorted()
        .collect_vec();

    Ok(games
        .iter()
        .enumerate()
        .map(|(rank, game)| (rank + 1) * game.bid as usize)
        .sum::<usize>()
        .to_string())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        assert_eq!(process(input)?, "5905");
        Ok(())
    }
}