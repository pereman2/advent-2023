use std::collections::HashMap;

fn update(
    v: &mut (i32, i32),
    prev: &mut (i32, i32),
    grid: &Vec<&[u8]>,
    DX: &Vec<(i32, i32)>,
    VALID_DX: &HashMap<(i32, i32), Vec<u8>>,
) {
    'outter: for dx in DX {
        if prev.0 == dx.0 && prev.1 == prev.1 {
            // don't go where you came from
            continue;
        }
        let valid = VALID_DX.get(&dx).unwrap();
        for valid_value in valid {
            dbg!(valid_value.to_string());
            dbg!(v.0 + dx.0);
            dbg!(v.1 + dx.1);
            dbg!(grid[(v.0 + dx.0) as usize][(v.1 + dx.1) as usize]);
            if grid[(v.0 + dx.0) as usize][(v.1 + dx.1) as usize] == *valid_value {
                // found
                *prev = dx.clone();
                *v = (v.0 + dx.0, v.1 + dx.1);
                break 'outter;
            }
        }
    }
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
    let mut count_grid = vec![vec![0; grid.first().unwrap().len()]; grid.len()];
    // our input starts with bottom and right, no need to be general here.
    let mut aprev: (i32, i32) = (0, 1);
    let mut a: (i32, i32) = (start.0, start.1 + 1);
    let mut bprev: (i32, i32) = (-1, 0);
    let mut b: (i32, i32) = (start.0 - 1, start.1);
    let mut count = 1;
    let DX: Vec<(i32, i32)> = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    let VALID_DX: HashMap<(i32, i32), Vec<u8>> = [
        ((0, 1), vec![b'-', b'7', b'J']),
        ((0, -1), vec![b'-', b'F', b'L']),
        ((1, 0), vec![b'|', b'F', b'7']),
        ((-1, 0), vec![b'-', b'L', b'J']),
    ]
    .iter()
    .cloned()
    .collect();
    let mut i = 0;
    while !(a.0 == b.0 && a.1 == b.1) {
        dbg!(a);
        dbg!(b);
        count_grid[a.0 as usize][a.1 as usize] = count;
        count_grid[b.0 as usize][b.1 as usize] = count;
        update(&mut a, &mut aprev, &grid, &DX, &VALID_DX);
        update(&mut b, &mut bprev, &grid, &DX, &VALID_DX);
        count += 1;
        if count > 2 {
            break;
        }
    }
    return count + 1;
}

pub fn day_10_2() -> u64 {
    return 0;
}
