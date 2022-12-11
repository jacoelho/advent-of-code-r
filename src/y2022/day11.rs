use std::cmp;

use crate::io;

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add(u64),
    AddSelf,
    Multiply(u64),
    MultiplySelf,
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    test_value: u64,
    on_true_monkey: usize,
    on_false_monkey: usize,
    inpections: usize,
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
            ["*", "old"] => Operation::MultiplySelf,
            ["*", v] => Operation::Multiply(v.parse::<u64>().unwrap()),
            ["+", "old"] => Operation::AddSelf,
            ["+", v] => Operation::Add(v.parse::<u64>().unwrap()),
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
            inpections: 0,
        };

        monkeys.push(m);
    }

    monkeys
}

fn round<F: Fn(u64) -> u64>(worry_level: F, monkeys: &mut [Monkey]) {
    for i in 0..monkeys.len() {
        let m = monkeys[i].clone();

        monkeys[i].inpections += m.items.len();
        monkeys[i].items.truncate(0);

        for i in 0..m.items.len() {
            let item = m.items[i];

            let item = match m.operation {
                Operation::Add(v) => item + v,
                Operation::AddSelf => item + item,
                Operation::Multiply(v) => item * v,
                Operation::MultiplySelf => item * item,
            };

            let item = worry_level(item);

            let target_monkey = if item % m.test_value == 0 {
                m.on_true_monkey
            } else {
                m.on_false_monkey
            };

            monkeys[target_monkey].items.push(item);
        }
    }
}

fn part01(path: &str) -> usize {
    let mut monkeys = parse_input(path);

    for _ in 0..20 {
        round(|w| w / 3, &mut monkeys);
    }

    let mut inpections = monkeys.iter().map(|m| m.inpections).collect::<Vec<_>>();

    inpections.sort_by_key(|el| cmp::Reverse(*el));

    inpections.iter().take(2).product()
}

fn part02(path: &str) -> usize {
    let mut monkeys = parse_input(path);

    let prod: u64 = monkeys.iter().map(|m| m.test_value).product();

    let worry = |v: u64| v % prod;

    for _ in 0..10_000 {
        round(worry, &mut monkeys);
    }

    let mut inpections = monkeys.iter().map(|m| m.inpections).collect::<Vec<_>>();

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
        assert_eq!(part02("data/y2022/day11-example.txt"), 2_713_310_158);
    }

    #[test]
    fn part02_input() {
        assert_eq!(part02("data/y2022/day11.txt"), 20_683_044_837);
    }
}
