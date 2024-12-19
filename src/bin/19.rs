use std::collections::HashMap;

advent_of_code::solution!(19);

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.lines();
    let towels: Vec<&str> = lines.next().unwrap().split(", ").collect();
    lines.next(); // Skip empty line

    let mut memo: HashMap<&str, bool> = HashMap::new();

    Some(
        lines
            .filter_map(|line| {
                if get_if_possible(line, &towels, &mut memo) {
                    Some(1)
                } else {
                    None
                }
            })
            .sum(),
    )
}

fn get_num_possible_combinations<'a>(
    towel: &'a str,
    possible_towels: &[&str],
    memo: &mut HashMap<&'a str, u64>,
) -> u64 {
    if towel.is_empty() {
        return 1;
    }
    if memo.contains_key(towel) {
        return memo[towel];
    }
    let p = possible_towels
        .iter()
        .map(|pt| {
            if let Some(new_towel) = towel.strip_prefix(pt) {
                get_num_possible_combinations(new_towel, possible_towels, memo)
            } else {
                0
            }
        })
        .sum();
    memo.insert(towel, p);
    p
}

fn get_if_possible<'a>(
    towel: &'a str,
    possible_towels: &[&str],
    memo: &mut HashMap<&'a str, bool>,
) -> bool {
    if towel.is_empty() {
        return true;
    }
    if memo.contains_key(towel) {
        return memo[towel];
    }
    let p = possible_towels.iter().any(|pt| {
        if let Some(new_towel) = towel.strip_prefix(pt) {
            get_if_possible(new_towel, possible_towels, memo)
        } else {
            false
        }
    });
    memo.insert(towel, p);
    p
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let towels: Vec<&str> = lines.next().unwrap().split(", ").collect();
    lines.next(); // Skip empty line

    let mut memo: HashMap<&str, u64> = HashMap::new();

    Some(
        lines
            .map(|line| get_num_possible_combinations(line, &towels, &mut memo))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
