use std::collections::{HashMap, HashSet};

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

fn maybe_move(rocks: &[Pos], offset: &Pos) -> Option<Vec<Pos>> {
    let rocks = rocks.iter().map(|p| *p + *offset).collect::<Vec<_>>();

    if rocks.iter().any(|p| p.0 < 0 || p.0 > 6) {
        None
    } else {
        Some(rocks)
    }
}

struct Chamber {
    rocks: HashSet<Pos>,
    jet_patterns: Vec<Pos>,
    piece_count: usize,
    jet_count: usize,
    height: i64,
}

impl Chamber {
    fn new(jet_patterns: Vec<Pos>) -> Self {
        Self {
            rocks: HashSet::from_iter((0..=6).map(|x| Pos(x, -1))),
            jet_patterns,
            piece_count: 0,
            jet_count: 0,
            height: 0,
        }
    }

    #[allow(dead_code)]
    fn print_chamber(&self, rocks: &[Pos]) {
        let rocks: HashSet<Pos> = HashSet::from_iter(rocks.iter().cloned());

        let chamber_max_y = self.rocks.iter().map(|p| p.1).max().unwrap_or(0);
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
                        if self.rocks.contains(p) {
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

    fn collides(&self, piece: &[Pos]) -> bool {
        piece.iter().any(|p| self.rocks.contains(p))
    }

    fn top(&self) -> Vec<i64> {
        let ys = self.rocks.iter().fold(vec![0; 7], |mut acc, p| {
            acc[p.0 as usize] = acc[p.0 as usize].max(p.1);
            acc
        });

        let min_y = *ys.iter().min().unwrap();

        ys.into_iter().map(|y| y - min_y).collect::<Vec<_>>()
    }

    fn drop_piece(&mut self) {
        let starting_offset = Pos(2, self.height + 3);

        let mut piece = ROCKS[self.piece_count]
            .iter()
            .map(|p| *p + starting_offset)
            .collect::<Vec<_>>();

        self.piece_count = (self.piece_count + 1) % ROCKS.len();

        loop {
            let jet = self.jet_patterns[self.jet_count];
            self.jet_count = (self.jet_count + 1) % self.jet_patterns.len();

            if let Some(attempt) = maybe_move(&piece, &jet) {
                if !self.collides(&attempt) {
                    piece = attempt;
                }
            };

            if let Some(attempt) = maybe_move(&piece, &DOWN) {
                if !self.collides(&attempt) {
                    piece = attempt;
                } else {
                    break;
                }
            };
        }

        self.rocks.extend(piece.iter());
        self.height = self
            .height
            .max(piece.iter().map(|p| p.1 + 1).max().unwrap());
    }
}

fn part01(path: &str) -> i64 {
    let jet_patterns = parse_input(path);

    let mut chamber = Chamber::new(jet_patterns);

    for _ in 0..2022 {
        chamber.drop_piece();
    }

    chamber.height
}

fn part02(path: &str) -> i64 {
    let jet_patterns = parse_input(path);

    let mut chamber = Chamber::new(jet_patterns);
    let mut seen: HashMap<(Vec<i64>, usize, usize), (i64, i64)> = HashMap::new();
    let mut drop_count: i64 = 0;
    loop {
        chamber.drop_piece();
        drop_count += 1;
        let height = chamber.height;

        let state = (chamber.top(), chamber.piece_count, chamber.jet_count);

        if let Some(entry) = seen.get(&state) {
            let delta_height = chamber.height - entry.0;
            let delta_drops = drop_count - (entry.1 as i64);

            let remaining_drops = 1000000000000_i64 - (entry.1 as i64);

            let needed_drops = remaining_drops / delta_drops;
            let leftover_drops = remaining_drops % delta_drops;
            let integral_height = entry.0 + delta_height * (needed_drops as i64);

            for _ in 0..leftover_drops {
                chamber.drop_piece();
            }

            let leftover_height = chamber.height - height;

            return integral_height + leftover_height;
        } else {
            seen.insert(state, (chamber.height, drop_count));
        }
    }
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

    #[test]
    fn part02_example() {
        assert_eq!(part02("data/y2022/day17-example.txt"), 1514285714288);
    }

    #[test]
    fn part02_input() {
        assert_eq!(part02("data/y2022/day17.txt"), 1500874635587);
    }
}
