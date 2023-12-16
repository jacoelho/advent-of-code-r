use crate::grid::Position2D;
use std::collections::HashMap;
use std::str;

pub fn read_value_per_line<T>(path: &str) -> Vec<T>
where
    T: str::FromStr,
{
    std::fs::read_to_string(path)
        .expect("should be able to read file")
        .lines()
        .filter_map(|line| line.parse::<T>().ok())
        .collect()
}

pub fn read_grid<T>(path: &str) -> HashMap<Position2D, T>
where
    T: TryFrom<char>,
{
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

                T::try_from(char).ok().map(|v| (pos, v))
            })
        })
        .collect()
}

pub fn read_value_chunks<T>(path: &str) -> Vec<Vec<T>>
where
    T: str::FromStr,
{
    std::fs::read_to_string(path)
        .expect("expected file")
        .split("\n\n")
        .map(|chunk| {
            chunk.lines().filter_map(|line| line.parse::<T>().ok()).collect()
        })
        .collect()
}
