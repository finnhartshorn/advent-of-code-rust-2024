use itertools::iproduct;
use std::cmp::PartialEq;
use std::collections::HashSet;

advent_of_code::solution!(6);

#[derive(PartialEq, Clone, Debug)]
enum Status {
    Empty,
    Visited,
    Obstacle,
}

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
    let mut grid: Vec<Vec<Status>> = vec![];
    let mut guard = Guard::new();
    let mut sum = 1;
    for (y, line) in input.lines().enumerate() {
        let mut row = vec![];
        for (x, c) in line.chars().enumerate() {
            let status = match c {
                '.' => Status::Empty,
                '#' => Status::Obstacle,
                '^' => {
                    guard.pos = (x, y);
                    Status::Visited
                }
                _ => panic!("Invalid character"),
            };
            row.push(status);
        }
        grid.push(row);
    }
    while let Some(next_pos) = guard.next_pos(grid[0].len() - 1, grid.len() - 1) {
        let (nx, ny) = next_pos;
        if grid[ny][nx] == Status::Obstacle {
            guard.turn();
            continue;
        }
        guard.move_forward();
        if grid[ny][nx] == Status::Empty {
            grid[ny][nx] = Status::Visited;
            sum += 1;
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid: Vec<Vec<Status>> = vec![];
    let mut guard = Guard::new();
    for (y, line) in input.lines().enumerate() {
        let mut row = vec![];
        for (x, c) in line.chars().enumerate() {
            let status = match c {
                '.' => Status::Empty,
                '#' => Status::Obstacle,
                '^' => {
                    guard.pos = (x, y);
                    Status::Visited
                }
                _ => panic!("Invalid character"),
            };
            row.push(status);
        }
        grid.push(row);
    }
    Some(
        iproduct!(0..grid.len(), 0..grid[0].len())
            .filter_map(|(oy, ox)| {
                let mut seen_position_directions = HashSet::<(usize, usize, Direction)>::new();
                let mut inner_guard = guard.clone();
                let mut inner_grid = grid.clone();
                match grid[oy][ox] {
                    Status::Empty => (inner_grid[oy][ox] = Status::Obstacle,),
                    Status::Obstacle => return None,
                    Status::Visited => return None,
                };
                while let Some(next_pos) =
                    inner_guard.next_pos(inner_grid[0].len() - 1, inner_grid.len() - 1)
                {
                    let (nx, ny) = next_pos;
                    if inner_grid[ny][nx] == Status::Obstacle {
                        inner_guard.turn();
                        continue;
                    }
                    inner_guard.move_forward();
                    if inner_grid[ny][nx] == Status::Empty {
                        inner_grid[ny][nx] = Status::Visited;
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
                        return Some(());
                    }
                }
                None
            })
            .count() as u32,
    )
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
