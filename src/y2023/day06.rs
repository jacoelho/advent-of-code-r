const fn calculate(length: u64, time: u64) -> u64 {
    time * (length - time)
}

fn different_ways(length: u64, record: u64) -> usize {
    (1..length).filter(|&time| calculate(length, time) > record).count()
}

fn parse_input(path: &str) -> Vec<(u64, u64)> {
    let values = std::fs::read_to_string(path)
        .expect("should be able to read file")
        .lines()
        .map(|line| {
            line.split_once(':')
                .map(|(_, values)| {
                    values
                        .split_whitespace()
                        .filter_map(|v| v.parse().ok())
                        .collect::<Vec<_>>()
                })
                .expect("expected values")
        })
        .collect::<Vec<_>>();

    values
        .get(0)
        .cloned()
        .unwrap_or_default()
        .into_iter()
        .zip(values.get(1).cloned().unwrap_or_default())
        .collect::<Vec<_>>()
}

fn parse_input_part2(path: &str) -> (u64, u64) {
    let values = std::fs::read_to_string(path)
        .expect("should be able to read file")
        .lines()
        .map(|line| {
            line.split_once(':')
                .map(|(_, value)| {
                    value.replace(' ', "").parse::<u64>().expect("value")
                })
                .expect("expected values")
        })
        .collect::<Vec<_>>();

    (values[0], values[1])
}

fn part01(path: &str) -> usize {
    parse_input(path)
        .iter()
        .map(|&(time, record)| different_ways(time, record))
        .product()
}

fn part02(path: &str) -> usize {
    let (time, record) = parse_input_part2(path);

    different_ways(time, record)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part01_example() {
        assert_eq!(part01("data/y2023/day06-example1.txt"), 288);
    }

    #[test]
    fn part01_input() {
        assert_eq!(part01("data/y2023/day06.txt"), 1_195_150);
    }

    #[test]
    fn part02_example() {
        assert_eq!(part02("data/y2023/day06-example1.txt"), 71503);
    }

    #[test]
    fn part02_input() {
        assert_eq!(part02("data/y2023/day06.txt"), 42_550_411);
    }
}
