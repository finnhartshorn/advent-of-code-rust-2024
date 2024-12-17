use itertools::Itertools;

advent_of_code::solution!(17);

pub fn part_one(input: &str) -> Option<String> {
    let mut splitter = input.lines();
    let mut register_a = splitter.next().unwrap().strip_prefix("Register A: ").unwrap().parse::<usize>().unwrap();
    let mut register_b = splitter.next().unwrap().strip_prefix("Register B: ").unwrap().parse::<usize>().unwrap();
    let mut register_c = splitter.next().unwrap().strip_prefix("Register C: ").unwrap().parse::<usize>().unwrap();
    splitter.next().unwrap(); // Skip empty line
    let mut instructions = splitter.next().unwrap().strip_prefix("Program: ").unwrap().split(",").map(|c| c.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let mut output = Vec::new();

    let mut i = 0;
    while i < instructions.len() {
        let instruction = instructions[i];
        let literal_operand = instructions[i+1];
        let combo_operand = match literal_operand {
            1 => 1,
            2 => 2,
            3 => 3,
            4 => register_a,
            5 => register_b,
            6 => register_c,
            7 => 7,
            8 => 8,
            0 => 9,
            _ => panic!("Invalid operand"),
        };

        match instruction {
            0 => {
                register_a = register_a / 2_usize.pow(combo_operand as u32);
            }
            1 => {
                register_b = register_b ^ literal_operand;
            }
            2 => {
                register_b = combo_operand % 8;
            }
            3 => {
                if register_a != 0 {
                    i = literal_operand;
                    continue;
                }
            }
            4 => {
                register_b = register_b ^ register_c;
            }
            5 => {
                output.push(combo_operand % 8)
            }
            6 => {
                register_b = register_a / 2_usize.pow(combo_operand as u32);
            }
            7 => {
                register_c = register_a / 2_usize.pow(combo_operand as u32);
            }
            _ => {}
        }

        i += 2;
    }
    Some(output.into_iter().join(","))
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut splitter = input.lines();
    let mut register_a = splitter.next().unwrap().strip_prefix("Register A: ").unwrap().parse::<usize>().unwrap();
    let mut register_b = splitter.next().unwrap().strip_prefix("Register B: ").unwrap().parse::<usize>().unwrap();
    let mut register_c = splitter.next().unwrap().strip_prefix("Register C: ").unwrap().parse::<usize>().unwrap();
    splitter.next().unwrap(); // Skip empty line
    let mut instructions = splitter.next().unwrap().strip_prefix("Program: ").unwrap().split(",").map(|c| c.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let mut j = 0;

    let mut matched = 0;

    loop {
        let mut i = 0;
        let mut output = Vec::new();
        register_a = j;
        while i < instructions.len() {
            let instruction = instructions[i];
            let literal_operand = instructions[i + 1];
            let combo_operand = match literal_operand {
                1 => 1,
                2 => 2,
                3 => 3,
                4 => register_a,
                5 => register_b,
                6 => register_c,
                7 => 7,
                8 => 8,
                0 => 9,
                _ => panic!("Invalid operand"),
            };

            match instruction {
                0 => {
                    register_a = register_a / 2_usize.pow(combo_operand as u32);
                }
                1 => {
                    register_b = register_b ^ literal_operand;
                }
                2 => {
                    register_b = combo_operand % 8;
                }
                3 => {
                    if register_a != 0 {
                        i = literal_operand;
                        continue;
                    }
                }
                4 => {
                    register_b = register_b ^ register_c;
                }
                5 => {
                    output.push(combo_operand % 8)
                }
                6 => {
                    register_b = register_a / 2_usize.pow(combo_operand as u32);
                }
                7 => {
                    register_c = register_a / 2_usize.pow(combo_operand as u32);
                }
                _ => {}
            }

            i += 2;
        }

        let common_suffix = instructions.clone().iter().rev().zip(output.clone().iter().rev()) .take_while(|(x, y)| **x == **y) .count();

        if common_suffix > matched {
            matched = common_suffix;
            if common_suffix == instructions.len() {
                return Some(j as u64);
            }
            j *= 8;
            continue;
        }

        j += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("5,7,3,0".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(117440));
    }
}
