
use std::collections::{HashSet, VecDeque};

fn score_trailhead(grid: &[Vec<u8>], start: (usize, usize)) -> usize {
    let height = grid.len();
    let width = grid[0].len();
    let mut reachable_nines = HashSet::new();
    //pre allocate
    let mut visited = HashSet::with_capacity(height * width);
    let mut queue = VecDeque::with_capacity(height * width);

    queue.push_back(start);
    visited.insert(start);

    //pas de 'path'
    while let Some(pos) = queue.pop_front() {
        let (i, j) = pos;
        let current_height = grid[i][j];

        if current_height == 9 {
            reachable_nines.insert(pos);
            continue;
        }

        // Pre-calculate
        let next_height = current_height + 1;

        let neighbors = [
            (i.wrapping_sub(1), j),
            (i + 1, j),
            (i, j.wrapping_sub(1)),
            (i, j + 1),
        ];

        for &next_pos @ (ni, nj) in &neighbors {
            if ni < height
                && nj < width
                && grid[ni][nj] == next_height
                && visited.insert(next_pos)
            {
                queue.push_back(next_pos);
            }
        }
    }

    reachable_nines.len()
}

fn _p1(_input: &str) -> usize {
    let grid: Vec<Vec<u8>> = _input
        .lines()
        .map(|line| line.bytes().map(|b| b - b'0').collect())
        .collect();

    let height = grid.len();
    let width = grid[0].len();

    (0..height)
        .flat_map(|i| (0..width).map(move |j| (i, j)))
        .filter(|&(i, j)| grid[i][j] == 0)
        .map(|pos| score_trailhead(&grid, pos))
        .sum()
}

fn count_trails(grid: &[Vec<u8>], start: (usize, usize)) -> i64 {
    let height = grid.len();
    let width = grid[0].len();
    let mut count = 0;
    let mut visited = vec![start];

    fn dfs(
        grid: &[Vec<u8>],
        pos: (usize, usize),
        visited: &mut Vec<(usize, usize)>,
        count: &mut i64,
        height: usize,
        width: usize,
    ) {
        let (i, j) = pos;
        if grid[i][j] == 9 {
            *count += 1;
            return;
        }

        let current_height = grid[i][j];
        let neighbors = [
            (i.wrapping_sub(1), j),
            (i + 1, j),
            (i, j.wrapping_sub(1)),
            (i, j + 1),
        ];

        for next_pos @ (ni, nj) in neighbors {
            if ni < height && nj < width
                && grid[ni][nj] == current_height + 1
                && !visited.contains(&next_pos)
            {
                visited.push(next_pos);
                dfs(grid, next_pos, visited, count, height, width);
                visited.pop();
            }
        }
    }

    dfs(&grid, start, &mut visited, &mut count, height, width);
    count
}

fn _p2(_input: &str) -> i64 {
    let grid: Vec<Vec<u8>> = _input
        .lines()
        .map(|line| line.bytes().map(|b| b - b'0').collect())
        .collect();

    let height = grid.len();
    let width = grid[0].len();

    (0..height)
        .flat_map(|i| (0..width).map(move |j| (i, j)))
        .filter(|&(i, j)| grid[i][j] == 0)
        .map(|pos| count_trails(&grid, pos))
        .sum()
}

pub fn p1() -> usize {
    _p1(include_str!("day10"))
}

pub fn p2() -> i64 {
    _p2(include_str!("day10"))
}

#[cfg(test)]
mod tests {
    use crate::day10::*;
    #[test]
    fn test_p1() {
        assert_eq!(5248, _p1(include_str!("day10_test")))
    }

    #[test]
    fn test_p2()
    {
        assert_eq!(328790210468594, _p2(include_str!("day10_test")))
    }

    /*#[test]
    fn real_p1(){
        assert_eq!(598484546, p1());
    }

    #[test]
    fn real_p2(){
        assert_eq!(64845668, p2());
    }*/
}
