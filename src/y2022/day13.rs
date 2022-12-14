use std::str::FromStr;

use crate::io;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Packet {
    Integer(usize),
    List(Vec<Packet>),
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Self::Integer(x), Self::Integer(y)) => x.partial_cmp(y),
            (Self::List(x), Self::List(y)) => x.partial_cmp(y),
            (Self::Integer(_), Self::List(_)) => Self::List(vec![self.clone()]).partial_cmp(other),
            (Self::List(_), Self::Integer(_)) => self.partial_cmp(&Self::List(vec![other.clone()])),
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl FromStr for Packet {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let p = if s.starts_with('[') {
            let s = &s[1..(s.len() - 1)];

            let mut items = s
                .char_indices()
                .fold((0, vec![0]), |(depth, mut acc), (i, c)| match (depth, c) {
                    (_, '[') => (depth + 1, acc),
                    (_, ']') => (depth - 1, acc),
                    (0, ',') => {
                        acc.push(i + 1);
                        (depth, acc)
                    }
                    _ => (depth, acc),
                });

            items.1.extend(vec![s.len() + 1]);

            Self::List(
                items
                    .1
                    .windows(2)
                    .filter_map(|idx| s[idx[0]..idx[1] - 1].parse().ok())
                    .collect::<Vec<_>>(),
            )
        } else if s.is_empty() {
            Self::List(vec![])
        } else {
            Self::Integer(s.parse().unwrap())
        };

        Ok(p)
    }
}

fn part01(path: &str) -> usize {
    io::read_value_chunks::<Packet>(path)
        .iter()
        .enumerate()
        .filter_map(|(i, pair)| {
            if pair[0] <= pair[1] {
                Some(i + 1)
            } else {
                None
            }
        })
        .sum()
}

fn part02(path: &str) -> usize {
    let mut packets = io::read_value_chunks::<Packet>(path)
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();

    let divider_1 = "[[2]]".parse::<Packet>().unwrap();
    let divider_2 = "[[6]]".parse::<Packet>().unwrap();

    packets.extend(vec![divider_1.clone(), divider_2.clone()]);

    packets.sort();

    packets
        .iter()
        .enumerate()
        .filter_map(|(i, p)| {
            if p == &divider_1 || p == &divider_2 {
                Some(i + 1)
            } else {
                None
            }
        })
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_pair_2() {
        let lhs = "[[1],[2,3,4]]".parse::<Packet>().unwrap();
        let rhs = "[[1],4]".parse::<Packet>().unwrap();

        assert!(lhs <= rhs);
    }

    #[test]
    fn part01_example() {
        assert_eq!(part01("data/y2022/day13-example.txt"), 13);
    }

    #[test]
    fn part01_input() {
        assert_eq!(part01("data/y2022/day13.txt"), 5393);
    }

    #[test]
    fn part02_example() {
        assert_eq!(part02("data/y2022/day13-example.txt"), 140);
    }

    #[test]
    fn part02_input() {
        assert_eq!(part02("data/y2022/day13.txt"), 26712);
    }
}
