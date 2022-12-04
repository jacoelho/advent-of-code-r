use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Range(i32, i32);

impl FromStr for Range {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((a, b)) = s.split_once('-') {
            let start = a.parse::<i32>().expect("expected number");
            let end = b.parse::<i32>().expect("expected number");

            Ok(Range(start, end))
        } else {
            Err(())
        }
    }
}

impl Range {
    fn contains(&self, rhs: &Range) -> bool {
        self.0 <= rhs.0 && self.1 >= rhs.1 || rhs.0 <= self.0 && rhs.1 >= self.1
    }

    fn overlaps(&self, rhs: &Range) -> bool {
        self.0 <= rhs.1 && rhs.0 <= self.1
    }
}

fn read_input(filename: &str) -> Vec<(Range, Range)> {
    std::fs::read_to_string(filename)
        .expect("should be able to read")
        .lines()
        .map(|line| {
            let (a, b) = line.split_once(',').unwrap();

            (a.parse::<Range>().unwrap(), b.parse::<Range>().unwrap())
        })
        .collect()
}

fn part01(filename: &str) -> usize {
    read_input(filename)
        .iter()
        .filter(|(a, b)| a.contains(b))
        .count()
}

fn part02(filename: &str) -> usize {
    read_input(filename)
        .iter()
        .filter(|(a, b)| a.overlaps(b))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part01_example() {
        assert_eq!(part01("data/y2022/day04-example.txt"), 2);
    }

    #[test]
    fn part01_input() {
        assert_eq!(part01("data/y2022/day04.txt"), 569);
    }

    #[test]
    fn part02_example() {
        assert_eq!(part02("data/y2022/day04-example.txt"), 4);
    }

    #[test]
    fn part02_input() {
        assert_eq!(part02("data/y2022/day04.txt"), 936);
    }
}
