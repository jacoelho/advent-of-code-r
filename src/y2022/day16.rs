use crate::search::shortest_path;
use itertools::Itertools;
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

#[derive(Debug)]
struct Room<'a>(&'a str, usize, Vec<&'a str>);

fn parse_input(input: &str) -> HashMap<&str, Room> {
    input
        .lines()
        .map(|line| {
            let fields = line.split([' ', '=', ',', ';']).collect::<Vec<_>>();

            let name = fields[1];
            let rate = fields[5].parse().unwrap();
            let tunnels = fields[11..]
                .iter()
                .filter_map(|s| if !s.is_empty() { Some(*s) } else { None })
                .collect::<Vec<_>>();

            (name, Room(name, rate, tunnels))
        })
        .collect()
}

fn shortest_paths<'a>(rooms: &'a HashMap<&str, Room>) -> HashMap<(&'a str, &'a str), usize> {
    let neighbours = |room| {
        let from = &rooms.get(&room).unwrap();

        from.2.clone()
    };

    rooms
        .iter()
        .filter_map(|(name, room)| if room.1 > 0 { Some(name) } else { None })
        .tuple_combinations()
        .fold(HashMap::new(), |mut acc, (name1, name2)| {
            acc.entry(("AA", name1))
                .or_insert_with(|| shortest_path("AA", |el| el == *name1, neighbours).unwrap());

            acc.entry(("AA", name2))
                .or_insert_with(|| shortest_path("AA", |el| el == *name2, neighbours).unwrap());

            let dist = shortest_path(*name1, |el| el == *name2, neighbours).unwrap();

            acc.insert((name1, name2), dist);
            acc.insert((name2, name1), dist);

            acc
        })
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct State<'a> {
    opened: BTreeSet<&'a str>,
    curr: &'a str,
    elapsed: usize,
    relieved: usize,
}

fn wait_until_ending(
    max_time: usize,
    elapsed: usize,
    relieved: usize,
    opened: &BTreeSet<&str>,
    map: &HashMap<&str, Room>,
) -> usize {
    let time_left = max_time - elapsed;
    let relieved_per_min: usize = opened.iter().map(|name| &map[name].1).sum();

    relieved + (relieved_per_min * time_left)
}

fn search_caves<'a>(
    flowing_caves: &HashSet<&'a str>,
    caves: &HashMap<&str, Room>,
    distances: &HashMap<(&str, &str), usize>,
    mut seen: HashSet<(BTreeSet<&'a str>, usize, usize)>,
    time_allowed: usize,
) -> HashMap<BTreeSet<&'a str>, usize> {
    let mut max_relieved_states: HashMap<BTreeSet<&str>, usize> = HashMap::new();
    let mut q = VecDeque::new();

    seen.insert((BTreeSet::new(), 0, 0));

    q.push_back(State {
        curr: "AA",
        opened: BTreeSet::new(),
        elapsed: 0,
        relieved: 0,
    });

    while let Some(State {
        opened,
        curr,
        elapsed,
        relieved,
    }) = q.pop_front()
    {
        let relieved_at_end = wait_until_ending(time_allowed, elapsed, relieved, &opened, caves);

        max_relieved_states
            .entry(opened.clone())
            .and_modify(|val| *val = relieved_at_end.max(*val))
            .or_insert(relieved_at_end);

        if opened.len() == flowing_caves.len() || elapsed >= time_allowed {
            continue;
        }

        let unopened = flowing_caves.iter().filter(|name| !opened.contains(*name));

        for dest in unopened {
            let cost = distances[&(curr, *dest)] + 1;

            let new_elapsed = elapsed + cost;

            if new_elapsed >= time_allowed {
                continue;
            }

            let relieved_per_min: usize = opened.iter().map(|name| &caves[name].1).sum();
            let new_relieved = relieved + (relieved_per_min * cost);
            let mut new_opened = opened.clone();
            new_opened.insert(dest);

            if seen.insert((new_opened.clone(), new_elapsed, new_relieved)) {
                q.push_back(State {
                    opened: new_opened,
                    curr: dest,
                    elapsed: new_elapsed,
                    relieved: new_relieved,
                });
            }
        }
    }
    max_relieved_states
}

fn part01(path: &str) -> usize {
    let input = std::fs::read_to_string(path).expect("expected file");
    let caves = parse_input(&input);
    let distances = shortest_paths(&caves);
    let flowing_caves = caves
        .iter()
        .filter_map(|(_, room)| if room.1 > 0 { Some(room.0) } else { None })
        .collect::<HashSet<_>>();

    search_caves(&flowing_caves, &caves, &distances, HashSet::new(), 30)
        .values()
        .copied()
        .max()
        .unwrap_or(0)
}

fn part02(path: &str) -> usize {
    let input = std::fs::read_to_string(path).expect("expected file");
    let caves = parse_input(&input);
    let distances = shortest_paths(&caves);
    let flowing_caves = caves
        .iter()
        .filter_map(|(_, room)| if room.1 > 0 { Some(room.0) } else { None })
        .collect::<HashSet<_>>();

    search_caves(&flowing_caves, &caves, &distances, HashSet::new(), 26)
        .iter()
        .tuple_combinations()
        .filter_map(|(human, elephant)| {
            if human.0.is_disjoint(elephant.0) {
                Some(human.1 + elephant.1)
            } else {
                None
            }
        })
        .max()
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part01_example() {
        assert_eq!(part01("data/y2022/day16-example.txt"), 1651);
    }

    #[test]
    fn part01_input() {
        assert_eq!(part01("data/y2022/day16.txt"), 1751);
    }

    #[test]
    fn part02_example() {
        assert_eq!(part02("data/y2022/day16-example.txt"), 1707);
    }

    #[test]
    fn part02_input() {
        assert_eq!(part02("data/y2022/day16.txt"), 2207);
    }
}
