use advent_of_code::get_cardinal_directions;
use itertools::Itertools;

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u32> {
    let mut trailheads: Vec<Vec<(usize, usize)>> = Vec::new();
    let grid = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    let digit = c.to_digit(10).unwrap() as usize;
                    if digit == 0 {
                        trailheads.push(vec![(x, y)]);
                    }
                    digit
                })
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    let mut frontier = Vec::new();
    for height in 1..=9 {
        for trailhead in trailheads.iter() {
            let mut inner_frontier = Vec::new();
            for (f_x, f_y) in trailhead {
                for (x, y) in get_cardinal_directions((*f_x, *f_y), grid[0].len(), grid.len()) {
                    if grid[y][x] == height {
                        inner_frontier.push((x, y));
                    }
                }
            }
            frontier.push(inner_frontier)
        }
        trailheads = frontier.clone();
        frontier.clear();
    }
    Some(
        trailheads
            .iter()
            .map(|trailhead| trailhead.iter().unique().count() as u32)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut trailheads: Vec<Vec<(usize, usize)>> = Vec::new();
    let grid = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    let digit = c.to_digit(10).unwrap() as usize;
                    if digit == 0 {
                        trailheads.push(vec![(x, y)]);
                    }
                    digit
                })
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    let mut frontier = Vec::new();
    for height in 1..=9 {
        for trailhead in trailheads.iter() {
            let mut inner_frontier = Vec::new();
            for (f_x, f_y) in trailhead {
                for (x, y) in get_cardinal_directions((*f_x, *f_y), grid[0].len(), grid.len()) {
                    if grid[y][x] == height {
                        inner_frontier.push((x, y));
                    }
                }
            }
            frontier.push(inner_frontier)
        }
        trailheads = frontier.clone();
        frontier.clear();
    }
    Some(
        trailheads
            .iter()
            .map(|trailhead| trailhead.len() as u32)
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
