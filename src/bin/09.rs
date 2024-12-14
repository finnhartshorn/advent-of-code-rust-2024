advent_of_code::solution!(9);

enum State {
    File,
    Free,
}

impl State {
    fn next(&self) -> State {
        match self {
            State::File => State::Free,
            State::Free => State::File,
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let disk_map: Vec<u32> = input
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    let mut state = State::File;
    let mut start_i = 0;
    let mut end_i = disk_map.len() + 1;
    let mut position_num = 0;
    let mut start_file_id = 0;
    let mut end_file_id = ((disk_map.len() / 2) + 1) as u64;
    let mut end_file_remainder = 0;
    let mut sum = 0;
    while start_i < end_i {
        match state {
            State::File => {
                let file = disk_map[start_i];
                for _ in 0..file {
                    sum += start_file_id * position_num;
                    position_num += 1;
                }
                start_file_id += 1;
                state = state.next();
            }
            State::Free => {
                for _ in 0..disk_map[start_i] {
                    if end_file_remainder == 0 {
                        end_i -= 2;
                        end_file_remainder = disk_map[end_i];
                        end_file_id -= 1;
                        if end_i < start_i {
                            return Some(sum);
                        }
                    }
                    sum += end_file_id * position_num;
                    position_num += 1;
                    end_file_remainder -= 1;
                }
                state = state.next();
            }
        }
        start_i += 1;
    }
    for _ in 0..end_file_remainder {
        sum += end_file_id * position_num;
        position_num += 1
    }
    Some(sum)
}

#[derive(Debug)]
struct Section {
    position: u32,
    size: u32,
    id: u32,
}

impl Section {
    fn checksum(&self) -> u64 {
        (0..self.size)
            .map(|n| (self.position + n) as u64 * self.id as u64)
            .sum()
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut spaces: Vec<Section> = Vec::new();
    let mut files: Vec<Section> = Vec::new();
    let mut state = State::File;
    let mut id = 0;
    let mut i = 0;
    input.lines().next().unwrap().chars().for_each(|c| {
        let size = c.to_digit(10).unwrap();
        if size == 0 {
            state = state.next();
            return;
        }
        match state {
            State::File => {
                files.push(Section {
                    position: i,
                    id,
                    size,
                });
                id += 1;
            }
            State::Free => {
                spaces.push(Section {
                    position: i,
                    id,
                    size,
                });
            }
        }
        state = state.next();
        i += size;
    });
    let mut sum: u64 = 0;
    'outer: while let Some(mut file) = files.pop() {
        let spaces_len = spaces.len();
        for i in 0..spaces_len {
            if spaces[i].position > file.position {
                break;
            }
            if spaces[i].size >= file.size {
                spaces[i].size -= file.size;
                file.position = spaces[i].position;
                spaces[i].position += file.size;
                sum += file.checksum();
                if spaces[i].size == 0 {
                    spaces.remove(i);
                }
                continue 'outer;
            }
        }
        sum += file.checksum();
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
