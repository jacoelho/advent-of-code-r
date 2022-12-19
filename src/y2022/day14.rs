use std::collections::HashMap;
use std::hash::Hash;

#[derive(PartialEq)]
enum Particle {
    Rock,
    Sand,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position2D {
    x: i32,
    y: i32,
}

impl Position2D {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn down(&self) -> Self {
        Self {
            x: self.x,
            y: self.y + 1,
        }
    }

    fn down_left(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y + 1,
        }
    }

    fn down_right(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y + 1,
        }
    }
}

fn parse_input(path: &str) -> HashMap<Position2D, Particle> {
    std::fs::read_to_string(path)
        .expect("expected file")
        .lines()
        .flat_map(|line| {
            line.split(" -> ")
                .map(|p| {
                    let (x, y) = p.split_once(',').unwrap();

                    (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap())
                })
                .collect::<Vec<_>>()
                .windows(2)
                .flat_map(|el| {
                    let (sx, sy) = el[0];
                    let (ex, ey) = el[1];

                    if sx == ex {
                        let (a, b) = if sy > ey { (ey, sy) } else { (sy, ey) };

                        (a..=b)
                            .map(|y| (Position2D::new(sx, y), Particle::Rock))
                            .collect::<Vec<_>>()
                    } else {
                        let (a, b) = if sx > ex { (ex, sx) } else { (sx, ex) };

                        (a..=b)
                            .map(|x| (Position2D::new(x, sy), Particle::Rock))
                            .collect::<Vec<_>>()
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<HashMap<_, _>>()
}

fn simulate(
    grid: &HashMap<Position2D, Particle>,
    void: i32,
    start: &Position2D,
) -> Option<Position2D> {
    let mut particle = *start;

    if grid.contains_key(start) {
        return None;
    }

    'simulation: while particle.y < void {
        let directions = vec![particle.down(), particle.down_left(), particle.down_right()];

        for direction in directions {
            if !grid.contains_key(&direction) {
                particle = direction;
                continue 'simulation;
            }
        }

        return Some(particle);
    }

    None
}

fn fill(
    mut grid: HashMap<Position2D, Particle>,
    void: i32,
    start: &Position2D,
) -> HashMap<Position2D, Particle> {
    while let Some(particule) = simulate(&grid, void, start) {
        grid.insert(particule, Particle::Sand);
    }

    grid
}

fn part01(path: &str) -> usize {
    let grid = parse_input(path);
    let void = grid.keys().map(|p| p.y).max().unwrap();
    let start = Position2D::new(500, 0);
    let after = fill(grid, void, &start);

    after
        .values()
        .filter(|v| matches!(v, Particle::Sand))
        .count()
}

fn part02(path: &str) -> usize {
    let mut grid = parse_input(path);
    let bottom = 2 + grid.keys().map(|p| p.y).max().unwrap();
    let xs = grid.keys().map(|p| p.x).collect::<Vec<_>>();
    let min_x = *xs.iter().min().unwrap();
    let max_x = *xs.iter().max().unwrap();

    grid.extend(
        (min_x - bottom..max_x + bottom).map(|x: i32| (Position2D::new(x, bottom), Particle::Rock)),
    );

    let start = Position2D::new(500, 0);

    let after = fill(grid, bottom, &start);

    after
        .values()
        .filter(|v| matches!(v, Particle::Sand))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part01_example() {
        assert_eq!(part01("data/y2022/day14-example.txt"), 24);
    }

    #[test]
    fn part01_input() {
        assert_eq!(part01("data/y2022/day14.txt"), 1513);
    }

    #[test]
    fn part02_example() {
        assert_eq!(part02("data/y2022/day14-example.txt"), 93);
    }

    #[test]
    fn part02_input() {
        assert_eq!(part02("data/y2022/day14.txt"), 22646);
    }
}
