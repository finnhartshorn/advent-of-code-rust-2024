use itertools::Itertools;
use std::iter::repeat_n;

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .map(|l| {
                let mut splitter = l.split(": ");
                let expected_result: u64 = splitter.next().unwrap().parse().unwrap();
                let nums = splitter
                    .next()
                    .unwrap()
                    .split(' ')
                    .map(|n| n.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>();
                for permutation in
                    repeat_n(['+', '*'].iter(), nums.len() - 1).multi_cartesian_product()
                {
                    let mut result = nums[0];
                    for (i, &num) in nums.iter().skip(1).enumerate() {
                        match permutation[i] {
                            '+' => result += num,
                            '*' => result *= num,
                            _ => panic!("Invalid operator"),
                        }
                    }
                    if result == expected_result {
                        return expected_result;
                    }
                }
                0
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .map(|l| {
                let mut splitter = l.split(": ");
                let expected_result: u64 = splitter.next().unwrap().parse().unwrap();
                let nums = splitter
                    .next()
                    .unwrap()
                    .split(' ')
                    .map(|n| n.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>();
                for permutation in
                    repeat_n(['+', '*', '|'].iter(), nums.len() - 1).multi_cartesian_product()
                {
                    let mut result = nums[0];
                    for (i, &num) in nums.iter().skip(1).enumerate() {
                        match permutation[i] {
                            '+' => result += num,
                            '*' => result *= num,
                            '|' => result = concat(result, num),
                            _ => panic!("Invalid operator"),
                        }
                    }
                    if result == expected_result {
                        return expected_result;
                    }
                }
                0
            })
            .sum(),
    )
}

fn concat(a: u64, b: u64) -> u64 {
    a * 10u64.pow(b.ilog10() + 1) + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
