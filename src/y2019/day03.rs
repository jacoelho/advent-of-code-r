use crate::grid::Position2D;
use crate::io;
use itertools::Itertools;
use std::collections::HashSet;
use std::iter::successors;

enum Direction {
    Up(usize),
    Down(usize),
    Left(usize),
    Right(usize),
}

impl std::str::FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s[1..].parse().expect("expected number");

        match &s[0..1] {
            "U" => Ok(Self::Up(value)),
            "D" => Ok(Self::Down(value)),
            "L" => Ok(Self::Left(value)),
            "R" => Ok(Self::Right(value)),
            _ => Err(format!("unexpected direction {s}")),
        }
    }
}

impl Direction {
    fn expand(&self, p: Position2D) -> Vec<Position2D> {
        let (step, count) = match self {
            Self::Up(count) => (Position2D::new(0, 1), count),
            Self::Down(count) => (Position2D::new(0, -1), count),
            Self::Left(count) => (Position2D::new(-1, 0), count),
            Self::Right(count) => (Position2D::new(1, 0), count),
        };

        successors(Some(p + step), |&point| Some(point + step))
            .take(*count)
            .collect()
    }
}

fn parse_input(path: &str) -> Vec<Vec<Direction>> {
    io::read_value_per_line::<String>(path)
        .iter()
        .map(|v| {
            v.split(',')
                .filter_map(|line| line.parse::<Direction>().ok())
                .collect()
        })
        .collect()
}

fn extend(wire: &[Direction]) -> Vec<Position2D> {
    wire.iter().fold(vec![Position2D::new(0, 0)], |mut acc, d| {
        let p = acc.last().expect("expected point");
        let mut positions = d.expand(*p);

        acc.append(&mut positions);

        acc
    })
}

fn part01(path: &str) -> usize {
    let input = parse_input(path);
    let start = Position2D::new(0, 0);
    let one =
        extend(&input[0]).into_iter().skip(1).collect::<HashSet<Position2D>>();
    let two =
        extend(&input[1]).into_iter().skip(1).collect::<HashSet<Position2D>>();

    one.intersection(&two)
        .map(|p| p.distance(&start))
        .min()
        .expect("value expected")
}

fn part02(path: &str) -> usize {
    let input = parse_input(path);

    let one = extend(&input[0]);
    let two = extend(&input[1]);

    let one_set = one.clone().into_iter().collect::<HashSet<Position2D>>();
    let two_set = two.clone().into_iter().collect::<HashSet<Position2D>>();

    one_set
        .intersection(&two_set)
        .map(|&p| {
            one.iter().find_position(|&el| el == &p).expect("expected").0
                + two.iter().find_position(|&el| el == &p).expect("expected").0
        })
        .filter(|&v| v > 2)
        .min()
        .expect("value expected")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part01_input() {
        assert_eq!(part01("data/y2019/day03.txt"), 386);
    }

    #[test]
    fn part02_input() {
        assert_eq!(part02("data/y2019/day03.txt"), 6484);
    }
}
