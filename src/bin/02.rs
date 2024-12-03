use itertools::Itertools;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let mut increasing = false;
                let mut decreasing = false;
                let windows = line
                    .split_whitespace()
                    .map(|c| c.parse::<i32>().unwrap())
                    .tuple_windows();
                for (a, b) in windows {
                    if a - b >= 1 && a - b < 4 {
                        if increasing {
                            return 0;
                        } else {
                            decreasing = true;
                            continue;
                        }
                    } else if a - b <= -1 && a - b > -4 {
                        if decreasing {
                            return 0;
                        } else {
                            increasing = true;
                            continue;
                        }
                    }
                    return 0;
                }
                1
            })
            .sum(),
    )
}

fn is_line_safe(line: &[&i32]) -> bool {
    let mut increasing = false;
    let mut decreasing = false;
    for (a, b) in line.iter().tuple_windows() {
        if *a - *b >= 1 && *a - *b < 4 {
            if increasing {
                return false;
            } else {
                decreasing = true;
                continue;
            }
        } else if *a - *b <= -1 && *a - *b > -4 {
            if decreasing {
                return false;
            } else {
                increasing = true;
                continue;
            }
        }
        return false;
    }
    true
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let parsed_line: Vec<i32> = line
                    .split_whitespace()
                    .map(|c| c.parse::<i32>().unwrap())
                    .collect();
                let line_len = &parsed_line.len();
                for new_line in parsed_line.iter().combinations(line_len - 1) {
                    if is_line_safe(&new_line) {
                        return 1;
                    }
                }
                0
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
