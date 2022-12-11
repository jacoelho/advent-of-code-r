use std::cmp;

use crate::io;

#[derive(Debug, Clone, Copy)]
enum Operator {
    Add,
    Multiply,
}

#[derive(Debug, Clone, Copy)]
enum Rhs {
    Static(u64),
    Old,
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<u64>,
    operation: (Operator, Rhs),
    test_value: u64,
    on_true_monkey: usize,
    on_false_monkey: usize,
}

fn parse_input(path: &str) -> Vec<Monkey> {
    let monkey_lines = io::read_value_chunks::<String>(path);

    let mut monkeys: Vec<Monkey> = Vec::with_capacity(monkey_lines.len());

    for line in monkey_lines {
        let items = line[1]
            .trim_start()
            .strip_prefix("Starting items: ")
            .unwrap()
            .split(", ")
            .filter_map(|v| v.parse::<u64>().ok())
            .collect::<Vec<_>>();

        let operation_values = line[2]
            .trim_start()
            .strip_prefix("Operation: new = old ")
            .unwrap()
            .split(' ')
            .collect::<Vec<&str>>();

        let operation = match operation_values[..] {
            ["*", "old"] => (Operator::Multiply, Rhs::Old),
            ["*", v] => (Operator::Multiply, Rhs::Static(v.parse::<u64>().unwrap())),
            ["+", "old"] => (Operator::Add, Rhs::Old),
            ["+", v] => (Operator::Add, Rhs::Static(v.parse::<u64>().unwrap())),
            _ => panic!("unreachable"),
        };

        let test_value = line[3]
            .trim_start()
            .strip_prefix("Test: divisible by ")
            .unwrap()
            .parse::<u64>()
            .unwrap();

        let on_true_monkey = line[4]
            .trim_start()
            .strip_prefix("If true: throw to monkey ")
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let on_false_monkey = line[5]
            .trim_start()
            .strip_prefix("If false: throw to monkey ")
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let m = Monkey {
            items,
            operation,
            test_value,
            on_true_monkey,
            on_false_monkey,
        };

        monkeys.push(m);
    }

    monkeys
}

fn round(relief: bool, monkeys: &mut [Monkey], inpections: &mut [usize]) {
    let prod: u64 = monkeys.iter().map(|m| m.test_value).product();

    for i in 0..monkeys.len() {
        let m = monkeys[i].clone();

        inpections[i] += m.items.len();
        monkeys[i].items.truncate(0);

        for i in 0..m.items.len() {
            let item = m.items[i];

            let item = match m.operation {
                (Operator::Add, Rhs::Static(v)) => item + v,
                (Operator::Add, Rhs::Old) => item + item,
                (Operator::Multiply, Rhs::Static(v)) => item * v,
                (Operator::Multiply, Rhs::Old) => item * item,
            };

            let item = if relief { item / 3 } else { item };

            let target_monkey = if item % m.test_value == 0 {
                m.on_true_monkey
            } else {
                m.on_false_monkey
            };

            monkeys[target_monkey].items.push(item % prod);
        }
    }
}

fn part01(path: &str) -> usize {
    let mut monkeys = parse_input(path);
    let mut inpections = vec![0; monkeys.len()];

    for _ in 0..20 {
        round(true, &mut monkeys, &mut inpections);
    }

    inpections.sort_by_key(|el| cmp::Reverse(*el));

    inpections.iter().take(2).product()
}

fn part02(path: &str) -> usize {
    let mut monkeys = parse_input(path);
    let mut inpections = vec![0; monkeys.len()];

    for _ in 0..10_000 {
        round(false, &mut monkeys, &mut inpections);
    }

    inpections.sort_by_key(|el| cmp::Reverse(*el));

    inpections.iter().take(2).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part01_example() {
        assert_eq!(part01("data/y2022/day11-example.txt"), 10_605);
    }

    #[test]
    fn part01_input() {
        assert_eq!(part01("data/y2022/day11.txt"), 99_840);
    }

    #[test]
    fn part02_example() {
        assert_eq!(part02("data/y2022/day11-example.txt"), 2713310158);
    }

    #[test]
    fn part02_input() {
        assert_eq!(part02("data/y2022/day11.txt"), 20683044837);
    }
}
