fn predict_next(s: &[i32]) -> i32 {
    s.last().map_or(0, |&v| v + predict_next(&history_difference(s)))
}

fn history_difference(s: &[i32]) -> Vec<i32> {
    s.windows(2).map(|w| w[1] - w[0]).collect()
}

fn parse_input(path: &str) -> Vec<Vec<i32>> {
    std::fs::read_to_string(path)
        .expect("should be able to read file")
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|v| v.parse::<i32>().ok())
                .collect::<Vec<_>>()
        })
        .collect()
}

fn part01(path: &str) -> i32 {
    parse_input(path).iter().map(|history| predict_next(history)).sum()
}

fn part02(path: &str) -> i32 {
    parse_input(path)
        .into_iter()
        .map(|history| {
            predict_next(&history.into_iter().rev().collect::<Vec<_>>())
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part01_example() {
        assert_eq!(part01("data/y2023/day09-example1.txt"), 114);
    }

    #[test]
    fn part01_input() {
        assert_eq!(part01("data/y2023/day09.txt"), 1_938_731_307);
    }

    #[test]
    fn part02_example() {
        assert_eq!(part02("data/y2023/day09-example1.txt"), 2);
    }

    #[test]
    fn part02_input() {
        assert_eq!(part02("data/y2023/day09.txt"), 948);
    }
}
