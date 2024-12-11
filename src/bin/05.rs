use std::collections::{HashMap, HashSet};

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let mut before_map = HashMap::new();
    let mut splitter = input.split("\n\n");
    for line in splitter.next()?.lines() {
        let mut chars = line.split('|');
        let a = chars.next()?.parse::<u32>().unwrap();
        let b = chars.next()?.parse::<u32>().unwrap();
        before_map.entry(a).or_insert(HashSet::new()).insert(b);
    }
    let mut sum = 0;
    'outer: for line in splitter.next()?.lines() {
        let pages: Vec<u32> = line.split(',').map(|n| n.parse::<u32>().unwrap()).collect();
        for i in 1..pages.len() {
            let Some(conflict_set) = before_map.get(&pages[i]) else {
                continue
            };
            for j in 0..i {
                if conflict_set.contains(&pages[j]) {
                    continue 'outer;
                }
            }
        }
        sum += pages[(pages.len()) / 2]
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut before_map = HashMap::new();
    let mut splitter = input.split("\n\n");
    for line in splitter.next()?.lines() {
        let mut chars = line.split('|');
        let a = chars.next()?.parse::<u32>().unwrap();
        let b = chars.next()?.parse::<u32>().unwrap();
        before_map.entry(a).or_insert(HashSet::new()).insert(b);
    }
    let mut sum = 0;
    for line in splitter.next()?.lines() {
        let mut fixed = false;
        let mut pages: Vec<u32> = line.split(',').map(|n| n.parse::<u32>().unwrap()).collect();
        let mut i = 1;
        while i < pages.len() {
            let Some(conflict_set) = before_map.get(&pages[i]) else {
                continue
            };
            for j in 0..i {
                if conflict_set.contains(&pages[j]) {
                    let element = pages.remove(i);
                    pages.insert(j, element);
                    i = j;
                    fixed = true;
                    break;
                }
            }
            i += 1;
        }
        if fixed {
            sum += pages[(pages.len()) / 2]
        }
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
