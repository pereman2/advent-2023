use std::cmp::max;
use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash, Hasher};

pub fn day_14_1() -> u64 {
    let contents = include_str!("../input_14_1");
    let mut cols: Vec<Vec<u8>> = Vec::new();
    for (i, line) in contents.lines().enumerate() {
        if i == 0 {
            cols = vec![Vec::new(); line.len()];
        }
        for (col, c) in line.as_bytes().iter().enumerate() {
            cols[col].push(*c);
        }
    }
    let mut sum = 0;
    for (i, col) in cols.iter().enumerate() {
        let mut prev: i64 = -1;
        for (j, c) in col.iter().enumerate() {
            let mut to_move = 0_u64;
            match c {
                b'O' => {
                    let j = j as i64;
                    let mut new_pos = max(j - (j - prev - 1), 0);
                    if prev == -1 {
                        new_pos = 0;
                    }
                    to_move = (cols.len() as i64 - new_pos) as u64;
                    prev = new_pos;
                }
                b'.' => {}
                b'#' => {
                    prev = j as i64;
                }
                _ => panic!("Unknown char: {}", c),
            }
            sum += to_move;
        }
    }
    return sum;
}
struct Grid {
    load: u64,
    grid: Vec<Vec<u8>>,
}

impl Hash for Grid {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.load.hash(state);
        for line in &self.grid {
            line.hash(state);
        }
    }
}

impl PartialEq for Grid {
    fn eq(&self, other: &Self) -> bool {
        self.grid == other.grid
    }
}
impl Grid {
    fn new(load: u64, grid: Vec<Vec<u8>>) -> Self {
        Self { load, grid }
    }
}

pub fn day_14_2() -> u64 {
    let contents = include_str!("../input_14_1");
    let mut grid: Vec<Vec<u8>> = contents.lines().map(|s| s.as_bytes().to_vec()).collect();

    let mut sum = 0;
    let n = grid.len();
    let m = grid[0].len();
    let mut grid = Grid::new(0, grid);

    let mut seen = HashMap::new();
    let mut cycle = 0;
    let mut forward = false;
    while cycle < 1000000000 {
        // north
        for j in 0..m {
            let mut prev: i64 = -1;
            for i in 0..n {
                let c = grid.grid[i][j];
                match c {
                    b'O' => {
                        let pos = i as i64;
                        let mut new_pos = max(pos - (pos - prev - 1), 0);
                        if prev == -1 {
                            new_pos = 0;
                        }
                        grid.grid[i][j] = b'.';
                        grid.grid[new_pos as usize][j] = b'O';
                        prev = new_pos;
                    }
                    b'.' => {}
                    b'#' => {
                        prev = i as i64;
                    }
                    _ => panic!("Unknown char: {}", c),
                }
            }
        }
        // west
        for i in 0..n {
            let mut prev: i64 = -1;
            for j in 0..m {
                let c = grid.grid[i][j];
                match c {
                    b'O' => {
                        let pos = j as i64;
                        let mut new_pos = max(pos - (pos - prev - 1), 0);
                        if prev == -1 {
                            new_pos = 0;
                        }
                        grid.grid[i][j] = b'.';
                        grid.grid[i][new_pos as usize] = b'O';
                        prev = new_pos;
                    }
                    b'.' => {}
                    b'#' => {
                        prev = j as i64;
                    }
                    _ => panic!("Unknown char: {}", c),
                }
            }
        }

        // south
        for j in 0..m {
            let mut prev: i64 = m as i64;
            for i in (0..n).rev() {
                let c = grid.grid[i][j];
                match c {
                    b'O' => {
                        let pos = i as i64;
                        let mut new_pos = max(pos + (prev - pos - 1), 0);
                        if prev == -1 {
                            new_pos = 0;
                        }
                        grid.grid[i][j] = b'.';
                        grid.grid[new_pos as usize][j] = b'O';
                        prev = new_pos;
                    }
                    b'.' => {}
                    b'#' => {
                        prev = i as i64;
                    }
                    _ => panic!("Unknown char: {}", c),
                }
            }
        }
        // east
        for i in 0..n {
            let mut prev: i64 = n as i64;
            for j in (0..m).rev() {
                let c = grid.grid[i][j];
                match c {
                    b'O' => {
                        let pos = j as i64;
                        let mut new_pos = max(pos + (prev - pos - 1), 0);
                        if prev == -1 {
                            new_pos = 0;
                        }
                        grid.grid[i][j] = b'.';
                        grid.grid[i][new_pos as usize] = b'O';
                        prev = new_pos;
                    }
                    b'.' => {}
                    b'#' => {
                        prev = j as i64;
                    }
                    _ => panic!("Unknown char: {}", c),
                }
            }
        }

        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        grid.hash(&mut hasher);
        let value = hasher.finish();
        let mut load = 0;
        for (i, line) in grid.grid.iter().enumerate() {
            for (j, c) in line.iter().enumerate() {
                if *c == b'O' {
                    load += (n - i);
                }
            }
        }
        grid.load = load as u64;
        if !forward {
            if let Some(c) = seen.get(&value) {
                cycle += (cycle - c) * ((1000000000 - cycle) / (cycle - c));
                forward = true;
            }
            seen.insert(value, cycle);
        }
        cycle += 1;
    }
    let mut load = 0;
    for (i, line) in grid.grid.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if *c == b'O' {
                load += (n - i);
            }
        }
    }
    return load as u64;
}
