use itertools::Itertools;

advent_of_code::solution!(17);

pub fn part_one(input: &str) -> Option<String> {
    let mut splitter = input.lines();
    let register_a = splitter
        .next()
        .unwrap()
        .strip_prefix("Register A: ")
        .unwrap()
        .parse::<usize>()
        .unwrap();

    let output = program_iter(register_a);

    Some(output.into_iter().join(","))
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut splitter = input.lines();
    splitter.next().unwrap(); // Skip Register A line
    splitter.next().unwrap(); // Skip Register B line
    splitter.next().unwrap(); // Skip Register C line
    splitter.next().unwrap(); // Skip empty line

    let instructions = splitter
        .next()
        .unwrap()
        .strip_prefix("Program: ")
        .unwrap()
        .split(",")
        .map(|c| c.parse::<u8>().unwrap())
        .collect::<Vec<u8>>();

    let mut j = 0;

    let mut matched = 0;

    loop {
        if single_run(j) == instructions[instructions.len() - (matched + 1)] {
            matched += 1;
            if matched == instructions.len() {
                return Some(j as u64);
            }
            j *= 8;
        } else {
            j += 1;
        }
    }
}

fn program_iter(init_a: usize) -> Vec<u8> {
    let mut i = 0;
    let mut a = init_a;
    while a > 0 {
        a >>= 3;
        i += 1;
    }
    (0..i)
        .scan(init_a, |state, _| {
            let b = (*state % 8) ^ 2;
            let c = *state >> b;
            *state >>= 3;
            Some(((b ^ 7 ^ c) % 8) as u8)
        })
        .collect()
}

fn single_run(init_a: usize) -> u8 {
    let b = (init_a % 8) ^ 2;
    let c = init_a >> b;
    ((b ^ 7 ^ c) % 8) as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("7,1,2,7".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(178229));
    }
}
