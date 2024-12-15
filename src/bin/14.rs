use std::collections::HashSet;

advent_of_code::solution!(14);

#[derive(Debug)]
struct Robot {
    x: i32,
    y: i32,
    delta_x: i32,
    delta_y: i32,
}

impl Robot {
    fn move_forward(&mut self, max_x: i32, max_y: i32) {
        self.x += self.delta_x;
        if self.x > max_x {
            self.x %= max_x + 1;
        }
        if self.x < 0 {
            self.x += max_x + 1;
        }
        self.y += self.delta_y;
        if self.y > max_y {
            self.y %= max_y + 1;
        }
        if self.y < 0 {
            self.y += max_y + 1;
        }
    }

    fn quadrant(&self, max_x: i32, max_y: i32) -> Option<u32> {
        let mid_x = max_x / 2;
        let mid_y = max_y / 2;
        if self.x < mid_x && self.y < mid_y {
            Some(0)
        } else if self.x > mid_x && self.y < mid_y {
            Some(1)
        } else if self.x < mid_x && self.y > mid_y {
            Some(2)
        } else if self.x > mid_x && self.y > mid_y {
            Some(3)
        } else {
            None
        }
    }
}
pub fn part_one(input: &str) -> Option<u32> {
    let mut robots: Vec<Robot> = input
        .lines()
        .map(|line| {
            let mut splitter = line.split(' ');
            let mut coord_splitter1 = splitter
                .next()
                .unwrap()
                .strip_prefix("p=")
                .unwrap()
                .split(',');
            let x = coord_splitter1.next().unwrap().parse().unwrap();
            let y = coord_splitter1.next().unwrap().parse().unwrap();
            let mut coord_splitter1 = splitter
                .next()
                .unwrap()
                .strip_prefix("v=")
                .unwrap()
                .split(',');
            let delta_x = coord_splitter1.next().unwrap().parse().unwrap();
            let delta_y = coord_splitter1.next().unwrap().parse().unwrap();
            Robot {
                x,
                y,
                delta_x,
                delta_y,
            }
        })
        .collect();
    let grid_bounds = if robots.len() == 12 {
        (10, 6)
    } else {
        (100, 102)
    };
    for _ in 0..100 {
        for robot in robots.iter_mut() {
            robot.move_forward(grid_bounds.0, grid_bounds.1);
        }
    }
    let mut quadrants = (0, 0, 0, 0);

    robots.iter().for_each(|robot| {
        if let Some(quadrant) = robot.quadrant(grid_bounds.0, grid_bounds.1) {
            match quadrant {
                0 => quadrants.0 += 1,
                1 => quadrants.1 += 1,
                2 => quadrants.2 += 1,
                3 => quadrants.3 += 1,
                _ => (),
            }
        }
    });

    Some(quadrants.0 * quadrants.1 * quadrants.2 * quadrants.3)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut robots: Vec<Robot> = input
        .lines()
        .map(|line| {
            let mut splitter = line.split(' ');
            let mut coord_splitter1 = splitter
                .next()
                .unwrap()
                .strip_prefix("p=")
                .unwrap()
                .split(',');
            let x = coord_splitter1.next().unwrap().parse().unwrap();
            let y = coord_splitter1.next().unwrap().parse().unwrap();
            let mut coord_splitter1 = splitter
                .next()
                .unwrap()
                .strip_prefix("v=")
                .unwrap()
                .split(',');
            let delta_x = coord_splitter1.next().unwrap().parse().unwrap();
            let delta_y = coord_splitter1.next().unwrap().parse().unwrap();
            Robot {
                x,
                y,
                delta_x,
                delta_y,
            }
        })
        .collect();
    let grid_bounds = if robots.len() == 12 {
        return None;
    } else {
        (100, 102)
    };
    let mut i = 0;
    loop {
        i += 1;
        let mut coords = HashSet::<(u32, u32)>::new();
        for robot in robots.iter_mut() {
            robot.move_forward(grid_bounds.0, grid_bounds.1);
            coords.insert((robot.x as u32, robot.y as u32));
        }
        // Slightly cheating here, the Christmas tree has a large border so we just look for a vertical run of 25 or more
        for coord in coords.iter() {
            let mut run = 1;
            let mut current_coord = *coord;
            loop {
                if coords.contains(&(current_coord.0, current_coord.1 + 1)) {
                    run += 1;
                    current_coord = (current_coord.0, current_coord.1 + 1);
                    if run >= 25 {
                        return Some(i);
                    }
                } else {
                    break;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
