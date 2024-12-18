use std::cmp::PartialEq;
use std::collections::HashSet;

advent_of_code::solution!(6);

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

#[derive(Clone)]
struct Guard {
    pos: (usize, usize),
    direction: Direction,
}

impl Guard {
    fn new() -> Self {
        Self {
            pos: (0, 0),
            direction: Direction::Up,
        }
    }

    fn turn(&mut self) {
        self.direction = self.direction.turn();
    }

    fn move_forward(&mut self) {
        match self.direction {
            Direction::Up => self.pos.1 -= 1,
            Direction::Down => self.pos.1 += 1,
            Direction::Left => self.pos.0 -= 1,
            Direction::Right => self.pos.0 += 1,
        }
    }

    fn next_pos(&self, max_x: usize, max_y: usize) -> Option<(usize, usize)> {
        Some(match self.direction {
            Direction::Up => {
                if self.pos.1 == 0 {
                    return None;
                }
                (self.pos.0, self.pos.1 - 1)
            }
            Direction::Down => {
                if self.pos.1 >= max_y {
                    return None;
                }
                (self.pos.0, self.pos.1 + 1)
            }
            Direction::Left => {
                if self.pos.0 == 0 {
                    return None;
                }
                (self.pos.0 - 1, self.pos.1)
            }
            Direction::Right => {
                if self.pos.0 >= max_x {
                    return None;
                }
                (self.pos.0 + 1, self.pos.1)
            }
        })
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid: Vec<Vec<bool>> = vec![];
    let mut guard = Guard::new();
    for (y, line) in input.lines().enumerate() {
        let mut row = vec![];
        for (x, c) in line.chars().enumerate() {
            let status = match c {
                '.' => false,
                '#' => true,
                '^' => {
                    guard.pos = (x, y);
                    false
                }
                _ => panic!("Invalid character"),
            };
            row.push(status);
        }
        grid.push(row);
    }

    let mut visited = HashSet::<(usize, usize)>::new();
    visited.insert(guard.pos);

    while let Some(next_pos) = guard.next_pos(grid[0].len() - 1, grid.len() - 1) {
        let (nx, ny) = next_pos;
        if grid[ny][nx] {
            guard.turn();
            continue;
        }
        guard.move_forward();
        if !visited.contains(&(nx, ny)) {
            visited.insert((nx, ny));
            continue;
        }
    }
    Some(visited.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid: Vec<Vec<bool>> = vec![];
    let mut guard = Guard::new();
    for (y, line) in input.lines().enumerate() {
        let mut row = vec![];
        for (x, c) in line.chars().enumerate() {
            let status = match c {
                '.' => false,
                '#' => true,
                '^' => {
                    guard.pos = (x, y);
                    false
                }
                _ => panic!("Invalid character"),
            };
            row.push(status);
        }
        grid.push(row);
    }

    // Simulate the path once so we know possible positions for obstructions
    let mut visited = HashSet::<(usize, usize)>::new();
    visited.insert(guard.pos);

    let mut visited_guard = guard.clone();

    while let Some(next_pos) = visited_guard.next_pos(grid[0].len() - 1, grid.len() - 1) {
        let (nx, ny) = next_pos;
        if grid[ny][nx] {
            visited_guard.turn();
            continue;
        }
        visited_guard.move_forward();
        if !visited.contains(&(nx, ny)) {
            visited.insert((nx, ny));
            continue;
        }
    }

    visited.remove(&guard.pos);

    let mut sum = 0;

    visited.iter().for_each(|(ox, oy)| {
        let mut seen_position_directions = HashSet::<(usize, usize, Direction)>::new();
        let mut inner_guard = guard.clone();
        grid[*oy][*ox] = true;
        while let Some(next_pos) =
            inner_guard.next_pos(grid[0].len() - 1, grid.len() - 1)
        {
            let (nx, ny) = next_pos;
            if grid[ny][nx] {
                inner_guard.turn();
                continue;
            }
            inner_guard.move_forward();
            if !grid[ny][nx]
                && !seen_position_directions.contains(&(
                    inner_guard.pos.0,
                    inner_guard.pos.1,
                    inner_guard.direction.clone(),
                ))
            {
                seen_position_directions.insert((
                    inner_guard.pos.0,
                    inner_guard.pos.1,
                    inner_guard.direction.clone(),
                ));
            } else if seen_position_directions.contains(&(
                inner_guard.pos.0,
                inner_guard.pos.1,
                inner_guard.direction.clone(),
            )) {
                grid[*oy][*ox] = false;
                sum += 1;
                return;
            }
        }
        grid[*oy][*ox] = false;
    });
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
