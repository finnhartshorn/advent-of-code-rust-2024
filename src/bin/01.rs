use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut list_1 = Vec::new();
    let mut list_2 = Vec::new();
    input.lines().for_each(|line| {
        let mut parts = line.split("   ");
        list_1.push(parts.next().unwrap().parse::<u32>().unwrap());
        list_2.push(parts.next().unwrap().parse::<u32>().unwrap());
    });
    list_1.sort();
    list_2.sort();
    Some(list_1.iter().zip(list_2.iter()).map(|(a, b)| a.abs_diff(*b)).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut list_1 = Vec::new();
    let mut counts = HashMap::new();
    input.lines().for_each(|line| {
        let mut parts = line.split("   ");
        list_1.push(parts.next().unwrap().parse::<u32>().unwrap());
        counts.entry(parts.next().unwrap().parse::<u32>().unwrap()).and_modify(|e| *e += 1).or_insert(1);
    });
    Some(list_1.iter().map(|a| a * counts.get(a).unwrap_or(&0)).sum())
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
