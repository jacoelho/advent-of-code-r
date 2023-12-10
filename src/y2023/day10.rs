use crate::grid::Position2D;
use crate::search::bfs;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
enum Pipe {
    Start,
    Vertical,
    Horizontal,
    UpperLeft,
    UpperRight,
    LowerLeft,
    LowerRight,
}

impl TryFrom<char> for Pipe {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '|' => Ok(Self::Vertical),
            '-' => Ok(Self::Horizontal),
            'L' => Ok(Self::LowerLeft),
            'J' => Ok(Self::LowerRight),
            '7' => Ok(Self::UpperRight),
            'F' => Ok(Self::UpperLeft),
            'S' => Ok(Self::Start),
            _ => Err("unexpected pipe"),
        }
    }
}

fn neighbours(
    tile: (Position2D, Pipe),
    grid: &HashMap<Position2D, Pipe>,
) -> Vec<(Position2D, Pipe)> {
    let neighbours = match tile.1 {
        Pipe::Start => vec![
            Position2D::new(1, 0),
            Position2D::new(-1, 0),
            Position2D::new(0, -1),
            Position2D::new(0, 1),
        ],
        Pipe::Vertical => {
            vec![Position2D::new(0, 1), Position2D::new(0, -1)]
        }
        Pipe::Horizontal => {
            vec![Position2D::new(1, 0), Position2D::new(-1, 0)]
        }
        Pipe::UpperLeft => {
            vec![Position2D::new(1, 0), Position2D::new(0, 1)]
        }
        Pipe::UpperRight => {
            vec![Position2D::new(-1, 0), Position2D::new(0, 1)]
        }
        Pipe::LowerLeft => {
            vec![Position2D::new(1, 0), Position2D::new(0, -1)]
        }
        Pipe::LowerRight => {
            vec![Position2D::new(-1, 0), Position2D::new(0, -1)]
        }
    };

    neighbours
        .iter()
        .filter_map(|pos| {
            let current = *pos + tile.0;

            grid.get(&current).map(|pipe| (current, *pipe))
        })
        .collect::<Vec<_>>()
}

fn parse_input(path: &str) -> HashMap<Position2D, Pipe> {
    std::fs::read_to_string(path)
        .expect("expected file")
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, char)| {
                let pos = Position2D {
                    x: x.try_into().ok()?,
                    y: y.try_into().ok()?,
                };

                Pipe::try_from(char).ok().map(|v| (pos, v))
            })
        })
        .collect()
}

fn start_position(grid: &HashMap<Position2D, Pipe>) -> (Position2D, Pipe) {
    grid.iter()
        .find(|(_, p)| **p == Pipe::Start)
        .map(|(p, v)| (*p, *v))
        .expect("grid needs a start position")
}

fn part01(path: &str) -> usize {
    let grid = parse_input(path);

    let start_position = start_position(&grid);

    let neighbours =
        |position: (Position2D, Pipe)| neighbours(position, &grid);

    let result = bfs(start_position, neighbours);

    result.len() / 2
}

fn part02(path: &str) -> usize {
    let grid = parse_input(path);

    let start_position = start_position(&grid);

    let neighbours =
        |position: (Position2D, Pipe)| neighbours(position, &grid);

    let shape = bfs(start_position, neighbours)
        .into_iter()
        .collect::<HashMap<Position2D, Pipe>>();

    let x_max = shape.keys().map(|position| position.x).max().unwrap_or(0);
    let y_max = shape.keys().map(|position| position.y).max().unwrap_or(0);

    let mut count = 0;

    for y in 0..=y_max {
        let mut inside = false;

        for x in 0..=x_max {
            let current_position = Position2D::new(x, y);

            match shape.get(&current_position) {
                Some(Pipe::Vertical | Pipe::LowerRight | Pipe::LowerLeft) => {
                    inside = !inside;
                }
                None if inside => count += 1,
                _ => {}
            };
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part01_example1() {
        assert_eq!(part01("data/y2023/day10-example1.txt"), 4);
    }

    #[test]
    fn part01_example2() {
        assert_eq!(part01("data/y2023/day10-example2.txt"), 8);
    }

    #[test]
    fn part01_input() {
        assert_eq!(part01("data/y2023/day10.txt"), 6815);
    }

    #[test]
    fn part02_example3() {
        assert_eq!(part02("data/y2023/day10-example3.txt"), 4);
    }

    #[test]
    fn part02_example4() {
        assert_eq!(part02("data/y2023/day10-example4.txt"), 8);
    }

    #[test]
    fn part02_example5() {
        assert_eq!(part02("data/y2023/day10-example5.txt"), 10);
    }

    #[test]
    fn part02_input() {
        assert_eq!(part02("data/y2023/day10.txt"), 269);
    }
}
