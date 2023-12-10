use std::collections::VecDeque;
use std::{
    collections::{BinaryHeap, HashSet},
    hash::Hash,
};

struct MinHeapContainer<T> {
    cost: usize,
    value: T,
}

impl<T> PartialEq for MinHeapContainer<T> {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl<T> PartialOrd for MinHeapContainer<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Eq for MinHeapContainer<T> {}

impl<T> Ord for MinHeapContainer<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost).reverse()
    }
}

pub fn shortest_path<T, G, N>(
    start: T,
    goal: G,
    neighbours: N,
) -> Option<usize>
where
    T: Eq + Hash + Copy,
    G: Fn(T) -> bool,
    N: Fn(T) -> Vec<T>,
{
    let mut frontier = BinaryHeap::new();
    let mut visited = HashSet::new();

    visited.insert(start);

    frontier.push(MinHeapContainer { cost: 0, value: start });

    while let Some(MinHeapContainer { cost, value }) = frontier.pop() {
        if goal(value) {
            return Some(cost);
        }

        let new_cost = 1 + cost;

        for el in neighbours(value) {
            if !visited.contains(&el) {
                visited.insert(el);
                frontier.push(MinHeapContainer { cost: new_cost, value: el });
            }
        }
    }

    None
}

pub fn bfs<T, N>(start: T, neighbours: N) -> Vec<T>
where
    T: Eq + Hash + Copy,
    N: Fn(T) -> Vec<T>,
{
    let mut frontier = VecDeque::new();
    let mut visited = HashSet::new();
    let mut result = Vec::new();

    visited.insert(start);
    frontier.push_back(start);

    while let Some(node) = frontier.pop_front() {
        result.push(node);

        for el in neighbours(node).into_iter().filter(|el| visited.insert(*el))
        {
            frontier.push_back(el);
        }
    }

    result
}
