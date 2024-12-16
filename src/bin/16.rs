use std::collections::{HashSet, HashMap};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use advent_of_code::Direction;

advent_of_code::solution!(16);

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
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
    direction: Direction,
    visited: HashSet<(usize, usize)>,
}

impl Node {
    fn new(x: usize, y: usize, cost: u32, direction: Direction) -> Self {
        let mut visited = HashSet::new();
        visited.insert((x, y));
        Node { x, y, cost, direction, visited }
    }

    fn new_from_node(node: &Node, x: usize, y: usize, cost: u32, direction: Direction) -> Self {
        let mut visited = node.visited.clone();
        visited.insert((x, y));
        Node { x, y, cost, direction, visited }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut end = (0, 0);
    let mut heap = BinaryHeap::new();
    let mut visited = HashSet::new();

    let grid = input.lines().enumerate().map(|(y, line)| {
        line.chars().enumerate().map(|(x, c)| {
            match c {
                '.' => true,
                '#' => false,
                'S' => {
                    heap.push(Node::new(x, y, 0, Direction::Right));
                    visited.insert((x, y));
                    true
                },
                'E' => {
                    end = (x, y);
                    true
                },
                _ => panic!("unexpected character"),
            }
        }).collect::<Vec<bool>>()
    }).collect::<Vec<Vec<bool>>>();

    while let Some(node) = heap.pop() {
        if (node.x, node.y) == end {
            return Some(node.cost);
        }

        let directions = vec![
            (node.x + 1, node.y, Direction::Right),
            (node.x, node.y + 1, Direction::Down),
            (node.x - 1, node.y, Direction::Left),
            (node.x, node.y - 1, Direction::Up),
        ];

        for (x, y, direction) in directions {
            let mut cost = node.cost + 1;
            if node.direction != direction {
                match (node.direction, direction) {
                    (Direction::Up, Direction::Down) | (Direction::Down, Direction::Up) => cost += 2000,
                    (Direction::Left, Direction::Right) | (Direction::Right, Direction::Left) => cost += 2000,
                    _ => cost += 1000,
                }
            }
            if grid[y][x] && !visited.contains(&(x, y)) {
                heap.push(Node::new(x, y, cost, direction));
                visited.insert((x, y));
            }
        }
    }
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut end = (0, 0);
    let mut heap = BinaryHeap::new();
    let mut visited = HashMap::new();

    let grid = input.lines().enumerate().map(|(y, line)| {
        line.chars().enumerate().map(|(x, c)| {
            match c {
                '.' => true,
                '#' => false,
                'S' => {
                    heap.push(Node::new(x, y, 0, Direction::Right));
                    visited.insert((x,y, Direction::Right), 0);
                    true
                },
                'E' => {
                    end = (x, y);
                    true
                },
                _ => panic!("unexpected character"),
            }
        }).collect::<Vec<bool>>()
    }).collect::<Vec<Vec<bool>>>();

    let mut best_path = HashSet::new();
    let mut lowest_cost = u32::MAX;

    while let Some(node) = heap.pop() {
        if (node.x, node.y) == end && node.cost <= lowest_cost{
            lowest_cost = node.cost;
            best_path.extend(node.visited.clone());
        }

        let directions = vec![
            (node.x + 1, node.y, Direction::Right),
            (node.x, node.y + 1, Direction::Down),
            (node.x - 1, node.y, Direction::Left),
            (node.x, node.y - 1, Direction::Up),
        ];

        for (x, y, direction) in directions {
            let mut cost = node.cost + 1;
            if node.direction != direction {
                match (node.direction, direction) {
                    (Direction::Up, Direction::Down) | (Direction::Down, Direction::Up) => cost += 2000,
                    (Direction::Left, Direction::Right) | (Direction::Right, Direction::Left) => cost += 2000,
                    _ => cost += 1000,
                }
            }
            let potentional_dupe = visited.get(&(x, y, direction));
            if grid[y][x] && (potentional_dupe.is_none() || *potentional_dupe.unwrap() >= cost) {
                heap.push(Node::new_from_node(&node, x, y, cost, direction));
                visited.insert((x, y, direction), cost);
            }
        }
    }
    Some(best_path.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11048));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
