use std::collections::HashSet;

fn find_window(window: usize, s: String) -> usize {
    let res = s
        .bytes()
        .collect::<Vec<u8>>()
        .windows(window)
        .enumerate()
        .filter(|&(_, item)| item.iter().cloned().collect::<HashSet<u8>>().len() == item.len())
        .take(1)
        .map(|(idx, _)| idx)
        .next()
        .unwrap();

    res + window
}

fn part01(filename: &str) -> usize {
    find_window(4, std::fs::read_to_string(filename).expect("expected file"))
}

fn part02(filename: &str) -> usize {
    find_window(
        14,
        std::fs::read_to_string(filename).expect("expected file"),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part01_example() {
        let _foo = part01("data/y2022/day06.txt");
    }

    #[test]
    fn part01_example_1() {
        assert_eq!(
            find_window(4, String::from("bvwbjplbgvbhsrlpgdmjqwftvncz")),
            5
        );
    }

    #[test]
    fn part01_example_2() {
        assert_eq!(
            find_window(4, String::from("nppdvjthqldpwncqszvftbrmjlhg")),
            6
        );
    }

    #[test]
    fn part01_example_3() {
        assert_eq!(
            find_window(4, String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")),
            10
        );
    }

    #[test]
    fn part01_input() {
        assert_eq!(part01("data/y2022/day06.txt"), 1578);
    }

    #[test]
    fn part02_example_3() {
        assert_eq!(
            find_window(14, String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")),
            29
        );
    }

    #[test]
    fn part02_input() {
        assert_eq!(part02("data/y2022/day06.txt"), 2178);
    }
}
