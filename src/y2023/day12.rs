use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

impl TryFrom<char> for Spring {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Operational),
            '#' => Ok(Self::Damaged),
            '?' => Ok(Self::Unknown),
            _ => Err("unexpected spring"),
        }
    }
}

#[derive(Debug)]
struct Row {
    springs: Vec<Spring>,
    damaged: Vec<usize>,
}

impl Row {
    fn is_valid(&self) -> bool {
        let non_operational = consecutive_elements(&self.springs)
            .into_iter()
            .filter(|(spring, _)| !matches!(spring, Spring::Operational))
            .collect::<Vec<_>>();

        if non_operational.len() != self.damaged.len() {
            return false;
        }

        non_operational
            .iter()
            .zip(self.damaged.iter())
            .all(|((_, count), instruction)| *count == *instruction)
    }
}

fn consecutive_elements<T: Eq>(input: &[T]) -> Vec<(&T, usize)> {
    input.iter().fold(vec![], |mut acc, elem| {
        if let Some((prev, count)) = acc.last_mut() {
            if *prev == elem {
                *count += 1;
            } else {
                acc.push((elem, 1));
            }
        } else {
            acc.push((elem, 1));
        }
        acc
    })
}

fn parse_input(path: &str) -> Vec<Row> {
    std::fs::read_to_string(path)
        .expect("expected file")
        .lines()
        .map(|line| {
            line.split_once(' ')
                .map(|(left, right)| {
                    let springs = left
                        .chars()
                        .filter_map(|ch| Spring::try_from(ch).ok())
                        .collect();

                    let damaged = right
                        .split(',')
                        .filter_map(|s| s.parse().ok())
                        .collect();

                    Row { springs, damaged }
                })
                .expect("correct input")
        })
        .collect()
}

fn resolve(
    springs: &[Spring],
    damaged: &[usize],
    damaged_group: Option<usize>,
) -> usize {
    fn resolve_memo<'a>(
        springs: &'a [Spring],
        damaged: &'a [usize],
        damaged_group: Option<usize>,
        memo: &mut HashMap<(&'a [Spring], &'a [usize], Option<usize>), usize>,
    ) -> usize {
        let key = (springs, damaged, damaged_group);

        if let Some(v) = memo.get(&key) {
            return *v;
        }

        if springs.is_empty() {
            return match (damaged_group, damaged) {
                (Some(count), [c]) if c == &count => 1,
                (None, []) => 1,
                _ => 0,
            };
        }

        let result = match (&springs[0], damaged, damaged_group) {
            (Spring::Operational, _, None) | (Spring::Unknown, [], None) => {
                resolve_memo(&springs[1..], damaged, None, memo)
            }
            (Spring::Operational | Spring::Unknown, [v, ..], Some(count))
                if count == *v =>
            {
                resolve_memo(&springs[1..], &damaged[1..], None, memo)
            }
            (Spring::Damaged | Spring::Unknown, [v, ..], Some(count))
                if count < *v =>
            {
                resolve_memo(&springs[1..], damaged, Some(count + 1), memo)
            }
            (Spring::Damaged, [_, ..], None) => {
                resolve_memo(&springs[1..], damaged, Some(1), memo)
            }
            (Spring::Unknown, _, None) => {
                resolve_memo(&springs[1..], damaged, Some(1), memo)
                    + resolve_memo(&springs[1..], damaged, None, memo)
            }
            _ => 0,
        };

        memo.insert(key, result);

        result
    }

    resolve_memo(springs, damaged, damaged_group, &mut HashMap::new())
}

fn part01(path: &str) -> usize {
    let rows = parse_input(path);

    rows.iter().map(|row| resolve(&row.springs, &row.damaged, None)).sum()
}

fn unfold<T: Copy>(input: &[T], n: usize, separator: Option<T>) -> Vec<T> {
    input
        .iter()
        .copied()
        .cycle()
        .take(input.len() * n)
        .enumerate()
        .flat_map(|(i, item)| {
            if i > 0 && i % input.len() == 0 { separator } else { None }
                .into_iter()
                .chain(Some(item))
        })
        .collect()
}

fn part02(path: &str) -> usize {
    parse_input(path)
        .iter()
        .map(|row| Row {
            springs: unfold(&row.springs, 5, Some(Spring::Unknown)),
            damaged: unfold(&row.damaged, 5, None),
        })
        .map(|row| resolve(&row.springs, &row.damaged, None))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part01_example1() {
        assert_eq!(part01("data/y2023/day12-example1.txt"), 21);
    }

    #[test]
    fn part01_input() {
        assert_eq!(part01("data/y2023/day12.txt"), 7694);
    }

    #[test]
    fn part01_example2() {
        assert_eq!(part02("data/y2023/day12-example1.txt"), 525_152);
    }

    #[test]
    fn part02_input() {
        assert_eq!(part02("data/y2023/day12.txt"), 5_071_883_216_318);
    }
}
