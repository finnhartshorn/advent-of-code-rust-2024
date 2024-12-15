use advent_of_code::get_cardinal_directions;
use std::collections::HashSet;

advent_of_code::solution!(12);

struct Region {
    id: char,
    perimeter: u32,
    area: u32,
    coord_set: HashSet<(usize, usize)>,
}

impl Region {
    fn new(id: char) -> Self {
        Self {
            id,
            perimeter: 0,
            area: 1,
            coord_set: HashSet::new(),
        }
    }
    fn contains(&self, x: usize, y: usize) -> bool {
        self.coord_set.contains(&(x, y))
    }
}
pub fn part_one(input: &str) -> Option<u32> {
    let mut seen = HashSet::<(usize, usize)>::new();
    let mut regions = Vec::<Region>::new();
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if seen.contains(&(x, y)) {
                continue;
            }
            let mut frontier = vec![(x, y)];
            seen.insert((x, y));
            let mut region = Region::new(grid[y][x]);
            while let Some((x, y)) = frontier.pop() {
                let adjacent = get_cardinal_directions((x, y), grid[y].len(), grid.len());
                if adjacent.len() < 4 {
                    region.perimeter += 4 - adjacent.len() as u32;
                }
                adjacent.iter().for_each(|(x_2, y_2)| {
                    if seen.contains(&(*x_2, *y_2)) {
                        if grid[*y_2][*x_2] != region.id {
                            region.perimeter += 1;
                        }
                        return;
                    }
                    if grid[*y_2][*x_2] == region.id {
                        frontier.push((*x_2, *y_2));
                        seen.insert((*x_2, *y_2));
                        region.area += 1;
                    } else {
                        region.perimeter += 1;
                    }
                });
            }
            regions.push(region);
        }
    }
    Some(
        regions
            .iter()
            .map(|region| region.perimeter * region.area)
            .sum::<u32>(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut seen = HashSet::<(usize, usize)>::new();
    let mut regions = Vec::<Region>::new();
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if seen.contains(&(x, y)) {
                continue;
            }
            let mut frontier = vec![(x, y)];
            seen.insert((x, y));
            let mut region = Region::new(grid[y][x]);
            region.coord_set.insert((x, y));
            while let Some((x, y)) = frontier.pop() {
                let adjacent = get_cardinal_directions((x, y), grid[y].len(), grid.len());
                if adjacent.len() < 4 {
                    region.coord_set.insert((x, y));
                }
                adjacent.iter().for_each(|(x_2, y_2)| {
                    if seen.contains(&(*x_2, *y_2)) {
                        return;
                    }
                    if grid[*y_2][*x_2] == region.id {
                        frontier.push((*x_2, *y_2));
                        seen.insert((*x_2, *y_2));
                        region.area += 1;
                        region.coord_set.insert((*x_2, *y_2));
                    }
                });
            }
            regions.push(region);
        }
    }
    Some(
        regions
            .iter_mut()
            .map(|region| {
                let mut sides = 0;
                for (x, y) in region.coord_set.iter() {
                    {
                        // Corner 1
                        let a = *x > 0 && region.contains(*x - 1, *y);
                        let b = *x > 0 && *y > 0 && region.contains(*x - 1, *y - 1);
                        let c = *y > 0 && region.contains(*x, *y - 1);
                        if (a == c) && (!a || !b) {
                            sides += 1
                        }
                    }
                    {
                        // Corner 2
                        let a = *y > 0 && region.contains(*x, *y - 1);
                        let b = *y > 0 && region.contains(*x + 1, *y - 1);
                        let c = region.contains(*x + 1, *y);
                        if a == c && (!a || !b) {
                            sides += 1
                        }
                    }
                    {
                        // Corner 3
                        let a = region.contains(*x + 1, *y);
                        let b = region.contains(*x + 1, *y + 1);
                        let c = region.contains(*x, *y + 1);
                        if a == c && (!a || !b) {
                            sides += 1
                        }
                    }
                    {
                        // Corner 4
                        let a = region.contains(*x, *y + 1);
                        let b = *x > 0 && region.contains(*x - 1, *y + 1);
                        let c = *x > 0 && region.contains(*x - 1, *y);
                        if a == c && (!a || !b) {
                            sides += 1
                        }
                    }
                }
                sides * region.area
            })
            .sum::<u32>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
