use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Copy, Clone)]
enum Card {
    Joker = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    Ace = 14,
}

impl TryFrom<char> for Card {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Self::Ace),
            'K' => Ok(Self::King),
            'Q' => Ok(Self::Queen),
            'J' => Ok(Self::Jack),
            'T' => Ok(Self::Ten),
            '9' => Ok(Self::Nine),
            '8' => Ok(Self::Eight),
            '7' => Ok(Self::Seven),
            '6' => Ok(Self::Six),
            '5' => Ok(Self::Five),
            '4' => Ok(Self::Four),
            '3' => Ok(Self::Three),
            '2' => Ok(Self::Two),
            _ => Err("unexpected card"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd)]
enum Value {
    HighCard = 1,
    OnePair = 2,
    TwoPair = 3,
    ThreeOfAKind = 4,
    FullHouse = 5,
    FourOfAKind = 6,
    FiveOfAKind = 7,
}

#[derive(Debug, Clone)]
struct Hand(Vec<Card>);

impl Hand {
    fn value(&self) -> Value {
        let count = self.0.iter().counts();

        match *count.values().max().unwrap_or(&0) {
            5 => Value::FiveOfAKind,
            4 => Value::FourOfAKind,
            3 => {
                if count.values().any(|&v| v == 2) {
                    Value::FullHouse
                } else {
                    Value::ThreeOfAKind
                }
            }
            2 => {
                if count.values().filter(|&&v| v == 2).count() == 2 {
                    Value::TwoPair
                } else {
                    Value::OnePair
                }
            }
            _ => Value::HighCard,
        }
    }

    fn value_wildcard(&self) -> Value {
        let value = self.value();
        let counts = self.0.iter().counts();
        let jokers_count = counts.get(&Card::Joker).unwrap_or(&0);
        let without_joker = counts
            .iter()
            .filter(|&(&k, _)| *k != Card::Joker)
            .map(|(_, v)| *v)
            .collect::<Vec<_>>();

        match (*without_joker.iter().max().unwrap_or(&0), jokers_count) {
            (_, 0) => value,
            (a, b) if a + b == 5 => Value::FiveOfAKind,
            (a, b) if a + b == 4 => Value::FourOfAKind,
            (2, _) => {
                let pairs = without_joker.iter().filter(|&&v| v == 2).count();
                match pairs {
                    2 => Value::FullHouse,
                    1 => Value::ThreeOfAKind,
                    _ => panic!("not expected"),
                }
            }
            (1, 2) => Value::ThreeOfAKind,
            (1, 1) => Value::OnePair,
            _ => panic!("not reachable")
            // 0 => value,
            // 1 => match value {
            //     Value::HighCard => Value::OnePair,
            //     Value::OnePair => Value::ThreeOfAKind,
            //     Value::TwoPair => Value::FullHouse,
            //     Value::ThreeOfAKind => Value::FourOfAKind,
            //     Value::FourOfAKind => Value::FiveOfAKind,
            //     _ => panic!("not expected {:?} {:?}", self, value),
            // },
            // 2 => match value {
            //     Value::HighCard => Value::ThreeOfAKind,
            //     Value::OnePair | Value::TwoPair => Value::FourOfAKind,
            //     Value::ThreeOfAKind | Value::FullHouse => Value::FiveOfAKind,
            //     _ => panic!("not expected {:?} {:?}", self, value),
            // },
            // 3 => match value {
            //     Value::ThreeOfAKind | Value::HighCard => Value::FourOfAKind,
            //     Value::TwoPair | Value::FullHouse => Value::FiveOfAKind,
            //     _ => panic!("not expected {:?} {:?}", self, value),
            // },
            // 4 | 5 => Value::FiveOfAKind,
            // _ => panic!("not expected {:?} {:?}", self, value),
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other).unwrap() == std::cmp::Ordering::Equal
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let lhs = self.value_wildcard();
        let rhs = other.value_wildcard();

        if lhs == rhs {
            if let Some((a, b)) =
                self.0.iter().zip(other.0.iter()).find(|(a, b)| a != b)
            {
                return Some(if a > b {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Less
                });
            }

            return Some(std::cmp::Ordering::Equal);
        }

        Some(lhs.cmp(&rhs))
    }
}

fn parse_input(path: &str) -> Vec<(Hand, u32)> {
    std::fs::read_to_string(path)
        .expect("should be able to read file")
        .lines()
        .map(|line| {
            line.split_once(' ')
                .map(|(hand, bid)| {
                    (
                        Hand(
                            hand.chars()
                                .filter_map(|ch| Card::try_from(ch).ok())
                                .collect::<Vec<_>>(),
                        ),
                        bid.trim().parse().unwrap(),
                    )
                })
                .expect("expected values")
        })
        .collect::<Vec<_>>()
}

fn part01(path: &str) -> u32 {
    let mut hands = parse_input(path);

    hands.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    hands
        .iter()
        .enumerate()
        .map(|(rank, hand)| (u32::try_from(rank).expect("value") + 1) * hand.1)
        .sum()
}

fn part02(path: &str) -> u32 {
    let mut hands = parse_input(path)
        .iter()
        .map(|(card, bid)| {
            (
                Hand(
                    card.0
                        .iter()
                        .map(|card| match card {
                            Card::Jack => Card::Joker,
                            _ => *card,
                        })
                        .collect::<Vec<_>>(),
                ),
                *bid,
            )
        })
        .collect::<Vec<_>>();

    hands.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    hands
        .iter()
        .enumerate()
        .map(|(rank, hand)| (u32::try_from(rank).expect("value") + 1) * hand.1)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part01_example() {
        assert_eq!(part01("data/y2023/day07-example1.txt"), 6440);
    }

    #[test]
    fn part01_input() {
        assert_eq!(part01("data/y2023/day07.txt"), 253_954_294);
    }

    #[test]
    fn part02_example() {
        assert_eq!(part02("data/y2023/day07-example1.txt"), 5905);
    }

    #[test]
    fn part02_input() {
        assert_eq!(part02("data/y2023/day07.txt"), 254_837_398);
    }
}
