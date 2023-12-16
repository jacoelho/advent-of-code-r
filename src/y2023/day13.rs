#[derive(Debug, PartialEq, Copy, Clone)]
enum Pattern {
    Ash,
    Rocks,
}

impl TryFrom<char> for Pattern {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Ash),
            '#' => Ok(Self::Rocks),
            _ => Err("unexpected pattern"),
        }
    }
}

fn parse_input(path: &str) -> Vec<Vec<Vec<Pattern>>> {
    std::fs::read_to_string(path)
        .expect("expected file")
        .split("\n\n")
        .map(|chunk| {
            chunk
                .lines()
                .map(|line| {
                    line.chars()
                        .filter_map(|ch| Pattern::try_from(ch).ok())
                        .collect()
                })
                .collect()
        })
        .collect()
}

fn count_differences(left: &[Pattern], right: &[Pattern]) -> usize {
    left.iter().zip(right.iter()).filter(|&(l, r)| l != r).count()
}

fn transpose_column(pattern: &[Vec<Pattern>], column: usize) -> Vec<Pattern> {
    pattern
        .iter()
        .map(|line| *line.get(column).expect("value should be there"))
        .collect()
}

fn range(start: usize, max: usize) -> Vec<(usize, usize)> {
    (0..=start).rev().zip(start + 1..max).collect()
}

fn find_vertical_mirror(pattern: &[Vec<Pattern>], goal_total: usize) -> usize {
    (0..pattern[0].len() - 1)
        .find_map(|start| {
            if range(start, pattern[0].len())
                .iter()
                .map(|(left, right)| {
                    count_differences(
                        &transpose_column(pattern, *left),
                        &transpose_column(pattern, *right),
                    )
                })
                .sum::<usize>()
                == goal_total
            {
                Some(start + 1)
            } else {
                None
            }
        })
        .unwrap_or(0)
}

fn find_horizontal_mirror(pattern: &Vec<Vec<Pattern>>, goal: usize) -> usize {
    (0..pattern.len() - 1)
        .find_map(|start| {
            if range(start, pattern.len())
                .iter()
                .map(|(up, down)| {
                    count_differences(&pattern[*up], &pattern[*down])
                })
                .sum::<usize>()
                == goal
            {
                Some((start + 1) * 100)
            } else {
                None
            }
        })
        .unwrap_or(0)
}

fn find_mirror(pattern: &Vec<Vec<Pattern>>, goal_total: usize) -> usize {
    find_horizontal_mirror(pattern, goal_total)
        + find_vertical_mirror(pattern, goal_total)
}

fn part01(path: &str) -> usize {
    parse_input(path).iter().map(|pattern| find_mirror(pattern, 0)).sum()
}

fn part02(path: &str) -> usize {
    parse_input(path).iter().map(|pattern| find_mirror(pattern, 1)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part01_example1() {
        assert_eq!(part01("data/y2023/day13-example1.txt"), 405);
    }

    #[test]
    fn part01_input() {
        assert_eq!(part01("data/y2023/day13.txt"), 33047);
    }

    #[test]
    fn part02_example1() {
        assert_eq!(part02("data/y2023/day13-example1.txt"), 400);
    }

    #[test]
    fn part02_input() {
        assert_eq!(part02("data/y2023/day13.txt"), 28806);
    }
}
