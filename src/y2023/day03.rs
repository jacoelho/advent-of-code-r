use crate::grid::Position2D;
use std::collections::HashSet;

#[derive(PartialEq, Debug)]
enum Schematic {
    Symbol(char),
    Gear,
}

fn parse_input(
    path: &str,
) -> (Vec<(u32, Vec<Position2D>)>, Vec<(Schematic, Position2D)>) {
    let content = std::fs::read_to_string(path).expect("expected file");

    let mut parts = Vec::new();
    let mut symbols = Vec::new();

    let mut number = 0;
    let mut positions = Vec::new();

    for (y, line) in content.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            let pos = Position2D {
                x: x.try_into().expect("expect try_into to work"),
                y: y.try_into().expect("expect try_into to work"),
            };

            match ch {
                '.' => {}
                '0'..='9' => {
                    number =
                        number * 10 + ch.to_digit(10).expect("is a digit");
                    positions.push(pos);
                    continue;
                }
                '*' => symbols.push((Schematic::Gear, pos)),
                _ => symbols.push((Schematic::Symbol(ch), pos)),
            };

            if !positions.is_empty() {
                parts.push((number, positions.clone()));
                number = 0;
                positions.clear();
            }
        }

        if !positions.is_empty() {
            parts.push((number, positions.clone()));
            number = 0;
            positions.clear();
        }
    }

    (parts, symbols)
}

fn part01(path: &str) -> u32 {
    let (parts, symbols) = parse_input(path);

    let symbols =
        symbols.iter().map(|&(_, pos)| pos).collect::<HashSet<Position2D>>();

    parts
        .iter()
        .filter_map(|(part, positions)| {
            let neighbours = positions
                .iter()
                .flat_map(Position2D::neighbours8)
                .collect::<HashSet<Position2D>>();

            if neighbours.is_disjoint(&symbols) {
                None
            } else {
                Some(*part)
            }
        })
        .sum()
}

fn part02(path: &str) -> u32 {
    let (parts, symbols) = parse_input(path);

    let symbols = symbols
        .iter()
        .filter_map(|(schematic, pos)| {
            if matches!(schematic, Schematic::Gear) {
                Some(*pos)
            } else {
                None
            }
        })
        .collect::<HashSet<Position2D>>();

    let parts = parts
        .iter()
        .map(|(pos, positions)| {
            (
                pos,
                positions
                    .iter()
                    .flat_map(Position2D::neighbours8)
                    .collect::<HashSet<Position2D>>(),
            )
        })
        .collect::<Vec<_>>();

    symbols.iter().fold(0, |acc, pos| {
        let neighbours = parts
            .iter()
            .filter_map(|(part, neighbours)| {
                if neighbours.contains(pos) {
                    Some(**part)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        if neighbours.len() == 2 {
            acc + neighbours.iter().product::<u32>()
        } else {
            acc
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part01_example() {
        assert_eq!(part01("data/y2023/day03-example1.txt"), 4361);
    }

    #[test]
    fn part01_input() {
        assert_eq!(part01("data/y2023/day03.txt"), 512_794);
    }

    #[test]
    fn part02_example() {
        assert_eq!(part02("data/y2023/day03-example1.txt"), 467_835);
    }

    #[test]
    fn part02_input() {
        assert_eq!(part02("data/y2023/day03.txt"), 67_779_080);
    }
}
