use std::collections::HashSet;

pub fn read_input(filename: &str) -> Vec<String> {
    std::fs::read_to_string(filename)
        .expect("should be able to read")
        .lines()
        .map(|line| line.to_string())
        .collect()
}

pub fn priority(item: &char) -> i32 {
    match item {
        'a'..='z' => *item as i32 - 'a' as i32 + 1,
        'A'..='Z' => *item as i32 - 'A' as i32 + 27,
        _ => panic!("unexpected"),
    }
}

pub fn part01(filename: &str) -> i32 {
    read_input(filename)
        .iter()
        .map(|line| {
            let (a, b) = line.split_at(line.len() / 2);

            let set_a = a.chars().collect::<HashSet<_>>();
            let set_b = b.chars().collect::<HashSet<_>>();

            set_a.intersection(&set_b).map(priority).sum::<i32>()
        })
        .sum::<i32>()
}

pub fn part02(filename: &str) -> i32 {
    read_input(filename)
        .chunks(3)
        .map(|chunk| {
            let mut chunks = chunk.iter().map(|e| e.chars().collect::<HashSet<_>>());

            chunks
                .next()
                .map(|set| {
                    chunks.fold(set, |set1, set2| {
                        set1.intersection(&set2).cloned().collect::<HashSet<_>>()
                    })
                })
                .unwrap()
                .iter()
                .map(priority)
                .sum::<i32>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part01_example() {
        assert_eq!(part01("data/y2022/day03-example.txt"), 157);
    }

    #[test]
    fn part01_input() {
        assert_eq!(part01("data/y2022/day03.txt"), 8233);
    }

    #[test]
    fn part02_example() {
        assert_eq!(part02("data/y2022/day03-example.txt"), 70);
    }

    #[test]
    fn part02_input() {
        assert_eq!(part02("data/y2022/day03.txt"), 2821);
    }
}
