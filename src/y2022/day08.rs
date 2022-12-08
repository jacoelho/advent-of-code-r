use std::ops::ControlFlow;

fn read_input(filename: &str) -> Vec<Vec<u32>> {
    std::fs::read_to_string(filename)
        .expect("should be able to read file")
        .lines()
        .map(|line| line.chars().filter_map(|c| c.to_digit(10)).collect())
        .collect()
}

type Position = (usize, usize);
type Bounds = (usize, usize);

fn get_neighbours(b: Bounds, p: Position) -> Vec<Vec<Position>> {
    let y_range_before = (0..p.1).rev().map(|y| (p.0, y)).collect::<Vec<Position>>();

    let y_range_after = (p.1 + 1..b.1).map(|y| (p.0, y)).collect::<Vec<Position>>();

    let x_range_before = (0..p.0).rev().map(|x| (x, p.1)).collect::<Vec<Position>>();

    let x_range_after = (p.0 + 1..b.0).map(|x| (x, p.1)).collect::<Vec<Position>>();

    vec![y_range_before, y_range_after, x_range_before, x_range_after]
}

fn part01(filename: &str) -> u32 {
    let trees = read_input(filename);
    let rows = trees.len();
    let columns = trees.get(0).unwrap().len();

    let mut count = 0;
    for y in 0..rows {
        for x in 0..columns {
            if x == 0 || y == 0 || x == columns - 1 || y == rows - 1 {
                count += 1;
                continue;
            }

            let n = get_neighbours((columns, rows), (x, y));
            let height = trees[y][x];

            for i in n {
                let visible = i.iter().map(|&(x, y)| trees[y][x]).all(|h| h < height);

                if visible {
                    count += 1;
                    break;
                }
            }
        }
    }

    count
}

fn part02(filename: &str) -> i32 {
    let trees = read_input(filename);
    let rows = trees.len();
    let columns = trees.get(0).unwrap().len();

    let mut max = 0;
    for y in 0..rows {
        for x in 0..columns {
            let n = get_neighbours((columns, rows), (x, y));
            let height = trees[y][x];

            let scenic_score = n
                .iter()
                .map(|c| {
                    c.iter()
                        .map(|&(x, y)| trees[y][x])
                        .try_fold(0, |acc, h| match h {
                            h if h < height => ControlFlow::Continue(acc + 1),
                            _ => ControlFlow::Break(acc + 1),
                        })
                })
                .map(|v| match v {
                    ControlFlow::Continue(a) => a,
                    ControlFlow::Break(b) => b,
                })
                .product();

            if scenic_score > max {
                max = scenic_score;
            }
        }
    }

    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part01_example() {
        assert_eq!(part01("data/y2022/day08-example.txt"), 21);
    }

    #[test]
    fn part01_input() {
        assert_eq!(part01("data/y2022/day08.txt"), 1801);
    }

    #[test]
    fn part02_example() {
        assert_eq!(part02("data/y2022/day08-example.txt"), 8);
    }

    #[test]
    fn part02_input() {
        assert_eq!(part02("data/y2022/day08.txt"), 209880);
    }
}
