use std::collections::HashSet;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Pos(i64, i64);

impl std::ops::Add for Pos {
    type Output = Pos;

    fn add(self, other: Pos) -> Pos {
        Pos(self.0 + other.0, self.1 + other.1)
    }
}

impl std::ops::AddAssign for Pos {
    fn add_assign(&mut self, other: Pos) {
        *self = Pos(self.0 + other.0, self.1 + other.1)
    }
}

impl std::ops::Sub for Pos {
    type Output = Pos;

    fn sub(self, other: Pos) -> Pos {
        Pos(self.0 - other.0, self.1 - other.1)
    }
}

const ROCKS: [&[Pos]; 5] = [
    &[Pos(0, 0), Pos(1, 0), Pos(2, 0), Pos(3, 0)],
    &[Pos(1, 0), Pos(0, 1), Pos(1, 1), Pos(2, 1), Pos(1, 2)],
    &[Pos(0, 0), Pos(1, 0), Pos(2, 0), Pos(2, 1), Pos(2, 2)],
    &[Pos(0, 0), Pos(0, 1), Pos(0, 2), Pos(0, 3)],
    &[Pos(0, 0), Pos(1, 0), Pos(0, 1), Pos(1, 1)],
];

const DOWN: Pos = Pos(0, -1);

fn parse_input(path: &str) -> Vec<Pos> {
    std::fs::read_to_string(path)
        .expect("expected file")
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '<' => Pos(-1, 0),
                    '>' => Pos(1, 0),
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
        })
        .next()
        .unwrap()
}

fn collides(chamber: &HashSet<Pos>, rocks: &[Pos]) -> bool {
    rocks.iter().any(|p| chamber.contains(p))
}

fn maybe_move(rocks: &[Pos], offset: &Pos) -> Option<Vec<Pos>> {
    let rocks = rocks.iter().map(|p| *p + *offset).collect::<Vec<_>>();

    if rocks.iter().any(|p| p.0 < 0 || p.0 > 6) {
        None
    } else {
        Some(rocks)
    }
}

#[allow(dead_code)]
fn print_chamber(chamber: &HashSet<Pos>, rocks: &[Pos]) {
    let rocks: HashSet<Pos> = HashSet::from_iter(rocks.iter().cloned());

    let chamber_max_y = rocks.iter().map(|p| p.1).max().unwrap_or(0);
    let rocks_max_y = rocks.iter().map(|p| p.1).max().unwrap_or(0);

    let max_y = chamber_max_y.max(rocks_max_y);

    for y in (-1..=max_y).rev() {
        for x in -1..=7 {
            let ch = match (x, y) {
                (-1, -1) | (7, -1) => '+',
                (_, -1) => '-',
                (-1, _) | (7, _) => '|',
                (_, _) => {
                    let p = &Pos(x, y);
                    if chamber.contains(p) {
                        '#'
                    } else if rocks.contains(p) {
                        '@'
                    } else {
                        '.'
                    }
                }
            };
            print!("{}", ch);
        }
        println!();
    }
    println!();
}

fn simulate(jets: &[Pos], resting_target: i64) -> i64 {
    let mut chamber = HashSet::from_iter((0..=6).map(|x| Pos(x, -1)));

    let mut height = 0;

    let mut rocks = ROCKS.iter().cycle().enumerate();

    let mut jets = jets.iter().cycle().enumerate();

    let mut resting = 0;

    while resting < resting_target {
        let starting_offset = Pos(2, height + 3);
        let (rock_idx, mut rocks) = rocks
            .next()
            .map(|(i, r)| {
                (
                    i,
                    r.iter().map(|p| *p + starting_offset).collect::<Vec<_>>(),
                )
            })
            .unwrap();

        loop {
            let (jet_idx, jet) = jets.next().unwrap();

            if let Some(move_rocks) = maybe_move(&rocks, jet) {
                if !collides(&chamber, &move_rocks) {
                    rocks = move_rocks;
                }
            };

            if let Some(move_rocks) = maybe_move(&rocks, &DOWN) {
                if !collides(&chamber, &move_rocks) {
                    rocks = move_rocks;
                } else {
                    break;
                }
            };
        }

        chamber.extend(rocks.iter());
        height = height.max(rocks.iter().map(|p| p.1 + 1).max().unwrap());
        resting += 1;
    }

    height
}

fn part01(path: &str) -> i64 {
    let jet_patterns = parse_input(path);

    simulate(&jet_patterns, 2022)
}

fn part02(path: &str) -> i64 {
    let jet_patterns = parse_input(path);

    simulate(&jet_patterns, 1000000000000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part01_example() {
        assert_eq!(part01("data/y2022/day17-example.txt"), 3068);
    }

    #[test]
    fn part01_input() {
        assert_eq!(part01("data/y2022/day17.txt"), 3059);
    }

    // #[test]
    // fn part02_example() {
    //     assert_eq!(part02("data/y2022/day17-example.txt"), 1514285714288);
    // }

    #[test]
    fn part02_input() {
        assert_eq!(part02("data/y2022/day17.txt"), 1514285714288);
    }
}
