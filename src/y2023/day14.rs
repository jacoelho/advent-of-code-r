use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Copy, Clone, Ord, PartialOrd, Hash)]
enum Entities {
    RoundRock,
    SquareRock,
    EmptySpace,
}

impl TryFrom<char> for Entities {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'O' => Ok(Self::RoundRock),
            '#' => Ok(Self::SquareRock),
            '.' => Ok(Self::EmptySpace),
            _ => Err("unexpected rock"),
        }
    }
}

fn parse_input(path: &str) -> Vec<Vec<Entities>> {
    std::fs::read_to_string(path)
        .expect("expected file")
        .lines()
        .map(|line| {
            line.chars().filter_map(|ch| Entities::try_from(ch).ok()).collect()
        })
        .collect()
}

fn total_load(grid: &[Vec<Entities>]) -> usize {
    grid.iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().filter_map(move |(_, entity)| {
                if *entity == Entities::RoundRock {
                    Some(grid.len() - y)
                } else {
                    None
                }
            })
        })
        .sum()
}

fn tilt_row_right(r: &[Entities]) -> Vec<Entities> {
    r.iter()
        .copied()
        .group_by(|&c| c == Entities::SquareRock)
        .into_iter()
        .flat_map(|(_, group)| group.sorted())
        .collect()
}

fn transpose<T: Copy>(matrix: &[Vec<T>]) -> Vec<Vec<T>> {
    (0..matrix[0].len())
        .map(|i| (0..matrix.len()).map(|j| matrix[j][i]).collect())
        .collect()
}

fn tilt(matrix: &[Vec<Entities>]) -> Vec<Vec<Entities>> {
    let v = transpose(matrix)
        .iter()
        .map(|row| tilt_row_right(row))
        .collect::<Vec<_>>();

    transpose(&v)
}

fn part01(path: &str) -> usize {
    let grid = parse_input(path);
    total_load(&tilt(&grid))
}

fn rot_ccw(grid: &[Vec<Entities>]) -> Vec<Vec<Entities>> {
    transpose(grid)
        .iter()
        .map(|row| row.iter().rev().copied().collect())
        .collect()
}

fn spin_cycle(grid: Vec<Vec<Entities>>) -> Vec<Vec<Entities>> {
    (0..4).fold(grid, |acc, _| tilt(&rot_ccw(&acc)))
}

fn part02(path: &str) -> usize {
    let grid = parse_input(path);

    let mut foo = grid.clone();

    foo = spin_cycle(foo);

    for y in 0..foo.len() {
        for x in 0..foo[0].len() {
            print!(
                "{}",
                match foo[y][x] {
                    Entities::RoundRock => 'O',
                    Entities::SquareRock => '#',
                    Entities::EmptySpace => '.',
                }
            );
        }
        println!();
    }

    return 0;

    //let goal = 1_000_000_000;
    let goal = 2;
    let mut seen = HashMap::new();
    let mut cycle_number = 1;
    let mut current = grid;
    while cycle_number <= goal {
        let updated = spin_cycle(current);

        if let Some(previous_cycle) = seen.get(&updated) {
            let cycle_length = cycle_number - previous_cycle;
            let remaining = (goal - cycle_number) % cycle_length;

            current =
                (0..remaining).fold(updated.clone(), |acc, _| spin_cycle(acc));

            break;
        }
        seen.insert(updated.clone(), cycle_number);
        cycle_number += 1;
        current = updated.clone();

        println!("----------");

        for y in 0..updated.len() {
            for x in 0..updated[0].len() {
                print!(
                    "{}",
                    match updated[y][x] {
                        Entities::RoundRock => 'O',
                        Entities::SquareRock => '#',
                        Entities::EmptySpace => '.',
                    }
                );
            }
            println!();
        }

        println!("<---------->");
    }

    total_load(&current)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part01_example1() {
        assert_eq!(part01("data/y2023/day14-example1.txt"), 136);
    }

    #[test]
    fn part01_input() {
        assert_eq!(part01("data/y2023/day14.txt"), 105_249);
    }

    #[test]
    #[ignore = "needs fixing"]
    fn part02_example1() {
        assert_eq!(part02("data/y2023/day14-example1.txt"), 64);
    }
}
