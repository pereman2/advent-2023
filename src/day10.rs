use std::{cmp::max, collections::HashMap, ops::Index};

#[derive(Debug)]
struct State {
    cur: (i32, i32),
    prev: (i32, i32),
    count: usize,
}

fn update(
    start: (i32, i32),
    grid: &Vec<&[u8]>,
    DX: &Vec<(i32, i32)>,
    VALID_DX: &HashMap<(i32, i32), Vec<u8>>,
) -> u64 {
    let n = grid.len();
    let m = grid[0].len();
    let mut done = false;
    let mut res = (0, 0);
    let mut resdx = (0, 0);
    let state = State {
        cur: start,
        prev: (0, 0),
        count: 0,
    };
    let mut stack: Vec<State> = Vec::new();
    stack.push(state);
    let mut seen: HashMap<(i32, i32), bool> = HashMap::new();
    while true {
        // dbg!(&stack);
        let cur_state = stack.pop().unwrap();
        // if cur_state.count > 10 {
        //     break;
        // }
        if cur_state.cur == start && cur_state.count > 0 {
            return (cur_state.count / 2) as u64;
        }
        if seen.contains_key(&cur_state.cur) {
            continue;
        }
        seen.insert(cur_state.cur.clone(), true);
        let mut valids = 0;
        for dx in DX {
            if cur_state.prev.0 == dx.0 && cur_state.prev.1 == dx.1 {
                // don't go where you came from
                continue;
            }
            // dbg!(dx);
            let valid = VALID_DX.get(&dx).unwrap();
            for valid_value in valid {
                // dbg!(&v);
                // dbg!(valid_value.to_string());
                // dbg!(v.0 + dx.0);
                // dbg!(v.1 + dx.1);
                let new_i = cur_state.cur.0 + dx.0;
                let new_j = cur_state.cur.1 + dx.1;
                if new_i < 0 || new_i >= n as i32 {
                    continue;
                }
                if new_j < 0 || new_j >= m as i32 {
                    continue;
                }
                if grid[new_i as usize][new_j as usize] == *valid_value {
                    valids += 1;
                    // found
                    stack.push(State {
                        cur: (new_i as i32, new_j as i32),
                        prev: ((-1) * dx.0, (-1) * dx.1),
                        count: cur_state.count + 1,
                    });
                }
            }
        }
        dbg!(valids);
    }
    return 0;
}

fn update2(
    start: (i32, i32),
    grid: &Vec<&[u8]>,
    DX: &Vec<(i32, i32)>,
    VALID_DX: &HashMap<(i32, i32), Vec<u8>>,
) -> u64 {
    let n = grid.len();
    let m = grid[0].len();
    let mut done = false;
    let mut res = (0, 0);
    let mut resdx = (0, 0);
    let state = State {
        cur: start,
        prev: (0, 0),
        count: 0,
    };
    let mut stack: Vec<State> = Vec::new();
    stack.push(state);
    let mut seen: HashMap<(i32, i32), bool> = HashMap::new();
    let mut path: Vec<(i32, i32)> = Vec::new();
    let DX_CROSS: HashMap<u8, Vec<(i32, i32)>> = [
        (b'J', [(0, -1), (-1, 0)].to_vec()),
        (b'L', [(0, 1), (-1, 0)].to_vec()),
        (b'7', [(1, 0), (0, -1)].to_vec()),
        (b'F', [(1, 0), (0, 1)].to_vec()),
        (b'|', [(1, 0), (-1, 0)].to_vec()),
        (b'-', [(0, -1), (0, 1)].to_vec()),
        (b'.', DX.clone()),
        (b'S', DX.clone()),
    ]
    .iter()
    .map(|v| v.clone())
    .collect();
    while true {
        // dbg!(&stack);
        let cur_state = stack.pop().unwrap();
        path.push(cur_state.cur);
        // if cur_state.count > 10 {
        //     break;
        // }
        let cur_value = grid[cur_state.cur.0 as usize][cur_state.cur.1 as usize];
        if cur_state.cur == start && cur_state.count > 0 {
            break;
        }
        if seen.contains_key(&cur_state.cur) {
            path.pop();
            continue;
        }
        seen.insert(cur_state.cur.clone(), true);
        let mut valids = 0;
        dbg!(cur_value as char);
        for dx in DX_CROSS.get(&cur_value).unwrap() {
            if cur_state.prev.0 == dx.0 && cur_state.prev.1 == dx.1 {
                // don't go where you came from
                continue;
            }
            // dbg!(dx);
            let valid = VALID_DX.get(&dx).unwrap();
            for valid_value in valid {
                // dbg!(&v);
                // dbg!(valid_value.to_string());
                // dbg!(v.0 + dx.0);
                // dbg!(v.1 + dx.1);
                let new_i = cur_state.cur.0 + dx.0;
                let new_j = cur_state.cur.1 + dx.1;
                if new_i < 0 || new_i >= n as i32 {
                    continue;
                }
                if new_j < 0 || new_j >= m as i32 {
                    continue;
                }
                if grid[new_i as usize][new_j as usize] == *valid_value {
                    valids += 1;
                    // found
                    stack.push(State {
                        cur: (new_i as i32, new_j as i32),
                        prev: ((-1) * dx.0, (-1) * dx.1),
                        count: cur_state.count + 1,
                    });
                }
            }
        }
        if valids == 0 {
            path.pop();
        }
        dbg!(valids);
    }

    let in_path = path
        .iter()
        .map(|v| (v.clone(), true))
        .collect::<HashMap<(i32, i32), bool>>();

    // check path is a loop
    let start = path[0];
    let mut cur = path[1];
    let mut i = 1;
    while cur != start {
        let next = path[i + 1];
        let mut found = false;
        dbg!(cur);
        for dx in DX {
            if cur.0 + dx.0 == next.0 && cur.1 + dx.1 == next.1 {
                i += 1;
                found = true;
                cur = next;
                break;
            }
        }
        dbg!(next);
        assert!(found);
    }
    dbg!(i);
    dbg!(path.len());

    let mut res: Vec<String> = Vec::new();
    for i in 0..n {
        res.push("".to_string());
        for j in 0..m {
            if in_path.contains_key(&(i as i32, j as i32)) {
                res[i].push(grid[i][j] as char);
            } else if (grid[i][j] == b'.') {
                res[i].push('.');
            } else {
                res[i].push('.');
            }
        }
    }
    for line in &res {
        println!("{}", line);
    }
    let mut count = 0;
    let mut grid_repr = res
        .iter()
        .map(|v| v.as_bytes().to_vec())
        .collect::<Vec<Vec<u8>>>();
    for (i, line) in grid.iter().enumerate() {
        let mut within = false;
        let mut up: Option<bool> = None;
        for (j, v) in line.iter().enumerate() {
            let mut a = *v;
            if *v != b'.' && !in_path.contains_key(&(i as i32, j as i32)) {
                a = b'.';
            }
            if *v == b'S' {
                a = b'J';
            }
            dbg!((i,j));
            grid_repr[i][j] = b'.';
            match a {
                b'J' => {
                    let ch = up.is_some() && up.unwrap();
                    let ch = if ch { b'J' } else { b'L' };
                    if b'J' != ch {
                        within = !within;
                    }
                    up = None;
                },
                b'L' => {
                    up = Some(true);
                },
                b'7' => {
                    let ch = up.is_some() && up.unwrap();
                    let ch = if ch { b'J' } else { b'L' };
                    if b'7' != ch {
                        within = !within;
                    }
                    up = None;
                },
                b'F' => {
                    up = Some(false);
                },
                b'|' => {
                    assert!(up.is_none());
                    within = !within;
                },
                b'-' => {
                    assert!(!up.is_none());
                    // crossings += 1;
                },
                b'S' => {
                    within = !within;
                },
                _ => {}
            }
            if within {
                grid_repr[i][j] = b'X';
                count += 1;
            }
        }
    }
    // print grid repr
    for line in grid_repr.iter() {
        println!("{}", String::from_utf8(line.clone()).unwrap());
    }
    // dbg!(path);
    return count;
}

pub fn day_10_1() -> u64 {
    let contents = include_str!("../input_10_1");
    let grid: Vec<&[u8]> = contents.lines().map(|v| v.as_bytes()).collect();
    let mut start: (i32, i32) = (0, 0);
    let mut found = false;
    for (i, line) in contents.lines().enumerate() {
        let as_b = line.as_bytes();
        if !found {
            for (j, c) in as_b.iter().enumerate() {
                if *c == b'S' {
                    start = (i as i32, j as i32);
                    found = true;
                    break;
                }
            }
        }
    }
    let DX: Vec<(i32, i32)> = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    let VALID_DX: HashMap<(i32, i32), Vec<u8>> = [
        ((0, 1), vec![b'-', b'7', b'J', b'S']),
        ((0, -1), vec![b'-', b'F', b'L', b'S']),
        ((1, 0), vec![b'|', b'L', b'J', b'S']),
        ((-1, 0), vec![b'|', b'F', b'7', b'S']),
    ]
    .iter()
    .cloned()
    .collect();
    return update(start, &grid, &DX, &VALID_DX);
}

pub fn day_10_2() -> u64 {
    let contents = include_str!("../input_10_1");
    let grid: Vec<&[u8]> = contents.lines().map(|v| v.as_bytes()).collect();
    let mut start: (i32, i32) = (0, 0);
    let mut found = false;
    for (i, line) in contents.lines().enumerate() {
        let as_b = line.as_bytes();
        if !found {
            for (j, c) in as_b.iter().enumerate() {
                if *c == b'S' {
                    start = (i as i32, j as i32);
                    found = true;
                    break;
                }
            }
        }
    }
    let DX: Vec<(i32, i32)> = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    let VALID_DX: HashMap<(i32, i32), Vec<u8>> = [
        ((0, 1), vec![b'-', b'7', b'J', b'S']),
        ((0, -1), vec![b'-', b'F', b'L', b'S']),
        ((1, 0), vec![b'|', b'L', b'J', b'S']),
        ((-1, 0), vec![b'|', b'F', b'7', b'S']),
    ]
    .iter()
    .cloned()
    .collect();
    return update2(start, &grid, &DX, &VALID_DX);
}
