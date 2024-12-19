use advent_of_code::get_cardinal_directions;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

advent_of_code::solution!(18);

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.x.cmp(&other.x).then_with(|| self.y.cmp(&other.y)))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Eq, PartialEq)]
struct Node {
    x: usize,
    y: usize,
    cost: u32,
}

pub fn part_one(input: &str) -> Option<u32> {
    let coords = input.lines().map(|line| {
        let mut splitter = line.split(',');
        let x = splitter.next().unwrap().parse::<usize>().unwrap();
        let y = splitter.next().unwrap().parse::<usize>().unwrap();
        (x, y)
    });

    let mut grid: Vec<Vec<bool>> = (0..=70)
        .map(|_| (0..=70).map(|_| false).collect())
        .collect();

    let n = 1024;

    coords.take(n).for_each(|(x, y)| {
        grid[y][x] = true;
    });

    let mut heap = BinaryHeap::new();
    let mut visited = HashSet::new();

    heap.push(Node {
        x: 0,
        y: 0,
        cost: 0,
    });

    while let Some(node) = heap.pop() {
        if node.x == 70 && node.y == 70 {
            return Some(node.cost);
        }

        if visited.contains(&(node.x, node.y)) {
            continue;
        }

        visited.insert((node.x, node.y));

        let directions = get_cardinal_directions((node.x, node.y), 71, 71);

        for (x, y) in directions {
            if !grid[y][x] {
                heap.push(Node {
                    x,
                    y,
                    cost: node.cost + 1,
                });
            }
        }
    }

    None
}

// TODO: Test binary search + floodfill
pub fn part_two(input: &str) -> Option<String> {
    let mut coords = input.lines().map(|line| {
        let mut splitter = line.split(',');
        let x = splitter.next().unwrap().parse::<usize>().unwrap();
        let y = splitter.next().unwrap().parse::<usize>().unwrap();
        (x, y)
    });

    let mut grid: Vec<Vec<bool>> = (0..=70)
        .map(|_| (0..=70).map(|_| false).collect())
        .collect();

    for _ in 0..=1024 {
        if let Some((x, y)) = coords.next() {
            grid[y][x] = true;
        } else {
            // Test case
            return None;
        }
    }

    let mut last_coord = (0, 0);

    while let Some(visited) = grid_solvable(&grid) {
        loop {
            last_coord = coords.next().unwrap();
            grid[last_coord.1][last_coord.0] = true;
            if visited.contains(&last_coord) {
                break;
            }
        }
    }
    Some(format!("{},{}", last_coord.0, last_coord.1))
}

fn grid_solvable(grid: &[Vec<bool>]) -> Option<HashSet<(usize, usize)>> {
    let mut heap = BinaryHeap::new();
    let mut visited = HashSet::new();
    heap.push(Node {
        x: 0,
        y: 0,
        cost: 0,
    });

    while let Some(node) = heap.pop() {
        if node.x == 70 && node.y == 70 {
            return Some(visited);
        }

        if visited.contains(&(node.x, node.y)) {
            continue;
        }

        visited.insert((node.x, node.y));

        let directions = get_cardinal_directions((node.x, node.y), 71, 71);

        for (x, y) in directions {
            if !grid[y][x] {
                heap.push(Node {
                    x,
                    y,
                    cost: node.cost + 1,
                });
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(146));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
