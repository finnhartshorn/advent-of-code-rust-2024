use advent_of_code::Direction;
use std::collections::HashSet;

advent_of_code::solution!(15);

struct Robot {
    x: u32,
    y: u32,
}

impl Robot {
    fn _move(&mut self, direction: &Direction) {
        match direction {
            Direction::Up => self.y -= 1,
            Direction::Down => self.y += 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }
    }
    fn get_next_pos(&mut self, direction: &Direction) -> (u32, u32) {
        match direction {
            Direction::Up => (self.x, self.y - 1),
            Direction::Down => (self.x, self.y + 1),
            Direction::Left => (self.x - 1, self.y),
            Direction::Right => (self.x + 1, self.y),
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut splitter = input.split("\n\n");
    let mut robot = Robot { x: 0, y: 0 };
    let mut grid: Vec<Vec<char>> = splitter
        .next()
        .unwrap()
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == '@' {
                        robot = Robot {
                            x: x as u32,
                            y: y as u32,
                        };
                        ' '
                    } else if c == '.' {
                        ' '
                    } else {
                        c
                    }
                })
                .collect::<Vec<char>>()
        })
        .collect();
    let directions = splitter
        .next()
        .unwrap()
        .lines()
        .flat_map(|line| {
            line.chars()
                .map(|c| match c {
                    '^' => Direction::Up,
                    'v' => Direction::Down,
                    '<' => Direction::Left,
                    '>' => Direction::Right,
                    _ => panic!("Invalid direction"),
                })
                .collect::<Vec<Direction>>()
        })
        .collect::<Vec<Direction>>();

    directions.iter().for_each(|direction| {
        let next_pos = robot.get_next_pos(direction);
        match grid[next_pos.1 as usize][next_pos.0 as usize] {
            ' ' => robot._move(direction),
            '#' => (),
            'O' => {
                if let Some(next_open_spot) = find_next_open_spot(&grid, next_pos, direction) {
                    robot._move(direction);
                    grid[next_pos.1 as usize][next_pos.0 as usize] = ' ';
                    grid[next_open_spot.1 as usize][next_open_spot.0 as usize] = 'O';
                }
            }
            _ => panic!("Invalid character"),
        }
    });

    Some(
        grid.iter()
            .enumerate()
            .map(|(y, line)| {
                line.iter()
                    .enumerate()
                    .map(|(x, c)| {
                        if *c == 'O' {
                            let x = x as u32;
                            let y = y as u32;
                            y * 100 + x
                        } else {
                            0
                        }
                    })
                    .sum::<u32>()
            })
            .sum::<u32>(),
    )
}

fn find_next_open_spot(
    grid: &[Vec<char>],
    coord: (u32, u32),
    direction: &Direction,
) -> Option<(u32, u32)> {
    let mut next_pos = coord;
    loop {
        next_pos = match direction {
            Direction::Up => (next_pos.0, next_pos.1 - 1),
            Direction::Down => (next_pos.0, next_pos.1 + 1),
            Direction::Left => (next_pos.0 - 1, next_pos.1),
            Direction::Right => (next_pos.0 + 1, next_pos.1),
        };
        match grid[next_pos.1 as usize][next_pos.0 as usize] {
            'O' | '[' | ']' => continue,
            '#' => return None,
            ' ' => return Some(next_pos),
            _ => {
                panic!("Invalid character")
            }
        }
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut splitter = input.split("\n\n");
    let mut robot = Robot { x: 0, y: 0 };
    let mut grid: Vec<Vec<char>> = splitter
        .next()
        .unwrap()
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .flat_map(|(x, c)| match c {
                    '@' => {
                        robot = Robot {
                            x: (x * 2) as u32,
                            y: y as u32,
                        };
                        [' ', ' ']
                    }
                    '.' => [' ', ' '],
                    'O' => ['[', ']'],
                    '#' => ['#', '#'],
                    _ => panic!("Invalid character"),
                })
                .collect::<Vec<char>>()
        })
        .collect();
    let directions = splitter
        .next()
        .unwrap()
        .lines()
        .flat_map(|line| {
            line.chars()
                .map(|c| match c {
                    '^' => Direction::Up,
                    'v' => Direction::Down,
                    '<' => Direction::Left,
                    '>' => Direction::Right,
                    _ => panic!("Invalid direction"),
                })
                .collect::<Vec<Direction>>()
        })
        .collect::<Vec<Direction>>();

    directions.iter().for_each(|direction| {
        let next_pos = robot.get_next_pos(direction);
        match grid[next_pos.1 as usize][next_pos.0 as usize] {
            ' ' => robot._move(direction),
            '#' => (),
            '[' | ']' => match direction {
                Direction::Up | Direction::Down => {
                    if attempt_move_boxes_vertically(&mut grid, (robot.x, robot.y), direction) {
                        robot._move(direction);
                    }
                }
                Direction::Left | Direction::Right => {
                    if find_next_open_spot(&grid, next_pos, direction).is_some() {
                        robot._move(direction);
                        shift_boxes_horizontally(&mut grid, (robot.x, robot.y), direction);
                    }
                }
            },
            _ => panic!("Invalid character"),
        }
    });

    // draw_grid(&grid, (robot.x, robot.y));

    Some(
        grid.iter()
            .enumerate()
            .map(|(y, line)| {
                line.iter()
                    .enumerate()
                    .map(|(x, c)| {
                        if *c == '[' {
                            let x = x as u32;
                            let y = y as u32;
                            y * 100 + x
                        } else {
                            0
                        }
                    })
                    .sum::<u32>()
            })
            .sum::<u32>(),
    )
}

// Assumes robot coord is passed in not box coord
fn attempt_move_boxes_vertically(
    grid: &mut [Vec<char>],
    coord: (u32, u32),
    direction: &Direction,
) -> bool {
    let delta: i32 = match direction {
        Direction::Up => -1,
        Direction::Down => 1,
        _ => panic!("Invalid direction"),
    };
    let mut handled_box_coords = HashSet::<(u32, u32)>::new();
    let mut frontier = vec![(coord.0, coord.1)];
    let mut box_stack = Vec::new();
    while !frontier.is_empty() {
        let mut new_frontier = Vec::new();
        while let Some(next_coord) = frontier.pop() {
            match grid[(next_coord.1 as i32 + delta) as usize][next_coord.0 as usize] {
                ' ' => continue,
                '#' => return false,
                '[' => {
                    if handled_box_coords
                        .insert((next_coord.0, (next_coord.1 as i32 + delta) as u32))
                    {
                        new_frontier.push((next_coord.0, (next_coord.1 as i32 + delta) as u32));
                        box_stack.push((next_coord.0, (next_coord.1 as i32 + delta) as u32));
                    }
                    if handled_box_coords
                        .insert((next_coord.0 + 1, (next_coord.1 as i32 + delta) as u32))
                    {
                        new_frontier.push((next_coord.0 + 1, (next_coord.1 as i32 + delta) as u32));
                        box_stack.push((next_coord.0 + 1, (next_coord.1 as i32 + delta) as u32));
                    }
                }
                ']' => {
                    if handled_box_coords
                        .insert((next_coord.0, (next_coord.1 as i32 + delta) as u32))
                    {
                        new_frontier.push((next_coord.0, (next_coord.1 as i32 + delta) as u32));
                        box_stack.push((next_coord.0, (next_coord.1 as i32 + delta) as u32));
                    }
                    if handled_box_coords
                        .insert((next_coord.0 - 1, (next_coord.1 as i32 + delta) as u32))
                    {
                        new_frontier.push((next_coord.0 - 1, (next_coord.1 as i32 + delta) as u32));
                        box_stack.push((next_coord.0 - 1, (next_coord.1 as i32 + delta) as u32));
                    }
                }
                _ => panic!("Invalid character"),
            }
        }
        frontier = new_frontier;
    }

    while let Some(_box) = box_stack.pop() {
        grid[(_box.1 as i32 + delta) as usize][_box.0 as usize] =
            grid[_box.1 as usize][_box.0 as usize];
        grid[_box.1 as usize][_box.0 as usize] = ' ';
    }

    true
}

// Assumes boxes can be moved (will not hit a wall)
fn shift_boxes_horizontally(grid: &mut [Vec<char>], coord: (u32, u32), direction: &Direction) {
    let (delta, end_char) = match direction {
        Direction::Left => (-1, '['),
        Direction::Right => (1, ']'),
        _ => panic!("Invalid direction"),
    };
    let mut next_pos = (coord.0 as i32 + delta, coord.1 as i32);
    loop {
        match grid[next_pos.1 as usize][next_pos.0 as usize] {
            ' ' => {
                grid[next_pos.1 as usize][next_pos.0 as usize] = end_char;
                grid[coord.1 as usize][coord.0 as usize] = ' ';
                return;
            }
            '[' => grid[next_pos.1 as usize][next_pos.0 as usize] = ']',
            ']' => grid[next_pos.1 as usize][next_pos.0 as usize] = '[',
            _ => panic!("Invalid character"),
        }
        next_pos = (next_pos.0 + delta, next_pos.1);
    }
}

#[allow(dead_code)]
fn draw_grid(grid: &[Vec<char>], robot_coords: (u32, u32)) {
    grid.iter().enumerate().for_each(|(y, line)| {
        line.iter().enumerate().for_each(|(x, c)| {
            if x == robot_coords.0 as usize && y == robot_coords.1 as usize {
                print!("@");
            } else {
                print!("{}", c);
            }
        });
        println!();
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9021));
    }
}
