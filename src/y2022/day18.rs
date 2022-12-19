use crate::io;
use std::{
    collections::{HashSet, VecDeque},
    str::FromStr,
};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Pos(i32, i32, i32);

impl std::ops::Add for Pos {
    type Output = Pos;

    fn add(self, other: Pos) -> Pos {
        Pos(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl Pos {
    fn neighbours(&self) -> Vec<Pos> {
        const NEIGHBOURS: [Pos; 6] = [
            Pos(-1, 0, 0),
            Pos(1, 0, 0),
            Pos(0, -1, 0),
            Pos(0, 1, 0),
            Pos(0, 0, -1),
            Pos(0, 0, 1),
        ];

        NEIGHBOURS.iter().map(|p| *p + *self).collect::<Vec<_>>()
    }
}

impl FromStr for Pos {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = s
            .split(',')
            .filter_map(|s| s.parse().ok())
            .collect::<Vec<_>>();

        Ok(Pos(res[0], res[1], res[2]))
    }
}

fn part01(path: &str) -> usize {
    let cubes: HashSet<Pos> = HashSet::from_iter(io::read_value_per_line::<Pos>(path).into_iter());

    cubes
        .iter()
        .map(|cube| {
            let neighbours = cube.neighbours();

            neighbours.len() - neighbours.iter().filter(|n| cubes.contains(n)).count()
        })
        .sum()
}

struct Range(i32, i32);

impl Range {
    fn contains(&self, p: i32) -> bool {
        self.0 <= p && self.1 >= p
    }
}

fn range(cubes: &[Pos], getter: fn(&Pos) -> i32) -> Range {
    let (min, max) = cubes.iter().fold((i32::MAX, i32::MIN), |acc, c| {
        (acc.0.min(getter(c)), acc.1.max(getter(c)))
    });

    Range(min - 1, max + 1)
}

fn part02(path: &str) -> usize {
    let cubes_vec = io::read_value_per_line::<Pos>(path);
    let cubes: HashSet<Pos> = HashSet::from_iter(cubes_vec.clone().into_iter());

    let x_range = range(&cubes_vec, |p| p.0);
    let y_range = range(&cubes_vec, |p| p.1);
    let z_range = range(&cubes_vec, |p| p.2);

    let mut seen = HashSet::new();
    let mut frontier = VecDeque::from(vec![Pos(x_range.0, y_range.0, z_range.0)]);

    let mut side_count = 0;
    while let Some(pos) = frontier.pop_front() {
        if !seen.insert(pos) {
            continue;
        }

        pos.neighbours()
            .iter()
            .filter(|p| x_range.contains(p.0) && y_range.contains(p.1) && z_range.contains(p.2))
            .for_each(|n| {
                if cubes.contains(n) {
                    side_count += 1;
                } else {
                    frontier.push_back(*n);
                }
            });
    }

    side_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part01_example() {
        assert_eq!(part01("data/y2022/day18-example.txt"), 64);
    }

    #[test]
    fn part01_input() {
        assert_eq!(part01("data/y2022/day18.txt"), 4340);
    }

    #[test]
    fn part02_example() {
        assert_eq!(part02("data/y2022/day18-example.txt"), 58);
    }

    #[test]
    fn part02_input() {
        assert_eq!(part02("data/y2022/day18.txt"), 2468);
    }
}
