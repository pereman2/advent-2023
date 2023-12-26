use std::{cmp::max, collections::HashSet};

fn solve2(
    grid: &Vec<Vec<u8>>,
    seen: &mut HashSet<(i64, i64)>,
    current: (i64, i64),
    steps: u64,
    res: &mut u64,
) {
    if current.0 < 0
        || current.0 >= grid.len() as i64
        || current.1 < 0
        || current.1 >= grid[0].len() as i64
    {
        return;
    }
    if seen.contains(&current) {
        return;
    }
    if grid[current.0 as usize][current.1 as usize] == b'#' {
        return;
    }
    seen.insert(current);

    if current.0 == (grid.len() - 1) as i64 && grid[current.0 as usize][current.1 as usize] == b'.'
    {
        *res = max(steps, *res);
        seen.remove(&current);
        return;
    }

    let DX = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    for dx in DX {
        solve2(
            grid,
            seen,
            (current.0 + dx.0, current.1 + dx.1),
            steps + 1,
            res,
        );
    }
    seen.remove(&current);
}

fn solve(
    grid: &Vec<Vec<u8>>,
    seen: &mut HashSet<(i64, i64)>,
    current: (i64, i64),
    steps: u64,
    res: &mut u64,
) {
    if current.0 < 0
        || current.0 >= grid.len() as i64
        || current.1 < 0
        || current.1 >= grid[0].len() as i64
    {
        return;
    }
    if seen.contains(&current) {
        return;
    }
    if grid[current.0 as usize][current.1 as usize] == b'#' {
        return;
    }
    seen.insert(current);

    if current.0 == (grid.len() - 1) as i64 && grid[current.0 as usize][current.1 as usize] == b'.'
    {
        *res = max(steps, *res);
        seen.remove(&current);
        return;
    }

    let mut slope = true;
    match grid[current.0 as usize][current.1 as usize] {
        b'>' => {
            solve(grid, seen, (current.0, current.1 + 1), steps + 1, res);
        }
        b'<' => {
            solve(grid, seen, (current.0, current.1 - 1), steps + 1, res);
        }
        b'v' => {
            solve(grid, seen, (current.0 + 1, current.1), steps + 1, res);
        }
        b'^' => {
            solve(grid, seen, (current.0 - 1, current.1), steps + 1, res);
        }
        _ => {
            slope = false;
        }
    }
    if !slope {
        let DX = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        for dx in DX {
            solve(
                grid,
                seen,
                (current.0 + dx.0, current.1 + dx.1),
                steps + 1,
                res,
            );
        }
    }
    seen.remove(&current);
}

pub fn day_23_1() -> u64 {
    let contents = include_str!("../input_23_1");
    let grid: Vec<Vec<u8>> = contents.lines().map(|l| l.as_bytes().to_vec()).collect();
    let mut start = (0, 0);
    for (i, c) in grid[0].iter().enumerate() {
        if *c == b'.' {
            start = (0 as i64, i as i64);
        }
    }

    let mut res = 0;
    solve(&grid, &mut HashSet::new(), start, 0, &mut res);
    return res;
}

pub fn day_23_2() -> u64 {
    let contents = include_str!("../input_23_1");
    let grid: Vec<Vec<u8>> = contents.lines().map(|l| l.as_bytes().to_vec()).collect();
    let mut start = (0, 0);
    for (i, c) in grid[0].iter().enumerate() {
        if *c == b'.' {
            start = (0 as i64, i as i64);
        }
    }

    let mut dp = vec![vec![0; grid[0].len()]; grid.len()];
    dp[0][start.1 as usize] = 1;

    let mut res = 0;
    solve2(&grid, &mut HashSet::new(), start, 0, &mut res);
    return res;
}
