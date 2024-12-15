use std::collections::HashMap;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u64> {
    Some(solution(input, 25))
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(solution(input, 75))
}

fn solution(input: &str, blinks: u32) -> u64 {
    let mut counts: HashMap<u64, u64> = HashMap::new();
    input.lines().next().unwrap().split(' ').for_each(|n| {
        counts.insert(n.parse::<u64>().unwrap(), 1);
    });
    for _ in 0..blinks {
        let mut new_counts: HashMap<u64, u64> = HashMap::new();
        for (n, count) in counts.iter() {
            if *n == 0 {
                new_counts
                    .entry(1)
                    .and_modify(|e| *e += *count)
                    .or_insert(*count);
            } else {
                let n_len = n.checked_ilog10().unwrap_or(0) + 1;
                if n_len % 2 == 0 {
                    let lower = n / 10_u64.pow(n_len / 2);
                    let upper = n % 10_u64.pow(n_len / 2);
                    new_counts
                        .entry(lower)
                        .and_modify(|e| *e += *count)
                        .or_insert(*count);
                    new_counts
                        .entry(upper)
                        .and_modify(|e| *e += *count)
                        .or_insert(*count);
                } else {
                    new_counts
                        .entry(n * 2024)
                        .and_modify(|e| *e += *count)
                        .or_insert(*count);
                }
            }
        }
        counts = new_counts;
    }
    counts.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
