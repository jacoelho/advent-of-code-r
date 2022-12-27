use std::collections::HashMap;

#[derive(Debug)]
enum Operator {
    Sub,
    Add,
    Div,
    Mul,
}

impl Operator {
    fn eval(&self, lhs: i64, rhs: i64) -> i64 {
        match self {
            Self::Sub => lhs - rhs,
            Self::Add => lhs + rhs,
            Self::Mul => lhs * rhs,
            Self::Div => lhs / rhs,
        }
    }

    fn eval_left_anti_operation(&self, lhs: i64, rhs: i64) -> i64 {
        match self {
            Self::Sub => lhs + rhs,
            Self::Add => lhs - rhs,
            Self::Mul => lhs / rhs,
            Self::Div => lhs * rhs,
        }
    }

    fn eval_right_anti_operation(&self, lhs: i64, rhs: i64) -> i64 {
        match self {
            Self::Sub => rhs - lhs,
            Self::Add => lhs - rhs,
            Self::Mul => lhs / rhs,
            Self::Div => rhs / lhs,
        }
    }
}

#[derive(Debug)]
enum Monkey<'a> {
    Number(i64),
    Operation(&'a str, Operator, &'a str),
}

fn parse_input(input: &str) -> HashMap<&str, Monkey> {
    input
        .lines()
        .map(|line| {
            let fields = line.split_whitespace().collect::<Vec<_>>();

            let name = fields[0].trim_end_matches(':');

            match fields[1].parse() {
                Ok(value) => (name, Monkey::Number(value)),
                Err(_) => {
                    let operator = match fields[2] {
                        "-" => Operator::Sub,
                        "+" => Operator::Add,
                        "/" => Operator::Div,
                        "*" => Operator::Mul,
                        _ => unreachable!(),
                    };

                    (name, Monkey::Operation(fields[1], operator, fields[3]))
                }
            }
        })
        .collect()
}

fn yell(monkey: &str, monkeys: &HashMap<&str, Monkey>) -> i64 {
    match &monkeys[monkey] {
        Monkey::Number(value) => *value,
        Monkey::Operation(lhs, operator, rhs) => {
            let lhs = yell(lhs, monkeys);
            let rhs = yell(rhs, monkeys);

            operator.eval(lhs, rhs)
        }
    }
}

fn part01(path: &str) -> i64 {
    let input = std::fs::read_to_string(path).expect("expected file");
    let monkeys = parse_input(&input);

    yell("root", &monkeys)
}

const HUMAN: &str = "humn";

fn contains_human(name: &str, monkeys: &HashMap<&str, Monkey>) -> bool {
    if name == HUMAN {
        return true;
    }

    match &monkeys[name] {
        Monkey::Number(_) => false,
        Monkey::Operation(lhs, _, rhs) => {
            contains_human(lhs, monkeys) || contains_human(rhs, monkeys)
        }
    }
}

fn yell_human(name: &str, value: i64, monkeys: &HashMap<&str, Monkey>) -> i64 {
    if name == HUMAN {
        return value;
    }

    match &monkeys[name] {
        Monkey::Number(v) => *v,
        Monkey::Operation(lhs, op, rhs) => {
            let (name, value) = if contains_human(lhs, monkeys) {
                let rhs = yell(rhs, monkeys);

                (lhs, op.eval_left_anti_operation(value, rhs))
            } else {
                let lhs = yell(lhs, monkeys);

                (rhs, op.eval_right_anti_operation(value, lhs))
            };

            yell_human(name, value, monkeys)
        }
    }
}

fn part02(path: &str) -> i64 {
    let input = std::fs::read_to_string(path).expect("expected file");
    let monkeys = parse_input(&input);

    let (lhs, rhs) = match monkeys["root"] {
        Monkey::Operation(lhs, _, rhs) => (lhs, rhs),
        _ => panic!(),
    };

    let (name, value) = if contains_human(lhs, &monkeys) {
        let value = yell(rhs, &monkeys);

        (lhs, value)
    } else {
        let value = yell(lhs, &monkeys);

        (rhs, value)
    };

    yell_human(name, value, &monkeys)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part01_example() {
        assert_eq!(part01("data/y2022/day21-example.txt"), 152);
    }

    #[test]
    fn part01_input() {
        assert_eq!(part01("data/y2022/day21.txt"), 223971851179174);
    }

    #[test]
    fn part02_example() {
        assert_eq!(part02("data/y2022/day21-example.txt"), 301);
    }

    #[test]
    fn part02_input() {
        assert_eq!(part02("data/y2022/day21.txt"), 3379022190351);
    }
}
