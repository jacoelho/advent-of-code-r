use crate::grid::Position2D;
use crate::io;
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Hash)]
struct Segment {
    sensor: Position2D,
    beacon: Position2D,
}

impl Segment {
    fn scan_range(&self) -> i32 {
        self.sensor.distance(&self.beacon) as _
    }
}

impl FromStr for Segment {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut sep = s
            .char_indices()
            .filter_map(|(i, c)| match c {
                '=' => Some(i + 1),
                ':' | ',' => Some(i),
                _ => None,
            })
            .collect::<Vec<usize>>();

        sep.push(s.len());

        let pos = sep
            .chunks(2)
            .filter_map(|idx| s[idx[0]..idx[1]].parse::<i32>().ok())
            .collect::<Vec<_>>();

        Ok(Segment {
            sensor: Position2D::new(pos[0], pos[1]),
            beacon: Position2D::new(pos[2], pos[3]),
        })
    }
}

#[derive(Debug)]
struct Interval(i32, i32);

impl Interval {
    fn len(&self) -> i32 {
        self.1 - self.0 + 1
    }

    fn is_adjacent(&self, rhs: &Self) -> bool {
        if rhs.0 < self.0 {
            self.0 - rhs.1 <= 1
        } else {
            rhs.0 - self.1 <= 1
        }
    }

    fn merge(&self, rhs: &Self) -> Self {
        Self(self.0.min(rhs.0), self.1.max(rhs.1))
    }
}

fn scan(segments: &[Segment], row: i32) -> Vec<Interval> {
    segments.iter().fold(Vec::new(), |acc, s| {
        let range = s.scan_range() - (s.sensor.y - row).abs();

        if range < 0 {
            return acc;
        }

        let interval = Interval(s.sensor.x - range, s.sensor.x + range);

        let (connected, mut remaining): (Vec<_>, Vec<_>) =
            acc.into_iter().partition(|i| i.is_adjacent(&interval));

        remaining.push(connected.iter().fold(interval, |acc, i| acc.merge(i)));

        remaining
    })
}

fn part01(path: &str, y: i32) -> i32 {
    let segments = io::read_value_per_line::<Segment>(path);
    let intervals: i32 = scan(&segments, y).iter().map(|i| i.len()).sum();
    let beacons = segments
        .iter()
        .filter_map(|s| {
            if s.beacon.y == y {
                Some(s.beacon.x)
            } else {
                None
            }
        })
        .collect::<HashSet<_>>()
        .len();

    intervals - beacons as i32
}

fn part02(path: &str) -> i64 {
    let segments = io::read_value_per_line::<Segment>(path);

    for y in 0..=4000000 {
        let intervals = scan(&segments, y);

        if intervals.len() > 1 {
            return (intervals[0].1.min(intervals[1].1) as i64 + 1) * 4000000 + y as i64;
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part01_example() {
        assert_eq!(part01("data/y2022/day15-example.txt", 10), 26);
    }

    #[test]
    fn part01_input() {
        assert_eq!(part01("data/y2022/day15.txt", 2_000_000), 5_525_847);
    }

    #[test]
    fn part02_example() {
        assert_eq!(part02("data/y2022/day15-example.txt"), 56000011);
    }

    #[test]
    fn part02_input() {
        assert_eq!(part02("data/y2022/day15.txt"), 13340867187704);
    }
}
