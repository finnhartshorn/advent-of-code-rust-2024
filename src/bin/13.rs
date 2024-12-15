advent_of_code::solution!(13);

#[derive(Debug)]
struct Game {
    x_target: i64,
    y_target: i64,
    a_x_step: i64,
    a_y_step: i64,
    b_x_step: i64,
    b_y_step: i64,
}

impl Game {
    fn solve_machine(&self, offset: i64) -> u64 {
        // let a_presses = (self.b_x_step * (self.y_target + offset) - self.b_y_step * (self.x_target + offset)) /
        //     (self.b_x_step * self.a_y_step - self.b_y_step * self.a_x_step);
        let a_part1 =
            self.b_x_step * (self.y_target + offset) - self.b_y_step * (self.x_target + offset);
        let a_part2 = self.b_x_step * self.a_y_step - self.b_y_step * self.a_x_step;
        if a_part1 % a_part2 != 0 {
            return 0;
        }
        let a_presses = a_part1 / a_part2;
        let b_part1 = (self.y_target + offset) - a_presses * self.a_y_step;
        if b_part1 % self.b_y_step != 0 {
            return 0;
        }
        let b_presses = b_part1 / self.b_y_step;
        if a_presses * self.a_x_step + b_presses * self.b_x_step == self.x_target + offset
            && a_presses * self.a_y_step + b_presses * self.b_y_step == self.y_target + offset
        {
            (a_presses * 3 + b_presses) as u64
        } else {
            0
        }

        // let prize = (self.x_target + offset, self.y_target + offset);
        // let det = self.a_x_step * self.b_y_step - self.a_y_step * self.a_x_step;
        // let a = (prize.1 * self.b_y_step - prize.0 * self.a_x_step) / det;
        // let b = (self.a_x_step * prize.0 - self.a_y_step * prize.1) / det;
        // println!("a: {}, b: {}", a, b);
        // if (self.a_x_step * a + self.a_x_step * b, self.a_y_step * a + self.b_y_step * b) == (prize.0, prize.1) {
        //     (a * 3 + b) as u32
        // } else {
        //     0
        // }
        // 0
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .split("\n\n")
            .map(|section| {
                let mut lines = section.lines();
                let mut split1 = lines
                    .next()
                    .unwrap()
                    .strip_prefix("Button A: ")
                    .unwrap()
                    .split(", ");
                let a_x_step = split1
                    .next()
                    .unwrap()
                    .strip_prefix("X+")
                    .unwrap()
                    .parse()
                    .ok()
                    .unwrap();
                let a_y_step = split1
                    .next()
                    .unwrap()
                    .strip_prefix("Y+")
                    .unwrap()
                    .parse()
                    .ok()
                    .unwrap();
                let mut split2 = lines
                    .next()
                    .unwrap()
                    .strip_prefix("Button B: ")
                    .unwrap()
                    .split(", ");
                let b_x_step = split2
                    .next()
                    .unwrap()
                    .strip_prefix("X+")
                    .unwrap()
                    .parse()
                    .ok()
                    .unwrap();
                let b_y_step = split2
                    .next()
                    .unwrap()
                    .strip_prefix("Y+")
                    .unwrap()
                    .parse()
                    .ok()
                    .unwrap();
                let mut split3 = lines
                    .next()
                    .unwrap()
                    .strip_prefix("Prize: ")
                    .unwrap()
                    .split(", ");
                let x_target = split3
                    .next()
                    .unwrap()
                    .strip_prefix("X=")
                    .unwrap()
                    .parse()
                    .ok()
                    .unwrap();
                let y_target = split3
                    .next()
                    .unwrap()
                    .strip_prefix("Y=")
                    .unwrap()
                    .parse()
                    .ok()
                    .unwrap();

                Game {
                    x_target,
                    y_target,
                    a_x_step,
                    a_y_step,
                    b_x_step,
                    b_y_step,
                }
                .solve_machine(0)
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let offset = 10000000000000;

    Some(
        input
            .split("\n\n")
            .map(|section| {
                let mut lines = section.lines();
                let mut split1 = lines
                    .next()
                    .unwrap()
                    .strip_prefix("Button A: ")
                    .unwrap()
                    .split(", ");
                let a_x_step = split1
                    .next()
                    .unwrap()
                    .strip_prefix("X+")
                    .unwrap()
                    .parse()
                    .ok()
                    .unwrap();
                let a_y_step = split1
                    .next()
                    .unwrap()
                    .strip_prefix("Y+")
                    .unwrap()
                    .parse()
                    .ok()
                    .unwrap();
                let mut split2 = lines
                    .next()
                    .unwrap()
                    .strip_prefix("Button B: ")
                    .unwrap()
                    .split(", ");
                let b_x_step = split2
                    .next()
                    .unwrap()
                    .strip_prefix("X+")
                    .unwrap()
                    .parse()
                    .ok()
                    .unwrap();
                let b_y_step = split2
                    .next()
                    .unwrap()
                    .strip_prefix("Y+")
                    .unwrap()
                    .parse()
                    .ok()
                    .unwrap();
                let mut split3 = lines
                    .next()
                    .unwrap()
                    .strip_prefix("Prize: ")
                    .unwrap()
                    .split(", ");
                let x_target = split3
                    .next()
                    .unwrap()
                    .strip_prefix("X=")
                    .unwrap()
                    .parse()
                    .ok()
                    .unwrap();
                let y_target = split3
                    .next()
                    .unwrap()
                    .strip_prefix("Y=")
                    .unwrap()
                    .parse()
                    .ok()
                    .unwrap();

                Game {
                    x_target,
                    y_target,
                    a_x_step,
                    a_y_step,
                    b_x_step,
                    b_y_step,
                }
                .solve_machine(offset)
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
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
