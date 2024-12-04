use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let pattern = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    Some(
        pattern
            .captures_iter(input)
            .map(|c| c.extract())
            .map(|(_, [a, b])| a.parse::<u32>().unwrap() * b.parse::<u32>().unwrap())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let pattern = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();
    let mut should_do = true;
    let mut accum = 0;
    pattern
        .captures_iter(input)
        .for_each(|c| match c.get(0).unwrap().as_str() {
            "do()" => should_do = true,
            "don't()" => should_do = false,
            _ => {
                if should_do {
                    accum += c.get(1).unwrap().as_str().parse::<u32>().unwrap()
                        * c.get(2).unwrap().as_str().parse::<u32>().unwrap()
                }
            }
        });
    Some(accum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
