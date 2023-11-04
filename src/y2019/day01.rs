use crate::io;
use std::iter;

pub fn part01(path: &str) -> u64 {
    io::read_value_per_line::<u64>(path).iter().map(|&v| fuel(v)).sum()
}

const fn fuel(mass: u64) -> u64 {
    (mass / 3) - 2
}

pub fn part02(path: &str) -> u64 {
    io::read_value_per_line::<u64>(path)
        .iter()
        .map(|&v| fuel_with_fuel(v))
        .sum()
}

fn fuel_with_fuel(mass: u64) -> u64 {
    iter::successors(Some(fuel(mass)), |&v| {
        Some(if v < 9 { 0 } else { fuel(v) })
    })
    .take_while(|&v| v > 0)
    .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part01_input() {
        assert_eq!(part01("data/y2019/day01.txt"), 3_399_394);
    }

    #[test]
    fn part02_input() {
        assert_eq!(part02("data/y2019/day01.txt"), 5_096_223);
    }
}
