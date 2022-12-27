use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    iter,
};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Pos(i32, i32);

impl std::ops::Add for Pos {
    type Output = Pos;

    fn add(self, other: Pos) -> Pos {
        Pos(self.0 + other.0, self.1 + other.1)
    }
}

impl Pos {
    pub fn neighbours(&self) -> [Pos; 4] {
        [
            Pos(self.0 - 1, self.1),
            Pos(self.0 + 1, self.1),
            Pos(self.0, self.1 - 1),
            Pos(self.0, self.1 + 1),
        ]
    }

    pub fn distance(&self, rhs: &Pos) -> i32 {
        (self.0 - rhs.0).abs() + (self.0 - rhs.0).abs()
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Tile {
    Wall,
    Blizzard(Direction),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn step(&self) -> Pos {
        match self {
            Self::Left => Pos(-1, 0),
            Self::Right => Pos(1, 0),
            Self::Up => Pos(0, -1),
            Self::Down => Pos(0, 1),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Map {
    map: HashMap<Pos, Tile>,
    max_x: i32,
    max_y: i32,
}

fn lcm(lhs: i32, rhs: i32) -> i32 {
    lhs * rhs / gcd(lhs, rhs)
}

fn gcd(lhs: i32, rhs: i32) -> i32 {
    let mut max = lhs;
    let mut min = rhs;
    if min > max {
        std::mem::swap(&mut max, &mut min);
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

fn bliz_maps(
    map: &HashMap<Pos, Tile>,
    rows: i32,
    cols: i32,
    max_time: i32,
) -> HashMap<i32, HashSet<Pos>> {
    // key: turn, val: set of a bliz locations
    let mut cache = HashMap::new();

    let mut blizzards: Vec<(Pos, Direction)> = map
        .iter()
        .filter_map(|(pos, tile)| match tile {
            Tile::Wall => None,
            Tile::Blizzard(dir) => Some((*pos, *dir)),
        })
        .collect();

    let coords = blizzards.iter().map(|(coord, _)| *coord).collect();
    cache.insert(0, coords);

    // precompute every blizzard coord at every time before the coords repeat
    for time in 1..max_time {
        for (pos, dir) in blizzards.iter_mut() {
            *pos = *pos + dir.step();
            // if next pos went to an edge, wrap
            match (dir, pos.0, pos.1) {
                (Direction::Left, 0, _) => pos.0 = cols - 2,
                (Direction::Right, x, _) if x == cols - 1 => pos.0 = 1,
                (Direction::Up, _, 0) => pos.1 = rows - 2,
                (Direction::Down, _, y) if y == rows - 1 => pos.1 = 1,
                _ => (),
            }
        }
        let coords = blizzards.iter().map(|(coord, _)| *coord).collect();
        cache.insert(time, coords);
    }

    cache
}

fn parse_input(path: &str) -> Map {
    let input = std::fs::read_to_string(path).expect("expected file");

    let max_y = input.lines().count() as i32;
    let max_x = input.lines().next().unwrap().chars().count() as i32;

    let map = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(|(x, char)| {
                    let pos = Pos(x as i32, y as i32);

                    let tile = match char {
                        '>' => Tile::Blizzard(Direction::Right),
                        '<' => Tile::Blizzard(Direction::Left),
                        '^' => Tile::Blizzard(Direction::Up),
                        'v' => Tile::Blizzard(Direction::Down),
                        '#' => Tile::Wall,
                        _ => return None,
                    };

                    Some((pos, tile))
                })
                .collect::<Vec<_>>()
        })
        .collect();

    Map { map, max_x, max_y }
}

#[derive(PartialEq, Eq)]
struct Node {
    cost: i32,
    heuristic: i32,
    pos: Pos,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_total = self.cost + self.heuristic;
        let other_total = other.cost + other.heuristic;
        other_total.cmp(&self_total)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

struct MapInfo {
    max_y: i32,
    max_x: i32,
    walls: HashSet<Pos>,
    blizzard_maps: HashMap<i32, HashSet<Pos>>,
    repeats_at: i32,
}

fn shortest(from: Pos, to: Pos, start_time: i32, map_info: &MapInfo) -> i32 {
    let MapInfo {
        max_y,
        max_x,
        walls,
        blizzard_maps,
        repeats_at,
    } = map_info;

    let mut pq = BinaryHeap::new();
    // backtracking is allowed, keep track of visited coords at a certain time
    let mut seen = HashSet::new();

    pq.push(Node {
        cost: start_time,
        heuristic: from.distance(&to),
        pos: from,
    });

    seen.insert((from, start_time));

    // keep stepping through time until the priority queue is empty
    while let Some(Node { cost, pos, .. }) = pq.pop() {
        // did we pop a node that's at the target position? It's guaranteed to be the shortest path
        if pos == to {
            return cost;
        }

        let new_cost = cost + 1;
        let blizzards = &blizzard_maps[&(new_cost % repeats_at)];

        let candidates = pos
            // moving to a neighbour is an option
            .neighbours()
            .into_iter()
            // not moving is an option
            .chain(iter::once(pos))
            // can not share a coordinate with a wall
            .filter(|pos| pos.0 >= 0 && pos.1 >= 0 && pos.0 <= *max_x && pos.1 <= *max_y)
            .filter(|coord| !walls.contains(coord))
            // can not share a coordinate with a blizzard
            .filter(|coord| !blizzards.contains(coord));

        for new_pos in candidates {
            // only push to pq if we didn't already see that coord at the same time
            if seen.insert((new_pos, new_cost)) {
                pq.push(Node {
                    cost: new_cost,
                    heuristic: new_pos.distance(&to),
                    pos: new_pos,
                });
            }
        }
    }
    i32::MAX
}

fn part01(path: &str) -> i32 {
    let map = parse_input(path);

    let walls: HashSet<Pos> = map
        .map
        .iter()
        .filter(|(_, tile)| **tile == Tile::Wall)
        .map(|(pos, _)| *pos)
        .collect();

    // lcm of inner area without the walls. patterns repeat every lcm steps
    let lcm = lcm(map.max_y - 2, map.max_x - 2);

    let blizzard_maps = bliz_maps(&map.map, map.max_y, map.max_x, lcm);
    let start = Pos(1, 0);
    let end = Pos(map.max_x - 2, map.max_y - 1);

    let map_info = MapInfo {
        max_y: map.max_y,
        max_x: map.max_x,
        repeats_at: lcm,
        walls,
        blizzard_maps,
    };

    shortest(start, end, 0, &map_info)
}

fn part02(path: &str) -> i32 {
    let map = parse_input(path);

    let walls: HashSet<Pos> = map
        .map
        .iter()
        .filter(|(_, tile)| **tile == Tile::Wall)
        .map(|(pos, _)| *pos)
        .collect();

    // lcm of inner area without the walls. patterns repeat every lcm steps
    let lcm = lcm(map.max_y - 2, map.max_x - 2);

    let blizzard_maps = bliz_maps(&map.map, map.max_y, map.max_x, lcm);
    let start = Pos(1, 0);
    let end = Pos(map.max_x - 2, map.max_y - 1);

    let map_info = MapInfo {
        max_y: map.max_y,
        max_x: map.max_x,
        repeats_at: lcm,
        walls,
        blizzard_maps,
    };

    let there = shortest(start, end, 0, &map_info);
    let back = shortest(end, start, there, &map_info);

    shortest(start, end, back, &map_info)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part01_example() {
        assert_eq!(part01("data/y2022/day24-example.txt"), 18);
    }

    #[test]
    fn part01_input() {
        assert_eq!(part01("data/y2022/day24.txt"), 322);
    }

    #[test]
    fn part02_example() {
        assert_eq!(part02("data/y2022/day24-example.txt"), 54);
    }

    #[test]
    fn part02_input() {
        assert_eq!(part02("data/y2022/day24.txt"), 974);
    }
}
