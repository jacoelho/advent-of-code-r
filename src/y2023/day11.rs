use crate::grid;
use crate::grid::Position2D;
use std::collections::{HashMap, HashSet};
use std::os::unix::raw::uid_t;

#[derive(Debug, Eq, PartialEq)]
enum Universe {
    Galaxy,
    EmptySpace,
}

impl TryFrom<char> for Universe {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::EmptySpace),
            '#' => Ok(Self::Galaxy),
            _ => Err("unexpected entity"),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Image {
    map: HashMap<Position2D, Universe>,
    x_max: i32,
    y_max: i32,
}

impl Image {
    fn is_galaxy(&self, x: i32, y: i32) -> bool {
        self.map.get(&Position2D::new(x, y)) == Some(&Universe::Galaxy)
    }

    fn columns_factors(&self) -> Vec<i32> {
        (0..=self.x_max)
            .scan(0, |acc, x| {
                if (0..=self.y_max).any(|y| self.is_galaxy(x, y)) {
                    Some(*acc)
                } else {
                    *acc += 1;
                    Some(*acc)
                }
            })
            .collect()
    }

    fn rows_factors(&self) -> Vec<i32> {
        (0..=self.y_max)
            .scan(0, |acc, y| {
                if (0..=self.x_max).any(|x| self.is_galaxy(x, y)) {
                    Some(*acc)
                } else {
                    *acc += 1;
                    Some(*acc)
                }
            })
            .collect()
    }

    fn galaxies(&self) -> Vec<Position2D> {
        self.map
            .iter()
            .filter_map(|(position, element)| match element {
                Universe::EmptySpace => None,
                Universe::Galaxy => Some(*position),
            })
            .collect()
    }
}

fn parse_input(path: &str) -> Image {
    let map = std::fs::read_to_string(path)
        .expect("expected file")
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, char)| {
                let pos = Position2D {
                    x: x.try_into().ok()?,
                    y: y.try_into().ok()?,
                };

                Universe::try_from(char).ok().map(|v| (pos, v))
            })
        })
        .collect();

    let (x_max, y_max) = grid::dimensions(&map);

    Image { map, x_max, y_max }
}

fn generate_pairs(input: &[Position2D]) -> Vec<(Position2D, Position2D)> {
    input
        .iter()
        .enumerate()
        .flat_map(|(i, elem1)| {
            input.iter().skip(i + 1).map(|elem2| (*elem1, *elem2))
        })
        .collect()
}

fn solve(image: &Image, scale_factor: i32) -> usize {
    let galaxies = image.galaxies();
    let rows_factors = image.rows_factors();
    let columns_factors = image.columns_factors();

    generate_pairs(&galaxies)
        .iter()
        .map(|(a, b)| {
            let aa = Position2D::new(
                a.x + columns_factors[a.x as usize] * (scale_factor - 1),
                a.y + rows_factors[a.y as usize] * (scale_factor - 1),
            );
            let bb = Position2D::new(
                b.x + columns_factors[b.x as usize] * (scale_factor - 1),
                b.y + rows_factors[b.y as usize] * (scale_factor - 1),
            );

            aa.distance(&bb)
        })
        .sum()
}

fn part01(path: &str) -> usize {
    let image = parse_input(path);

    solve(&image, 2)
}

fn part02(path: &str) -> usize {
    let image = parse_input(path);

    solve(&image, 1_000_000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part01_example1() {
        assert_eq!(part01("data/y2023/day11-example1.txt"), 374);
    }

    #[test]
    fn part01_input() {
        assert_eq!(part01("data/y2023/day11.txt"), 9_795_148);
    }

    #[test]
    fn part02_example1() {
        assert_eq!(part02("data/y2023/day11-example1.txt"), 82_000_210);
    }

    #[test]
    fn part02_input() {
        assert_eq!(part02("data/y2023/day11.txt"), 650_672_493_820);
    }
}
