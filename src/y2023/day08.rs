use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
enum Instruction {
    Left,
    Right,
}

impl TryFrom<char> for Instruction {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(Self::Left),
            'R' => Ok(Self::Right),
            _ => Err("unexpected instruction"),
        }
    }
}

#[derive(Debug)]
struct Network {
    instructions: Vec<Instruction>,
    nodes: HashMap<String, Vec<String>>,
}

impl FromStr for Network {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split_once("\n\n")
            .map(|(instructions, nodes)| {
                let instructions = instructions
                    .chars()
                    .map(|ch| {
                        Instruction::try_from(ch)
                            .expect("expected instruction")
                    })
                    .collect::<Vec<_>>();

                let nodes = nodes
                    .lines()
                    .map(|line| {
                        line.split(['=', ' ', '(', ',', ')'])
                            .filter(|s| !s.is_empty())
                            .collect::<Vec<_>>()
                    })
                    .fold(HashMap::new(), |mut acc, line| {
                        acc.entry(line[0].to_string()).or_insert_with(|| {
                            line[1..]
                                .iter()
                                .map(std::string::ToString::to_string)
                                .collect::<Vec<_>>()
                        });
                        acc
                    });

                Ok(Self { instructions, nodes })
            })
            .expect("network")
    }
}

impl Network {
    fn solve(&self, start: &String, is_goal: fn(&String) -> bool) -> usize {
        let mut position = start;
        let mut steps = 0;

        for instruction in self.instructions.iter().cycle() {
            let pos = self.nodes.get(position).expect("pos");
            steps += 1;

            let current = match instruction {
                Instruction::Left => pos.get(0).expect("should work"),
                Instruction::Right => pos.get(1).expect("should work"),
            };

            if is_goal(current) {
                break;
            }
            position = current;
        }

        steps
    }
}

fn part01(path: &str) -> usize {
    let network = std::fs::read_to_string(path)
        .expect("should be able to read file")
        .parse::<Network>()
        .expect("parsing");

    let goal = |v: &String| v == &"ZZZ".to_string();

    network.solve(&"AAA".to_string(), goal)
}

fn part02(path: &str) -> usize {
    let network = std::fs::read_to_string(path)
        .expect("should be able to read file")
        .parse::<Network>()
        .expect("parsing");

    let goal = |v: &String| v.ends_with('Z');

    network
        .nodes
        .keys()
        .filter(|v| v.ends_with('A'))
        .map(|start| network.solve(start, goal))
        .fold(1, lcm)
}

const fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

const fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part01_example() {
        assert_eq!(part01("data/y2023/day08-example1.txt"), 2);
    }

    #[test]
    fn part01_input() {
        assert_eq!(part01("data/y2023/day08.txt"), 13771);
    }

    #[test]
    fn part02_example() {
        assert_eq!(part02("data/y2023/day08-example2.txt"), 6);
    }

    #[test]
    fn part02_input() {
        assert_eq!(part02("data/y2023/day08.txt"), 13_129_439_557_681);
    }
}
