use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
    hash::Hash,
};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Pos(i32, i32);

impl std::ops::Add for Pos {
    type Output = Pos;

    fn add(self, other: Pos) -> Pos {
        Pos(self.0 + other.0, self.1 + other.1)
    }
}

impl Pos {
    pub fn neighbours(&self) -> [Pos; 8] {
        [
            Pos(self.0 - 1, self.1 - 1),
            Pos(self.0, self.1 - 1),
            Pos(self.0 + 1, self.1 - 1),
            Pos(self.0 - 1, self.1),
            Pos(self.0 + 1, self.1),
            Pos(self.0 - 1, self.1 + 1),
            Pos(self.0, self.1 + 1),
            Pos(self.0 + 1, self.1 + 1),
        ]
    }
}

fn parse_input(path: &str) -> HashSet<Pos> {
    std::fs::read_to_string(path)
        .expect("expected file")
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(|(x, char)| {
                    if char == '#' {
                        Some(Pos(x as i32, y as i32))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

const DIRECTIONS: [[Pos; 3]; 4] = [
    [Pos(0, -1), Pos(-1, -1), Pos(1, -1)], // N
    [Pos(0, 1), Pos(-1, 1), Pos(1, 1)],    // S
    [Pos(-1, 0), Pos(-1, -1), Pos(-1, 1)], // W
    [Pos(1, 0), Pos(1, -1), Pos(1, 1)],    // E
];

fn round(mut elves: HashSet<Pos>, round: usize) -> (HashSet<Pos>, bool) {
    let movers = elves
        .iter()
        .filter(|elf| elf.neighbours().iter().any(|n| elves.contains(n)))
        .filter_map(|elf| {
            for dir in round..round + 4 {
                let dirs = DIRECTIONS[dir % 4];

                if !dirs.iter().any(|d| elves.contains(&(*elf + *d))) {
                    return Some((*elf, *elf + dirs[0]));
                }
            }
            None
        })
        .collect::<HashMap<Pos, Pos>>();

    let safe = movers
        .iter()
        .fold(HashMap::new(), |mut acc, (_, v)| {
            acc.entry(v).and_modify(|count| *count += 1).or_insert(1);
            acc
        })
        .into_iter()
        .filter_map(|(k, v)| if v == 1 { Some(*k) } else { None })
        .collect::<HashSet<_>>();

    let mut moved = false;

    movers
        .iter()
        .filter(|(_, v)| safe.contains(v))
        .for_each(|(from, to)| {
            elves.remove(from);
            elves.insert(*to);
            moved = true;
        });

    (elves, moved)
}

fn part01(path: &str) -> i32 {
    let mut elves = parse_input(path);

    for r in 0..10 {
        (elves, _) = round(elves, r);
    }

    let (min_x, max_x, min_y, max_y) = elves.iter().fold(
        (i32::MAX, i32::MIN, i32::MAX, i32::MIN),
        |(min_x, max_x, min_y, max_y), &pos| {
            (
                min(min_x, pos.0),
                max(max_x, pos.0),
                min(min_y, pos.1),
                max(max_y, pos.1),
            )
        },
    );
    (max_x - min_x + 1) * (max_y - min_y + 1) - elves.len() as i32
}

fn part02(path: &str) -> i32 {
    let mut elves = parse_input(path);

    for r in 0.. {
        let (new_elves, moved) = round(elves, r);

        if !moved {
            return r as i32 + 1;
        }

        elves = new_elves
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part01_example() {
        assert_eq!(part01("data/y2022/day23-example.txt"), 110);
    }

    #[test]
    fn part01_input() {
        assert_eq!(part01("data/y2022/day23.txt"), 3762);
    }

    #[test]
    fn part02_example() {
        assert_eq!(part02("data/y2022/day23-example.txt"), 20);
    }

    #[test]
    fn part02_input() {
        assert_eq!(part02("data/y2022/day23.txt"), 997);
    }
}
