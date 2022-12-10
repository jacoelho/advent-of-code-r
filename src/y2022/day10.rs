use std::{str::FromStr, vec};

use crate::io;

#[derive(Debug, PartialEq, Clone)]
enum Instruction {
    Noop,
    Addx(i32),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = match s {
            "noop" => Self::Noop,
            _ => {
                let (_, d) = s.split_once(' ').unwrap();

                Self::Addx(d.parse::<i32>().unwrap())
            }
        };

        Ok(res)
    }
}

fn part01(filename: &str) -> i32 {
    let foo = io::read_value_per_line::<Instruction>(filename)
        .into_iter()
        .flat_map(|i| match i {
            Instruction::Noop => vec![i],
            Instruction::Addx(_) => vec![Instruction::Addx(0), i],
        })
        .collect::<Vec<Instruction>>();

    let mut strenghts: Vec<i32> = Vec::new();
    let mut register_x = 1;

    for cycle in 1..221 {
        let instruction = &foo[cycle];

        println!("{} {}", cycle, register_x);

        match cycle {
            20 | 60 | 100 | 140 | 180 | 220 => strenghts.push(cycle as i32 * register_x),
            _ => {}
        }

        if let Instruction::Addx(v) = instruction {
            register_x += v
        }
    }

    println!("{:?}", strenghts);

    strenghts.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part01_example() {
        assert_eq!(part01("data/y2022/day10-example.txt"), 13140);
    }

    #[test]
    fn part01_input() {
        assert_eq!(part01("data/y2022/day10.txt"), 14240);
    }

    #[test]
    fn part02_example() {
        part02("data/y2022/day10-example.txt");
    }

    #[test]
    fn part02_input() {
        part02("data/y2022/day10.txt");
    }
}
