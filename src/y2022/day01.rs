use crate::io;
use std::collections::BinaryHeap;

pub fn part01(path: &str) -> Result<i32, std::io::Error> {
    let calories: Vec<Vec<i32>> = io::read_value_chunks(path)?;

    Ok(calories
        .iter()
        .map(|elf_calories| elf_calories.iter().sum::<i32>())
        .max()
        .unwrap())
}

pub fn part02(path: &str) -> Result<i32, std::io::Error> {
    let mut calories: BinaryHeap<_> = io::read_value_chunks(path)?
        .iter()
        .map(|elf_calories| elf_calories.iter().sum::<i32>())
        .collect();

    let mut top_three_sum = 0;
    for _ in 0..3 {
        if let Some(v) = calories.pop() {
            top_three_sum += v;
        }
    }

    Ok(top_three_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part01_example() {
        assert_eq!(part01("data/y2022/day01-example.txt").unwrap(), 24_000);
    }

    #[test]
    fn part01_input() {
        assert_eq!(part01("data/y2022/day01.txt").unwrap(), 69_177);
    }

    #[test]
    fn part02_example() {
        assert_eq!(part02("data/y2022/day01-example.txt").unwrap(), 45_000);
    }

    #[test]
    fn part02_input() {
        assert_eq!(part02("data/y2022/day01.txt").unwrap(), 207_456);
    }
}
