use std::collections::VecDeque;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Move(usize, usize, usize);

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words = s.split(' ').collect::<Vec<&str>>();

        let qty = words.get(1).unwrap().parse::<usize>().unwrap();
        let from = words.get(3).unwrap().parse::<usize>().unwrap();
        let to = words.get(5).unwrap().parse::<usize>().unwrap();

        Ok(Move(qty, from - 1, to - 1))
    }
}

fn read_input(filename: &str) -> (Vec<VecDeque<char>>, Vec<Move>) {
    let input = std::fs::read_to_string(filename)
        .expect("expected file")
        .split('\n')
        .map(|line| line.lines().map(|l| l.to_string()).collect())
        .collect::<Vec<String>>();

    let mut input_iter = input.iter();

    let stack_lines = input_iter
        .by_ref()
        .take_while(|line| !line.is_empty())
        .cloned()
        .collect::<Vec<String>>();

    let stacks = stack_lines.iter().fold(Vec::new(), |mut acc, line| {
        acc.push(line.chars().rev().collect::<VecDeque<char>>());
        acc
    });

    let moves = input_iter
        .cloned()
        .filter_map(|line| line.parse::<Move>().ok())
        .collect::<Vec<Move>>();

    (stacks, moves)
}

fn part01(filename: &str) -> String {
    let (mut stacks, moves) = read_input(filename);

    for m in moves {
        for _ in 0..m.0 {
            let v = stacks[m.1].pop_front().unwrap();
            stacks[m.2].push_front(v);
        }
    }

    let mut result = String::with_capacity(stacks.len());
    for mut s in stacks {
        result.push(s.pop_front().unwrap());
    }

    result
}

fn part02(filename: &str) -> String {
    let (mut stacks, moves) = read_input(filename);

    for m in moves {
        let mut v = Vec::new();
        for _ in 0..m.0 {
            v.push(stacks[m.1].pop_front().unwrap());
        }

        for x in v.iter().rev() {
            stacks[m.2].push_front(*x);
        }
    }

    let mut result = String::with_capacity(stacks.len());
    for mut s in stacks {
        result.push(s.pop_front().unwrap());
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part01_example() {
        let foo = part01("data/y2022/day05-example-modified.txt");

        println!("{:?}", foo);

        //assert_eq!(foo.first().unwrap(),);
    }

    #[test]
    fn part01_input() {
        // FZCMJCRHZ

        let foo = part01("data/y2022/day05-modified.txt");

        println!("{:?}", foo);

        //assert_eq!(foo.first().unwrap(),);
    }

    #[test]
    fn part02_example() {
        let foo = part02("data/y2022/day05-example-modified.txt");

        println!("{:?}", foo);

        //assert_eq!(foo.first().unwrap(),);
    }

    #[test]
    fn part02_input() {
        // JSDHQMZGF

        let foo = part02("data/y2022/day05-modified.txt");

        println!("{:?}", foo);

        //assert_eq!(foo.first().unwrap(),);
    }
}
