use std::collections::HashMap;

use crate::grid::Position2D;
use crate::search;

fn parse_input(path: &str) -> (Position2D, Position2D, HashMap<Position2D, i32>) {
    let lines = std::fs::read_to_string(path)
        .expect("expected file")
        .lines()
        .map(|line| line.bytes().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut m = HashMap::new();
    let mut start = None;
    let mut end = None;

    for (y, line) in lines.into_iter().enumerate() {
        for (x, height) in line.into_iter().enumerate() {
            let pos = Position2D {
                x: x.try_into().unwrap(),
                y: y.try_into().unwrap(),
            };

            let value = match height {
                b'E' => {
                    end = Some(pos);
                    25
                }
                b'S' => {
                    start = Some(pos);
                    0
                }
                v => (v - b'a') as i32,
            };

            m.entry(pos).or_insert(value);
        }
    }

    (start.unwrap(), end.unwrap(), m)
}

fn part01(path: &str) -> usize {
    let (start, end, m) = parse_input(path);

    let neighbours = |p: Position2D| {
        let from = *m.get(&p).unwrap();

        p.neighbours4()
            .into_iter()
            .filter(|&c| match m.get(&c) {
                Some(&to) => to - from <= 1,
                None => false,
            })
            .collect::<Vec<_>>()
    };

    search::shortest_path(start, |p| p == end, neighbours).unwrap()
}

fn part02(path: &str) -> usize {
    let (_, end, m) = parse_input(path);

    let neighbours = |p: Position2D| {
        let from = *m.get(&p).unwrap();

        p.neighbours4()
            .into_iter()
            .filter(|&c| match m.get(&c) {
                Some(&to) => from - to <= 1,
                None => false,
            })
            .collect::<Vec<_>>()
    };

    let goal = |p| {
        let height = *m.get(&p).unwrap();

        height == 0
    };

    search::shortest_path(end, goal, neighbours).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part01_example() {
        assert_eq!(part01("data/y2022/day12-example.txt"), 31);
    }

    #[test]
    fn part01_input() {
        assert_eq!(part01("data/y2022/day12.txt"), 447);
    }

    #[test]
    fn part02_example() {
        assert_eq!(part02("data/y2022/day12-example.txt"), 29);
    }

    #[test]
    fn part02_input() {
        assert_eq!(part02("data/y2022/day12.txt"), 446);
    }
}
