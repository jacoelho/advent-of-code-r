use std::collections::HashMap;
use std::str::FromStr;

use crate::io;
use crate::y2022::day07::Output::{Cd, Dir, File, Ls};

#[derive(Debug, PartialEq)]
enum Output {
    Ls,
    Cd(String),
    File(String, i32),
    Dir(String),
}

impl FromStr for Output {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = match s {
            "$ ls" => Ls,
            i if i.starts_with("$ cd") => {
                let d = s.strip_prefix("$ cd ").unwrap();
                Cd(d.to_string())
            }
            i if i.starts_with("dir") => {
                let d = s.strip_prefix("dir ").unwrap();
                Dir(d.to_string())
            }
            _ => {
                let (size, f) = s.split_once(' ').unwrap();
                File(f.to_string(), size.parse().unwrap())
            }
        };

        Ok(res)
    }
}

fn directory_sizes(output: Vec<Output>) -> HashMap<Vec<String>, i32> {
    let mut directories = HashMap::new();
    let mut current_path = Vec::new();

    for x in output {
        match x {
            Ls => continue,
            Dir(_) => continue,
            Cd(dir) => {
                if dir == ".." {
                    current_path.pop();
                } else {
                    current_path.push(dir);
                }
            }
            File(_, size) => {
                for i in 0..=current_path.len() {
                    let path = current_path[..i].to_owned();

                    if path.is_empty() {
                        continue;
                    }

                    *directories.entry(path).or_insert(0) += size;
                }
            }
        }
    }
    directories
}

fn part01(filename: &str) -> i32 {
    let output = io::read_value_per_line::<Output>(filename);

    directory_sizes(output)
        .values()
        .filter(|&v| *v <= 100_000)
        .sum()
}

fn part02(filename: &str) -> i32 {
    let output = io::read_value_per_line::<Output>(filename);

    let mut sizes = directory_sizes(output)
        .values()
        .cloned()
        .collect::<Vec<i32>>();

    sizes.sort();

    let unused = 70000000 - sizes[sizes.len() - 1];

    *sizes.iter().find(|&i| unused + *i >= 30000000).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part01_example() {
        assert_eq!(part01("data/y2022/day07-example.txt"), 95437);
    }

    #[test]
    fn part01_input() {
        assert_eq!(part01("data/y2022/day07.txt"), 1427048);
    }

    #[test]
    fn part02_example() {
        assert_eq!(part02("data/y2022/day07-example.txt"), 24933642);
    }

    #[test]
    fn part02_input() {
        assert_eq!(part02("data/y2022/day07.txt"), 2940614);
    }
}
