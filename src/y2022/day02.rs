use crate::io;
use crate::y2022::day02::GameResult::{Draw, Lose, Win};
use crate::y2022::day02::Shape::{Paper, Rock, Scissors};
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl FromStr for Shape {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let shape = match s {
            "A" | "X" => Rock,
            "B" | "Y" => Paper,
            "C" | "Z" => Scissors,
            _ => return Err(()),
        };

        Ok(shape)
    }
}

#[derive(Debug, PartialEq)]
struct Round(Shape, Shape);

impl Round {
    fn score(&self) -> i32 {
        match (&self.0, &self.1) {
            (Rock, Rock) => 1 + 3,
            (Rock, Paper) => 2 + 6,
            (Rock, Scissors) => 3,

            (Paper, Rock) => 1,
            (Paper, Paper) => 2 + 3,
            (Paper, Scissors) => 3 + 6,

            (Scissors, Rock) => 1 + 6,
            (Scissors, Paper) => 2,
            (Scissors, Scissors) => 3 + 3,
        }
    }
}

impl FromStr for Round {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((a, b)) = s.split_once(' ') {
            let left = a.parse::<Shape>().expect("expected shape");
            let right = b.parse::<Shape>().expect("expected shape");

            Ok(Round(left, right))
        } else {
            Err(())
        }
    }
}

#[derive(Debug, PartialEq)]
enum GameResult {
    Lose,
    Draw,
    Win,
}

impl FromStr for GameResult {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result = match s {
            "X" => Lose,
            "Y" => Draw,
            "Z" => Win,
            _ => return Err(()),
        };

        Ok(result)
    }
}

#[derive(Debug, PartialEq)]
struct FixedScore(Shape, GameResult);

impl FixedScore {
    fn score(&self) -> i32 {
        let r = match (&self.0, &self.1) {
            (Rock, Lose) => Round(Rock, Scissors),
            (Rock, Draw) => Round(Rock, Rock),
            (Rock, Win) => Round(Rock, Paper),

            (Paper, Lose) => Round(Paper, Rock),
            (Paper, Draw) => Round(Paper, Paper),
            (Paper, Win) => Round(Paper, Scissors),

            (Scissors, Lose) => Round(Scissors, Paper),
            (Scissors, Draw) => Round(Scissors, Scissors),
            (Scissors, Win) => Round(Scissors, Rock),
        };
        r.score()
    }
}

impl FromStr for FixedScore {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((a, b)) = s.split_once(' ') {
            let shape = a.parse::<Shape>().expect("expect shape");
            let result = b.parse::<GameResult>().expect("result");

            Ok(FixedScore(shape, result))
        } else {
            Err(())
        }
    }
}

fn part01(path: &str) -> i32 {
    io::read_value_per_line::<Round>(path)
        .iter()
        .map(|r| r.score())
        .sum()
}

fn part02(path: &str) -> i32 {
    io::read_value_per_line::<FixedScore>(path)
        .iter()
        .map(|r| r.score())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part01_example() {
        assert_eq!(part01("data/y2022/day02-example.txt"), 15);
    }

    #[test]
    fn part01_input() {
        assert_eq!(part01("data/y2022/day02.txt"), 11_475);
    }

    #[test]
    fn part02_example() {
        assert_eq!(part02("data/y2022/day02-example.txt"), 12);
    }

    #[test]
    fn part02_input() {
        assert_eq!(part02("data/y2022/day02.txt"), 16862);
    }
}
