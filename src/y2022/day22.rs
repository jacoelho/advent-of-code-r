use itertools::Itertools;

#[derive(Debug, PartialEq, Eq)]
enum Tile {
    Open,
    Solid,
    None,
}

#[derive(Debug)]
enum Turn {
    Left,
    Right,
}

#[derive(Debug)]
enum Instruction {
    Rotate(Turn),
    Forward(i32),
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn turn(&self, turn: &Turn) -> Self {
        match (self, turn) {
            (Direction::Left, Turn::Left) => Direction::Down,
            (Direction::Left, Turn::Right) => Direction::Up,
            (Direction::Right, Turn::Left) => Direction::Up,
            (Direction::Right, Turn::Right) => Direction::Down,
            (Direction::Up, Turn::Left) => Direction::Left,
            (Direction::Up, Turn::Right) => Direction::Right,
            (Direction::Down, Turn::Left) => Direction::Right,
            (Direction::Down, Turn::Right) => Direction::Left,
        }
    }

    fn step(&self) -> Pos {
        match self {
            Direction::Left => Pos(-1, 0),
            Direction::Right => Pos(1, 0),
            Direction::Up => Pos(0, -1),
            Direction::Down => Pos(0, 1),
        }
    }

    fn score(&self) -> i32 {
        match self {
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Up => 3,
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Pos(i32, i32);

impl std::ops::Add for Pos {
    type Output = Pos;

    fn add(self, other: Pos) -> Pos {
        Pos(self.0 + other.0, self.1 + other.1)
    }
}

impl std::ops::Sub for Pos {
    type Output = Pos;

    fn sub(self, other: Pos) -> Pos {
        Pos(self.0 - other.0, self.1 - other.1)
    }
}

fn parse_input(path: &str) -> (Vec<Vec<Tile>>, Vec<Instruction>) {
    let input = std::fs::read_to_string(path).expect("expected file");

    let (grid, instructions) = input.split_once("\n\n").unwrap();

    let instructions = instructions
        .chars()
        .group_by(|c| c.is_ascii_digit())
        .into_iter()
        .map(|(_, v)| -> Instruction {
            let s = v.collect::<String>();

            match s.as_str() {
                "L" => Instruction::Rotate(Turn::Left),
                "R" => Instruction::Rotate(Turn::Right),
                _ => Instruction::Forward(s.parse().unwrap()),
            }
        })
        .collect();

    let grid = grid
        .lines()
        .map(|line| {
            line.chars()
                .map(|el| match el {
                    ' ' => Tile::None,
                    '.' => Tile::Open,
                    '#' => Tile::Solid,
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    (grid, instructions)
}

fn get(grid: &[Vec<Tile>], position: Pos) -> Option<&Tile> {
    grid.get(position.1 as usize)
        .and_then(|row| row.get(position.0 as usize))
}

fn wrap_around(grid: &[Vec<Tile>], pos: Pos, direction: Direction) -> (Pos, Direction) {
    let step = direction.step();

    let mut curr = pos;

    while let Some(tile) = get(grid, curr - step) {
        if *tile == Tile::None {
            break;
        }
        curr = curr - step;
    }

    (curr, direction)
}

type WrapMap = fn(grid: &[Vec<Tile>], pos: Pos, direction: Direction) -> (Pos, Direction);

fn move_grid(grid: &[Vec<Tile>], instructions: Vec<Instruction>, wrap: WrapMap) -> i32 {
    let start_x = grid[0].iter().position(|tile| *tile == Tile::Open).unwrap() as i32;

    let mut pos = Pos(start_x, 0);

    let mut direction = Direction::Right;

    for instruction in &instructions {
        match instruction {
            Instruction::Rotate(turn) => direction = direction.turn(turn),
            Instruction::Forward(steps) => {
                for _ in 0..*steps {
                    let step = direction.step();

                    let new_pos = pos + step;
                    let new_tile = get(grid, new_pos).unwrap_or(&Tile::None);

                    match new_tile {
                        Tile::Solid => break,
                        Tile::Open => pos = new_pos,
                        Tile::None => {
                            let (new_pos, dir) = wrap(grid, pos, direction);

                            if let Some(Tile::Solid) = get(grid, new_pos) {
                                break;
                            }

                            pos = new_pos;
                            direction = dir;
                        }
                    }
                }
            }
        }
    }

    1000 * (pos.1 + 1) + 4 * (pos.0 + 1) + direction.score()
}

fn part01(path: &str) -> i32 {
    let (grid, instructions) = parse_input(path);

    move_grid(&grid, instructions, wrap_around)
}

fn wrap_cube(_grid: &[Vec<Tile>], pos: Pos, direction: Direction) -> (Pos, Direction) {
    let (cube_y, cube_x, new_dir) = match (pos.1 / 50, pos.0 / 50, direction) {
        (0, 1, Direction::Up) => (3, 0, Direction::Right),
        (0, 1, Direction::Left) => (2, 0, Direction::Right),
        (0, 2, Direction::Up) => (3, 0, Direction::Up),
        (0, 2, Direction::Right) => (2, 1, Direction::Left),
        (0, 2, Direction::Down) => (1, 1, Direction::Left),
        (1, 1, Direction::Right) => (0, 2, Direction::Up),
        (1, 1, Direction::Left) => (2, 0, Direction::Down),
        (2, 0, Direction::Up) => (1, 1, Direction::Right),
        (2, 0, Direction::Left) => (0, 1, Direction::Right),
        (2, 1, Direction::Right) => (0, 2, Direction::Left),
        (2, 1, Direction::Down) => (3, 0, Direction::Left),
        (3, 0, Direction::Right) => (2, 1, Direction::Up),
        (3, 0, Direction::Down) => (0, 2, Direction::Down),
        (3, 0, Direction::Left) => (0, 1, Direction::Down),
        _ => unreachable!(),
    };

    // find idxes within the cube
    let (row_idx, col_idx) = (pos.1 % 50, pos.0 % 50);

    let i = match direction {
        Direction::Left => 49 - row_idx,
        Direction::Right => row_idx,
        Direction::Up => col_idx,
        Direction::Down => 49 - col_idx,
    };

    // find new idxes within the cube
    let new_row = match new_dir {
        Direction::Left => 49 - i,
        Direction::Right => i,
        Direction::Up => 49,
        Direction::Down => 0,
    };
    let new_col = match new_dir {
        Direction::Left => 49,
        Direction::Right => 0,
        Direction::Up => i,
        Direction::Down => 49 - i,
    };

    let new_pos = Pos(cube_x * 50 + new_col, cube_y * 50 + new_row);

    (new_pos, new_dir)
}

fn part02(path: &str) -> i32 {
    let (grid, instructions) = parse_input(path);

    move_grid(&grid, instructions, wrap_cube)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part01_example() {
        assert_eq!(part01("data/y2022/day22-example.txt"), 6032);
    }

    #[test]
    fn part01_input() {
        assert_eq!(part01("data/y2022/day22.txt"), 88226);
    }

    #[test]
    fn part02_input() {
        assert_eq!(part02("data/y2022/day22.txt"), 57305);
    }
}
