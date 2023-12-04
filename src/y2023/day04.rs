use crate::io;
use std::cmp::min;
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct ScratchCard {
    id: u32,
    matched_count: usize,
}

impl FromStr for ScratchCard {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split_once(": ").ok_or("invalid input").and_then(|(id, numbers)| {
            let id =
                id.split_once(' ').ok_or("invalid input").map(|(_, id)| {
                    id.trim().parse::<u32>().expect("valid number")
                })?;

            let matched_count = numbers
                .split_once(" | ")
                .ok_or("invalid input")
                .map(|(left, right)| {
                    let winning: HashSet<_> = left
                        .split_whitespace()
                        .filter_map(|s| s.parse::<u32>().ok())
                        .collect();
                    let numbers: HashSet<_> = right
                        .split_whitespace()
                        .filter_map(|s| s.parse::<u32>().ok())
                        .collect();
                    winning.intersection(&numbers).count()
                })?;

            Ok(Self { id, matched_count })
        })
    }
}

fn part01(path: &str) -> i32 {
    io::read_value_per_line::<ScratchCard>(path)
        .iter()
        .map(|card| {
            if card.matched_count > 0 {
                1 << (card.matched_count - 1)
            } else {
                0
            }
        })
        .sum()
}

fn part02(path: &str) -> i32 {
    let scratch_cards = io::read_value_per_line::<ScratchCard>(path);

    scratch_cards
        .iter()
        .enumerate()
        .fold(vec![1; scratch_cards.len()], |mut count, (idx, card)| {
            let matches = card.matched_count;

            for x in idx + 1..min(idx + 1 + matches, count.len()) {
                count[x] += count[idx];
            }

            count
        })
        .iter()
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part01_example() {
        assert_eq!(part01("data/y2023/day04-example1.txt"), 13);
    }

    #[test]
    fn part01_input() {
        assert_eq!(part01("data/y2023/day04.txt"), 21088);
    }

    #[test]
    fn part02_example() {
        assert_eq!(part02("data/y2023/day04-example1.txt"), 30);
    }

    #[test]
    fn part02_input() {
        assert_eq!(part02("data/y2023/day04.txt"), 6_874_754);
    }
}
