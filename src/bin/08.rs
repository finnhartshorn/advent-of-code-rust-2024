use itertools::Itertools;
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let mut antenna_positions = HashMap::<char, Vec<(i32, i32)>>::new();
    let mut x_size = 0;
    let mut y_size = 0;
    input.lines().enumerate().for_each(|(y, line)| {
        y_size += 1;
        let mut inner_x_size = 0;
        line.chars().enumerate().for_each(|(x, c)| {
            inner_x_size += 1;
            if c != '.' {
                antenna_positions
                    .entry(c)
                    .or_default()
                    .push((x as i32, y as i32));
            }
        });
        x_size = inner_x_size;
    });
    let mut unique_antinode_positions = HashSet::<(usize, usize)>::new();
    for antenna in antenna_positions.keys() {
        let antennas = antenna_positions.get(antenna).unwrap();
        if antennas.len() < 2 {
            continue;
        }
        for combination in antennas.iter().combinations(2) {
            let &[(x1, y1), (x2, y2)] = combination.as_slice() else {
                panic!("Impossible condition")
            };
            let x_diff = *x2 - *x1;
            let y_diff = *y2 - *y1;
            let new_x1 = x1 - x_diff;
            let new_y1 = y1 - y_diff;
            let new_x2 = x2 + x_diff;
            let new_y2 = y2 + y_diff;
            if new_x1 >= 0 && new_y1 >= 0 && new_x1 < x_size && new_y1 < y_size {
                unique_antinode_positions.insert((new_x1 as usize, new_y1 as usize));
            }
            if new_x2 >= 0 && new_y2 >= 0 && new_x2 < x_size && new_y2 < y_size {
                unique_antinode_positions.insert((new_x2 as usize, new_y2 as usize));
            }
        }
    }
    Some(unique_antinode_positions.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut antenna_positions = HashMap::<char, Vec<(i32, i32)>>::new();
    let mut x_size = 0;
    let mut y_size = 0;
    input.lines().enumerate().for_each(|(y, line)| {
        y_size += 1;
        let mut inner_x_size = 0;
        line.chars().enumerate().for_each(|(x, c)| {
            inner_x_size += 1;
            if c != '.' {
                antenna_positions
                    .entry(c)
                    .or_default()
                    .push((x as i32, y as i32));
            }
        });
        x_size = inner_x_size;
    });
    let mut unique_antinode_positions = HashSet::<(usize, usize)>::new();
    for antenna in antenna_positions.keys() {
        let antennas = antenna_positions.get(antenna).unwrap();
        if antennas.len() < 2 {
            continue;
        }
        for combination in antennas.iter().combinations(2) {
            let &[(x1, y1), (x2, y2)] = combination.as_slice() else {
                panic!("Impossible condition")
            };
            let x_diff = *x2 - *x1;
            let y_diff = *y2 - *y1;
            unique_antinode_positions.insert((*x1 as usize, *y1 as usize));
            for i in 1..x_size {
                let new_x1 = x1 + i;
                let new_x2 = x1 - i;
                if (i * y_diff) % x_diff == 0 {
                    let y_step = (i * y_diff) / x_diff;
                    let new_y1 = y1 + y_step;
                    let new_y2 = y1 - y_step;
                    if new_x1 >= 0 && new_y1 >= 0 && new_x1 < x_size && new_y1 < y_size {
                        unique_antinode_positions.insert((new_x1 as usize, new_y1 as usize));
                    }
                    if new_x2 >= 0 && new_y2 >= 0 && new_x2 < x_size && new_y2 < y_size {
                        unique_antinode_positions.insert((new_x2 as usize, new_y2 as usize));
                    }
                }
            }
        }
    }
    Some(unique_antinode_positions.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
