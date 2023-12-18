use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::collections::HashSet;

#[derive(Debug)]
struct State {
    i: usize,
    j: usize,
    dx: usize,
    steps: i64,
    value: u64,
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for State {
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    } 
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.value.cmp(&self.value)
    }
}


pub fn day_17_1() -> u64 {
    let contents = include_str!("../input_17_1");
    let grid = contents
        .lines()
        .map(|s| s.as_bytes().to_vec())
        .collect::<Vec<Vec<u8>>>();

    let n = grid.len();
    let m = grid[0].len();
    let mut DX = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut DXH = HashMap::new();
    DXH.insert((0, 1), 0);
    DXH.insert((0, -1), 1);
    DXH.insert((1, 0), 2);
    DXH.insert((-1, 0), 3);

    // dp[i][j][dx][steps] = number of way to get to (i, j) if previous dx was dx and we have gone in that direction
    // for steps
    // solve(i, j, dx, steps))
    let tod = |v| (v - b'0') as u64;
    let mut seen = HashSet::new();

    let mut stack = BinaryHeap::new();
    stack.push(State{i: 0, j: 0, dx: DXH[&(0, 1)], steps: 1, value: 0});
    stack.push(State{i: 0, j: 0, dx: DXH[&(1, 0)], steps: 1, value: 0});
    let mut best = u64::MAX;
    while stack.len() > 0 {
        let state = stack.pop().unwrap();
        let i = state.i;
        let j = state.j;
        let prev_dx = state.dx;
        let steps = state.steps;
        let previous_value = state.value;
        if i == n-1 && j == m-1 {
            best = previous_value;
            break;
        }
        if seen.contains(&(i, j, prev_dx, steps)) {
            continue;
        }
        seen.insert((i, j, prev_dx, steps));
        let new_i = i as i64 + DX[prev_dx].0;
        let new_j = j as i64 + DX[prev_dx].1;
        let inside = new_i >= 0 && new_i < n as i64 && new_j >= 0 && new_j < m as i64;
        if steps < 3 && inside {
            let new_i = new_i as usize;
            let new_j = new_j as usize;
            let grid_value: u64 = tod(grid[new_i][new_j]);
            let new_value: u64 = previous_value + grid_value;
            stack.push(State{i: new_i, j: new_j, dx: prev_dx, steps: steps+1, value: new_value});
        }
        let neighbors = match DX[prev_dx] {
            (0, 1) => vec![(1, 0), (-1, 0)],
            (0, -1) => vec![(1, 0), (-1, 0)],
            (1, 0) => vec![(0, 1), (0, -1)],
            (-1, 0) => vec![(0, 1), (0, -1)],
            _ => panic!("Unknown dx: {:?}", DX[prev_dx]),
        };
        for dx in neighbors {
            let new_i = i as i64 + dx.0;
            let new_j = j as i64 + dx.1;
            let inside = new_i >= 0 && new_i < n as i64 && new_j >= 0 && new_j < m as i64;
            if !inside {
                continue;
            }   
            let new_i = new_i as usize;
            let new_j = new_j as usize;
            let grid_value: u64 = tod(grid[new_i][new_j]);
            let new_value: u64 = previous_value + grid_value;
            stack.push(State{i: new_i, j: new_j, dx: DXH[&dx], steps: 1, value: new_value});
        }
    }
    return best;
}

pub fn day_17_2() -> u64 {
    let contents = include_str!("../input_17_1");
    let grid = contents
        .lines()
        .map(|s| s.as_bytes().to_vec())
        .collect::<Vec<Vec<u8>>>();

    let n = grid.len();
    let m = grid[0].len();
    let mut DX = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut DXH = HashMap::new();
    DXH.insert((0, 1), 0);
    DXH.insert((0, -1), 1);
    DXH.insert((1, 0), 2);
    DXH.insert((-1, 0), 3);

    // dp[i][j][dx][steps] = number of way to get to (i, j) if previous dx was dx and we have gone in that direction
    // for steps
    // solve(i, j, dx, steps))
    let tod = |v| (v - b'0') as u64;
    let mut seen = HashSet::new();

    let mut stack = BinaryHeap::new();
    stack.push(State{i: 0, j: 0, dx: DXH[&(0, 1)], steps: 1, value: 0});
    stack.push(State{i: 0, j: 0, dx: DXH[&(1, 0)], steps: 1, value: 0});
    let mut best = u64::MAX;
    while stack.len() > 0 {
        let state = stack.pop().unwrap();
        let i = state.i;
        let j = state.j;
        let prev_dx = state.dx;
        let steps = state.steps;
        let previous_value = state.value;
        if i == n-1 && j == m-1 {
            best = previous_value;
            break;
        }
        if seen.contains(&(i, j, prev_dx, steps)) {
            continue;
        }
        seen.insert((i, j, prev_dx, steps));
        let new_i = i as i64 + DX[prev_dx].0;
        let new_j = j as i64 + DX[prev_dx].1;
        let inside = new_i >= 0 && new_i < n as i64 && new_j >= 0 && new_j < m as i64;
        if steps < 10 && inside {
            let new_i = new_i as usize;
            let new_j = new_j as usize;
            let grid_value: u64 = tod(grid[new_i][new_j]);
            let new_value: u64 = previous_value + grid_value;
            stack.push(State{i: new_i, j: new_j, dx: prev_dx, steps: steps+1, value: new_value});
        }
        let neighbors = match DX[prev_dx] {
            (0, 1) => vec![(1, 0), (-1, 0)],
            (0, -1) => vec![(1, 0), (-1, 0)],
            (1, 0) => vec![(0, 1), (0, -1)],
            (-1, 0) => vec![(0, 1), (0, -1)],
            _ => panic!("Unknown dx: {:?}", DX[prev_dx]),
        };
        if steps >= 4 {
            for dx in neighbors {
                let new_i = i as i64 + dx.0;
                let new_j = j as i64 + dx.1;
                let inside = new_i >= 0 && new_i < n as i64 && new_j >= 0 && new_j < m as i64;
                if !inside {
                    continue;
                }   
                let new_i = new_i as usize;
                let new_j = new_j as usize;
                let grid_value: u64 = tod(grid[new_i][new_j]);
                let new_value: u64 = previous_value + grid_value;
                stack.push(State{i: new_i, j: new_j, dx: DXH[&dx], steps: 1, value: new_value});
            }
        }
    }
    return best;
}
