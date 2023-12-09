use std::{cmp::Ordering, collections::HashMap};
use utils::read_lines;

type Card = char;
type CardWeight = u8;
type Bid = u64;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
enum HandType {
    HighCard = 0,
    OnePair = 1,
    TwoPair = 2,
    ThreeOfAKind = 3,
    FullHouse = 4,
    FourOfAKind = 5,
    FiveOfAKind = 6,
}

#[derive(Debug)]
struct Hand {
    pub cards: Vec<Card>,
}

impl Hand {
    const POSSIBLE_CARDS: [char; 12] =
        ['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A'];

    pub fn get_type(&self, is_joker_supported: bool) -> HandType {
        if !is_joker_supported {
            Self::calc_type(&self.cards)
        } else {
            let jokers_idx = self
                .cards
                .iter()
                .enumerate()
                .filter_map(|(i, c)| if *c == 'J' { Some(i) } else { None })
                .collect::<Vec<_>>();
            if jokers_idx.is_empty() {
                Self::calc_type(&self.cards)
            } else {
                let mut max_type = HandType::HighCard;
                for card in Self::POSSIBLE_CARDS {
                    let mut cards = self.cards.clone();
                    // it only makes sense to replace jokers with the same card
                    for joker_idx in &jokers_idx {
                        cards[*joker_idx] = card;
                    }
                    let hand_type = Self::calc_type(&cards);
                    if hand_type > max_type {
                        max_type = hand_type;
                    }
                }
                max_type
            }
        }
    }

    fn calc_type(cards: &[Card]) -> HandType {
        let mut counts = HashMap::new();
        for card in cards {
            let count = counts.entry(*card).or_insert(0);
            *count += 1;
        }
        let mut counts = counts.values().collect::<Vec<_>>();
        counts.sort();

        match counts.as_slice() {
            [1, 1, 1, 1, 1] => HandType::HighCard,
            [1, 1, 1, 2] => HandType::OnePair,
            [1, 2, 2] => HandType::TwoPair,
            [1, 1, 3] => HandType::ThreeOfAKind,
            [2, 3] => HandType::FullHouse,
            [1, 4] => HandType::FourOfAKind,
            [5] => HandType::FiveOfAKind,
            _ => panic!("Invalid hand"),
        }
    }
}

fn main() {
    let mut hands = Vec::new();
    for line in read_lines("day7/input.txt").unwrap() {
        let line = line.unwrap();
        hands.push(parse_hand(&line));
    }

    let rules1: HashMap<Card, CardWeight> = [
        ('2', 2),
        ('3', 3),
        ('4', 4),
        ('5', 5),
        ('6', 6),
        ('7', 7),
        ('8', 8),
        ('9', 9),
        ('T', 10),
        ('J', 11),
        ('Q', 12),
        ('K', 13),
        ('A', 14),
    ]
    .into_iter()
    .collect();
    hands.sort_by(|(h1, _), (h2, _)| compare(h1, h2, &rules1, false));

    let mut winnings: u64 = 0;
    for (i, (_, bid)) in hands.iter().enumerate() {
        winnings += (i as u64 + 1) * bid
    }

    println!("Winnings 1: {winnings}");

    let rules2: HashMap<Card, CardWeight> = {
        let mut cards = rules1.clone();
        cards.insert('J', 1);
        cards
    };
    hands
        .sort_by(|(hand1, _), (hand2, _)| compare(hand1, hand2, &rules2, true));

    let mut winnings: u64 = 0;
    for (i, (_, bid)) in hands.iter().enumerate() {
        winnings += (i as u64 + 1) * bid
    }

    println!("Winnings 2: {winnings}");
}

fn compare(
    hand1: &Hand,
    hand2: &Hand,
    rules: &HashMap<Card, CardWeight>,
    is_joker_supported: bool,
) -> Ordering {
    match hand1
        .get_type(is_joker_supported)
        .cmp(&hand2.get_type(is_joker_supported))
    {
        Ordering::Equal => compare_cards(&hand1.cards, &hand2.cards, rules),
        other => other,
    }
}

fn compare_cards(
    cards1: &[Card],
    cards2: &[Card],
    rules: &HashMap<Card, CardWeight>,
) -> Ordering {
    weights(cards1, rules).cmp(&weights(cards2, rules))
}

fn weights(
    cards: &[Card],
    rules: &HashMap<Card, CardWeight>,
) -> Vec<CardWeight> {
    cards.iter().map(|c| rules[c]).collect()
}

fn parse_hand(s: &str) -> (Hand, Bid) {
    s.split_once(' ')
        .map(|(cards, bid)| {
            (
                Hand {
                    cards: cards.chars().collect(),
                },
                bid.parse().unwrap(),
            )
        })
        .unwrap()
}
