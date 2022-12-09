use crate::io;
use std::{collections::HashSet, str::FromStr};

#[derive(Debug, PartialEq, Clone)]
enum Move {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
}

impl Move {
    fn offset(&self) -> (i32, i32) {
        match self {
            Move::Up(_) => (0, 1),
            Move::Down(_) => (0, -1),
            Move::Left(_) => (-1, 0),
            Move::Right(_) => (1, 0),
        }
    }

    fn value(&self) -> i32 {
        match *self {
            Move::Up(v) => v,
            Move::Down(v) => v,
            Move::Left(v) => v,
            Move::Right(v) => v,
        }
    }
}

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_digit(s: &str) -> i32 {
            let (_, d) = s.split_once(' ').unwrap();

            d.parse::<i32>().unwrap()
        }

        let res = match s {
            i if i.starts_with('U') => Self::Up(parse_digit(s)),
            i if i.starts_with('D') => Self::Down(parse_digit(s)),
            i if i.starts_with('L') => Self::Left(parse_digit(s)),
            i if i.starts_with('R') => Self::Right(parse_digit(s)),
            _ => panic!("unreachble"),
        };

        Ok(res)
    }
}

type Position = (i32, i32);

fn add_assign_tuple(lhs: &mut Position, rhs: Position) {
    lhs.0 += rhs.0;
    lhs.1 += rhs.1;
}

fn is_connected(lhs: &Position, rhs: &Position) -> bool {
    let dx = if lhs.0 > rhs.0 {
        lhs.0 - rhs.0
    } else {
        rhs.0 - lhs.0
    };

    let dy = if lhs.1 > rhs.1 {
        lhs.1 - rhs.1
    } else {
        rhs.1 - lhs.1
    };

    dx <= 1 && dy <= 1
}

#[derive(Debug, PartialEq)]
struct Rope {
    knots: Vec<Position>,
    visited: HashSet<Position>,
}

impl Rope {
    fn new(knots: usize) -> Self {
        Self {
            knots: vec![(0, 0); knots],
            visited: HashSet::new(),
        }
    }

    fn follow(&mut self) {
        for i in 1..self.knots.len() {
            if !is_connected(&self.knots[i], &self.knots[i - 1]) {
                let dx = self.knots[i - 1].0 - self.knots[i].0;
                let dy = self.knots[i - 1].1 - self.knots[i].1;

                self.knots[i] = (self.knots[i].0 + dx.signum(), self.knots[i].1 + dy.signum());
            }
        }
    }

    fn motion(&mut self, m: Move) {
        let offset = m.offset();

        for _ in 0..m.value() {
            add_assign_tuple(&mut self.knots[0], offset);

            self.follow();

            self.visited.insert(self.knots[self.knots.len() - 1]);
        }
    }
}

fn part01(filename: &str) -> usize {
    let foo = io::read_value_per_line::<Move>(filename);

    foo.into_iter()
        .fold(Rope::new(2), |mut r, m| {
            r.motion(m);
            r
        })
        .visited
        .len()
}

fn part02(filename: &str) -> usize {
    let foo = io::read_value_per_line::<Move>(filename);

    foo.into_iter()
        .fold(Rope::new(10), |mut r, m| {
            r.motion(m);
            r
        })
        .visited
        .len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part01_example() {
        assert_eq!(part01("data/y2022/day09-example.txt"), 13);
    }

    #[test]
    fn part01_input() {
        assert_eq!(part01("data/y2022/day09.txt"), 6269);
    }

    #[test]
    fn part02_example() {
        assert_eq!(part02("data/y2022/day09-example.txt"), 1);
    }

    #[test]
    fn part02_input() {
        assert_eq!(part02("data/y2022/day09.txt"), 2557);
    }
}
