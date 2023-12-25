use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::hash::{Hash, Hasher};

#[derive(PartialOrd, Clone, Eq, Debug)]
struct State {
    pos: (usize, usize),
    distance: usize,
    steps: usize,
    real_pos: (i64, i64),
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos && self.distance == other.distance && self.steps == other.steps
    }
}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.pos.hash(state);
        // (self.distance + self.steps).hash(state);
        self.distance.hash(state);
        self.steps.hash(state);
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance.cmp(&other.distance)
    }
}

pub fn day_21_1() -> u64 {
    return solve(64);
}

pub fn solve(STEPS: usize) -> u64 {
    let contents = include_str!("../input_21_1");
    let mut grid = contents
        .lines()
        .map(|s| s.as_bytes().to_vec())
        .collect::<Vec<Vec<u8>>>();
    let mut start = (0, 1);
    'outter: for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == b'S' {
                start = (i, j);
                grid[i][j] = b'.';
                break 'outter;
            }
        }
    }
    let mut seen = HashSet::new();
    let mut seen_dp = HashSet::new();
    let distance = |a: (usize, usize), b: (usize, usize)| {
        ((a.0 as i64 - b.0 as i64).abs() + (a.1 as i64 - b.1 as i64).abs()) as usize
    };
    let mut count = 0;
    let mut queue = VecDeque::new();
    let start_state = State {
        pos: start,
        distance: 0,
        steps: 0,
        real_pos: (start.0 as i64, start.1 as i64)
    };
    queue.push_front(start_state);
    while queue.len() > 0 {
        let state = queue.pop_back().unwrap();
        if seen_dp.contains(&state) {
            continue;
        }
        seen_dp.insert(state.clone());
        if state.steps > STEPS {
            continue;
        }
        if state.steps == STEPS {
            if (STEPS - state.steps) % 2 == 0 && !seen.contains(&state.real_pos) {
                seen.insert(state.real_pos);
                count += 1;
            }
            continue;
        }

        for dx in vec![(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let new_real_pos = (state.real_pos.0 + dx.0, state.real_pos.1 + dx.1);
            let mut new_pos = (state.pos.0 as i64 + dx.0, state.pos.1 as i64 + dx.1);
            if new_pos.0 == -1 {
                new_pos.0 = grid.len() as i64 - 1;
            }
            if new_pos.1 == -1 {
                new_pos.1 = grid[0].len() as i64 - 1;
            }
            if new_pos.0 < 0
                || new_pos.1 < 0
                || new_pos.0 >= grid.len() as i64
                || new_pos.1 >= grid[0].len() as i64
            {
                continue;
            }
            let new_pos = (new_pos.0 as usize, new_pos.1 as usize);
            if grid[new_pos.0][new_pos.1] == b'#' {
                continue;
            }
            let new_distance = distance(new_pos, start);
            let new_state = State {
                pos: new_pos,
                distance: new_distance,
                steps: state.steps + 1,
                real_pos: new_real_pos
            };
            queue.push_front(new_state);
        }
    }

    return count;
}


#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Position(i32, i32);

#[derive(Clone, Debug)]
struct Garden {
    rocks: HashSet<Position>,
    start: Position,
    extents: Position,
}


fn build_garden(contents: &str) -> Garden {
    let mut rocks: HashSet<Position> = HashSet::new();
    let mut start: Position = Position(0, 0);
    for (row, line) in contents.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            match ch {
                '#' => {
                    rocks.insert(Position(row as i32, col as i32));
                }
                _ => {}
            };
        }
    }
    let num_rows = contents.lines().count() as i32;
    let num_cols = contents.lines().next().unwrap().chars().count() as i32;
    let mut expanded_rocks: HashSet<Position> = HashSet::new();
    for rock in rocks.iter() {
        for row_mul in 0..5 {
            for col_mul in 0..5 {
                let expanded_rock = Position(
                    rock.0 + (num_rows * row_mul as i32),
                    rock.1 + (num_cols * col_mul as i32),
                );
                expanded_rocks.insert(expanded_rock);
            }
        }
    }
    Garden {
        rocks: expanded_rocks,
        start: Position(num_rows * 5 / 2, num_cols * 5 / 2),
        extents: Position(num_rows * 5, num_cols * 5),
    }
}

fn in_bounds(pos: Position, extents: Position) -> bool {
    pos.0 >= 0 && pos.0 <= extents.0 && pos.1 >= 0 && pos.1 < extents.1
}

fn walk(garden: &Garden, steps: u32) -> u32 {
    let mut visited: HashSet<Position> = HashSet::new();
    visited.insert(garden.start);
    for i in 0..steps {
        let mut new_visited = HashSet::new();
        for pos in visited.iter().clone() {
            let neighbors = [
                Position(pos.0 - 1, pos.1),
                Position(pos.0 + 1, pos.1),
                Position(pos.0, pos.1 - 1),
                Position(pos.0, pos.1 + 1),
            ]
            .into_iter()
            .filter(|p| in_bounds(*p, garden.extents) && !garden.rocks.contains(p));
            for neighbor in neighbors {
                new_visited.insert(neighbor);
            }
        }
        visited = new_visited;
    }
    visited.len() as u32
}
pub fn day_21_2() -> u64 {
    let contents = include_str!("../input_21_1");
    let garden = build_garden(contents);
    let b0: i64 = walk(&garden, 65) as i64;
    let b1: i64 = walk(&garden, 65 + 131) as i64;
    let b2: i64 = walk(&garden, 65 + 2 * 131) as i64;
    let n: i64 = 202300;
    // the following formula comes from inv(A) * B = X,
    // where A is Vandemonde matrix:
    // [ 0 0 1 ]
    // [ 1 1 1 ]
    // [ 4 2 1 ]
    // and B is a column vector from the above values b0, b1, b2
    // credit to: https://gist.github.com/dllu/0ca7bfbd10a199f69bcec92f067ec94c
    // below uses Cramer's Rule to solve for x0, x1, x2
    let det_a: f64 = -2.0;
    let det_a0: f64 = -b0 as f64 + 2.0 * b1 as f64 - b2 as f64;
    let det_a1: f64 = 3.0 * b0 as f64 - 4.0 * b1 as f64 + b2 as f64;
    let det_a2: f64 = -2.0 * b0 as f64;
    let x0: i64 = (det_a0 / det_a) as i64;
    let x1: i64 = (det_a1 / det_a) as i64;
    let x2: i64 = (det_a2 / det_a) as i64;
    let res = x0 * n * n + x1 * n + x2;
    return res as u64;
}