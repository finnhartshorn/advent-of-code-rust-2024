advent_of_code::solution!(4);

enum Direction {
    Horizontal,
    Vertical,
    DiagonalRight,
    DiagonalLeft,
}
pub fn part_one(input: &str) -> Option<u32> {
    let xmas = "XMAS".to_string();
    let samx = "SAMX".to_string();
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let height = &grid.len();
    let width = &grid[0].len();
    let mut count = 0;
    for row in 0..*height {
        for col in 0..*width {
            if grid[row][col] != 'X' && grid[row][col] != 'S' {
                continue;
            }
            let can_h = col < *width - 3;
            let can_v = row < *height - 3;
            let can_nh = col >= 3;
            if can_h {
                let word = get_word(&grid, row, col, 4, Direction::Horizontal);
                if word == xmas || word == samx {
                    count += 1;
                }
            }
            if can_v {
                let word = get_word(&grid, row, col, 4, Direction::Vertical);
                if word == xmas || word == samx {
                    count += 1;
                }
            }
            if can_v && can_h {
                let word = get_word(&grid, row, col, 4, Direction::DiagonalRight);
                if word == xmas || word == samx {
                    count += 1;
                }
            }
            if can_v && can_nh {
                let word = get_word(&grid, row, col, 4, Direction::DiagonalLeft);
                if word == xmas || word == samx {
                    count += 1;
                }
            }
        }
    }
    Some(count)
}

fn get_word(
    grid: &[Vec<char>],
    row: usize,
    col: usize,
    size: usize,
    direction: Direction,
) -> String {
    let mut word = String::new();
    word.push(grid[row][col]);
    match direction {
        Direction::Horizontal => {
            for i in 1..size {
                word.push(grid[row][col + i]);
            }
        }
        Direction::Vertical => {
            for i in 1..size {
                word.push(grid[row + i][col]);
            }
        }
        Direction::DiagonalRight => {
            for i in 1..size {
                word.push(grid[row + i][col + i]);
            }
        }
        Direction::DiagonalLeft => {
            for i in 1..size {
                word.push(grid[row + i][col - i]);
            }
        }
    }
    word
}

pub fn part_two(input: &str) -> Option<u32> {
    let mas = "MAS".to_string();
    let sam = "SAM".to_string();
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let height = &grid.len();
    let width = &grid[0].len();
    let mut count = 0;
    for row in 0..*height {
        for col in 0..*width {
            if grid[row][col] != 'M' && grid[row][col] != 'S' {
                continue;
            }

            let can_h = col < *width - 2;
            let can_v = row < *height - 2;
            if !can_h || !can_v {
                continue;
            }
            let right_word = get_word(&grid, row, col, 3, Direction::DiagonalRight);
            if right_word == mas || right_word == sam {
                let left_word = get_word(&grid, row, col + 2, 3, Direction::DiagonalLeft);
                if left_word == mas || left_word == sam {
                    count += 1;
                }
            }
        }
    }
    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
