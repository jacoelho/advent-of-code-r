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

fn cycle_values(instructions: Vec<Instruction>) -> Vec<i32> {
    instructions.into_iter().fold(vec![1], |mut values, i| {
        let x = values[values.len() - 1];

        match i {
            Instruction::Noop => {
                values.push(x);
                values
            }
            Instruction::Addx(v) => {
                values.push(x);
                values.push(x + v);

                values
            }
        }
    })
}

fn part01(filename: &str) -> i32 {
    let register_x = cycle_values(io::read_value_per_line::<Instruction>(filename));

    let mut sum = 0;
    for cycle in [20, 60, 100, 140, 180, 220].iter() {
        sum += register_x[cycle - 1] * *cycle as i32;
    }

    sum
}

fn part02(filename: &str) {
    let register_x = cycle_values(io::read_value_per_line::<Instruction>(filename));

    let pixels = (0..241)
        .map(|cycle| {
            let x = register_x[cycle];

            if (x - (cycle as i32 % 40)).abs() <= 1 {
                '#'
            } else {
                '.'
            }
        })
        .collect::<Vec<char>>();

    for line in pixels.chunks(40) {
        println!("{}", line.iter().collect::<String>());
    }
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
        part02("data/y2022/day10.txt"); // PLULKBZH
    }
}
