use crate::io;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(PartialEq, Debug)]
enum Cube {
    Red(u32),
    Green(u32),
    Blue(u32),
}

impl FromStr for Cube {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split_once(' ').ok_or("invalid input").and_then(|(count, colour)| {
            let count = count.parse().map_err(|_| "invalid count")?;

            let cube = match colour {
                "red" => Self::Red(count),
                "green" => Self::Green(count),
                "blue" => Self::Blue(count),
                _ => return Err("invalid colour"),
            };

            Ok(cube)
        })
    }
}

impl Cube {
    const fn to_index(&self) -> u32 {
        match self {
            Self::Red(_) => 1,
            Self::Green(_) => 2,
            Self::Blue(_) => 3,
        }
    }

    const fn value(&self) -> u32 {
        match *self {
            Self::Green(v) | Self::Blue(v) | Self::Red(v) => v,
        }
    }
}

fn parse_set(s: &str) -> Vec<Cube> {
    s.split(',')
        .filter_map(|v| v.trim_start_matches(' ').parse::<Cube>().ok())
        .collect()
}

#[derive(Debug, PartialEq)]
struct Game(u32, Vec<Vec<Cube>>);

impl FromStr for Game {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split_once(": ").ok_or("invalid input").and_then(|(game, sets)| {
            let game = game.split_once(' ').ok_or("invalid game").and_then(
                |(_, game)| game.parse().map_err(|_| "invalid game"),
            )?;

            let sets = sets.split(';').map(parse_set).collect();

            Ok(Self(game, sets))
        })
    }
}

fn part01(path: &str) -> u32 {
    io::read_value_per_line::<Game>(path)
        .iter()
        .filter(|game| {
            game.1.iter().all(|set| {
                set.iter().all(|cube| match *cube {
                    Cube::Red(v) => v <= 12,
                    Cube::Green(v) => v <= 13,
                    Cube::Blue(v) => v <= 14,
                })
            })
        })
        .map(|&Game(number, _)| number)
        .sum()
}

fn part02(path: &str) -> u32 {
    io::read_value_per_line::<Game>(path)
        .iter()
        .map(|game| {
            game.1.iter().flatten().fold(HashMap::new(), |mut map, cube| {
                map.entry(cube.to_index())
                    .and_modify(|e| {
                        *e = if *e > cube.value() { *e } else { cube.value() }
                    })
                    .or_insert_with(|| cube.value());

                map
            })
        })
        .map(|map| map.values().product::<u32>())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_game_example() {
        assert_eq!(
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red".parse::<Game>().unwrap(),
            Game(4, vec![
                vec![Cube::Green(1), Cube::Red(3), Cube::Blue(6)],
                vec![Cube::Green(3), Cube::Red(6)],
                vec![Cube::Green(3), Cube::Blue(15), Cube::Red(14)]])
        );
    }

    #[test]
    fn part01_example() {
        assert_eq!(part01("data/y2023/day02-example1.txt"), 8);
    }

    #[test]
    fn part01_input() {
        assert_eq!(part01("data/y2023/day02.txt"), 2476);
    }

    #[test]
    fn part02_example() {
        assert_eq!(part02("data/y2023/day02-example1.txt"), 2286);
    }

    #[test]
    fn part02_input() {
        assert_eq!(part02("data/y2023/day02.txt"), 54911);
    }
}
