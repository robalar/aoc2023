use std::{cmp::Ordering, collections::HashMap, fmt::Debug};

fn main() {
    let input = include_str!("input.txt");

    let mut hands = input.lines().map(Hand::from).collect::<Vec<_>>();

    hands.sort();

    let answer: usize = hands
        .iter()
        .enumerate()
        .map(|(rank, hand)| (rank + 1) * hand.bid)
        .sum();

    dbg!(answer);
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType {
    fn from_cards(cards: &[char]) -> HandType {
        let mut card_counts = HashMap::new();
        cards.iter().for_each(|c| {
            *card_counts.entry(c).or_insert(0) += 1;
        });

        let mut values = card_counts.values().collect::<Vec<_>>();
        values.sort();

        match values[..] {
            [5] => HandType::FiveOfAKind,
            [1, 4] => HandType::FourOfAKind,
            [2, 3] => HandType::FullHouse,
            [1, 1, 3] => HandType::ThreeOfAKind,
            [1, 2, 2] => HandType::TwoPair,
            [1, 1, 1, 2] => HandType::OnePair,
            [1, 1, 1, 1, 1] => HandType::HighCard,
            _ => {
                unreachable!()
            }
        }
    }
}

#[derive(Eq)]
struct Hand {
    cards: Vec<char>,
    hand_type: HandType,
    bid: usize,
}

impl From<&str> for Hand {
    fn from(value: &str) -> Self {
        let (cards, bid) = value.split_once(" ").expect("could not split line");
        let cards = cards.chars().collect::<Vec<_>>();
        let hand_type = HandType::from_cards(&cards);
        Self {
            cards,
            hand_type,
            bid: bid.parse().expect("could not parse bid"),
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Equal => {
                // If hand types are the same, order by value of card labels
                let label_values: HashMap<_, _> = "23456789TJQKA"
                    .chars()
                    .enumerate()
                    .map(|(i, c)| (c, i))
                    .collect();

                self.cards
                    .iter()
                    .zip(other.cards.iter())
                    .find_map(|(a, b)| {
                        let a_val = label_values.get(a);
                        let b_val = label_values.get(b);

                        match a_val.cmp(&b_val) {
                            Ordering::Equal => None,
                            o => Some(o),
                        }
                    })
                    .unwrap_or(Ordering::Equal)
            }
            o => o,
        }
    }
}
