use std::collections::{HashMap, VecDeque};
use std::str::FromStr;

fn hash(s: &str) -> u32 {
    s.chars().fold(0, |acc, ch| ((acc + ch as u32) * 17) % 256)
}

fn part01(path: &str) -> u32 {
    std::fs::read_to_string(path)
        .expect("expected file")
        .lines()
        .map(|line| line.split(',').map(hash).sum::<u32>())
        .sum()
}

enum Instruction {
    Remove(usize, String),
    Upsert(usize, String, usize),
}

impl FromStr for Instruction {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.ends_with('-') {
            let label = s.trim_end_matches('-');

            Ok(Self::Remove(hash(label) as usize, label.to_string()))
        } else {
            let (label, focal) = s.split_once('=').expect("should work");

            Ok(Self::Upsert(
                hash(label) as usize,
                label.to_string(),
                focal.parse().expect("should be a number"),
            ))
        }
    }
}

fn part02(path: &str) -> usize {
    let instructions = std::fs::read_to_string(path)
        .expect("expected file")
        .lines()
        .map(|line| {
            line.split(',')
                .filter_map(|s| s.parse::<Instruction>().ok())
                .collect::<Vec<_>>()
        })
        .next()
        .expect("expected one line");

    let mut boxes: [VecDeque<(String, usize)>; 256] =
        core::array::from_fn(|_| VecDeque::new());

    for instruction in instructions {
        match instruction {
            Instruction::Remove(box_number, label) => {
                if let Some(index) =
                    boxes[box_number].iter().position(|(s, _)| s == &label)
                {
                    boxes[box_number].remove(index);
                }
            }
            Instruction::Upsert(box_number, label, focal) => {
                if let Some(index) =
                    boxes[box_number].iter().position(|(s, _)| *s == label)
                {
                    boxes[box_number][index] = (label, focal);
                } else {
                    boxes[box_number].push_back((label, focal));
                }
            }
        }
    }

    boxes.iter().enumerate().fold(0, |acc, (box_number, lenses)| {
        acc + lenses.iter().enumerate().fold(
            0,
            |inner_acc, (lens_number, lens)| {
                inner_acc + (box_number + 1) * (lens_number + 1) * lens.1
            },
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part01_example1() {
        assert_eq!(part01("data/y2023/day15-example1.txt"), 1320);
    }

    #[test]
    fn part01_input() {
        assert_eq!(part01("data/y2023/day15.txt"), 516_469);
    }

    #[test]
    fn part02_example1() {
        assert_eq!(part02("data/y2023/day15-example1.txt"), 145);
    }

    #[test]
    fn part02_input() {
        assert_eq!(part02("data/y2023/day15.txt"), 221_627);
    }
}
