pub mod template;

// Use this file to add helper functions and additional modules.
pub fn get_cardinal_directions(
    coord: (usize, usize),
    max_x: usize,
    max_y: usize,
) -> Vec<(usize, usize)> {
    let mut results = vec![];
    if coord.0 < max_x - 1 {
        results.push((coord.0 + 1, coord.1));
    }
    if coord.1 < max_y - 1 {
        results.push((coord.0, coord.1 + 1));
    }
    if coord.0 != 0 {
        results.push((coord.0 - 1, coord.1));
    }
    if coord.1 != 0 {
        results.push((coord.0, coord.1 - 1));
    }
    results
}

pub fn get_all_directions(
    coord: (usize, usize),
    max_x: usize,
    max_y: usize,
) -> Vec<(usize, usize)> {
    let mut results = vec![];
    if coord.0 < max_x - 1 {
        results.push((coord.0 + 1, coord.1));
    }
    if coord.1 < max_y - 1 {
        results.push((coord.0, coord.1 + 1));
    }
    if coord.0 < max_x - 1 && coord.1 < max_y - 1 {
        results.push((coord.0 + 1, coord.1 + 1));
    }
    if coord.0 != 0 {
        results.push((coord.0 - 1, coord.1));
    }
    if coord.1 != 0 {
        results.push((coord.0, coord.1 - 1));
    }
    if coord.0 != 0 && coord.1 != 0 {
        results.push((coord.0 - 1, coord.1 - 1));
    }
    if coord.0 != 0 && coord.1 < max_y - 1 {
        results.push((coord.0 - 1, coord.1 + 1));
    }
    if coord.0 < max_x - 1 && coord.1 != 0 {
        results.push((coord.0 + 1, coord.1 - 1));
    }
    results
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

