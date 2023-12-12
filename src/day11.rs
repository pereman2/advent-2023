use std::cmp::min;



pub fn day_11_1() -> u64 {
    let contents = include_str!("../input_11_1");
    let mut grid: Vec<Vec<u8>> = contents.lines().map(|line| line.as_bytes().to_vec()).collect();
    let mut i = 0;
    let mut rows: Vec<u64> = vec![0; grid.len()];
    let mut cols: Vec<u64> = vec![0; grid[0].len()];
    for l in grid.iter() {
        for c in l.iter() {
            print!("{}", *c as char);
        }
        println!("");
    }
    while i < grid.len() {
        for j in 0..grid[i].len() {
            rows[i] += (grid[i][j] != b'.') as u64;
            cols[j] += (grid[i][j] != b'.') as u64;
        }
        i += 1 ;
    }
    println!("rows: {:?}", rows);
    println!("cols: {:?}", cols);
    for i in 0..rows.len() {
        rows[i] = (rows[i] == 0) as u64;
    }
    for i in 0..cols.len() {
        cols[i] = (cols[i] == 0) as u64;
    }
    for i in 1..rows.len() {
        rows[i] = rows[i-1] + rows[i];
    }
    for i in 1..cols.len() {
        cols[i] = cols[i-1] + cols[i];
    }
    println!("rows: {:?}", rows);
    println!("cols: {:?}", cols);
    // create list of points that equal to '#'
    let mut points: Vec<(i64, i64)> = Vec::new();
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == b'#' {
                points.push((i as i64 + (rows[i] * 1) as i64 , j as i64 + (cols[j] * 1) as i64));
            }
        }
    }
    let mut sum = 0;
    // dbg!(&points);

    for (i, a) in points.iter().enumerate() {
        // let mut min_diff = 100000000_u32;
        if i == points.len() - 1 {
            break;
        }
        for b in points.iter().skip(i) {
            if a == b {
                continue;
            }
            let diff = a.0.abs_diff(b.0) + a.1.abs_diff(b.1);
            // min_diff = min(min_diff, diff);
            sum += diff as u64;
        }
        // dbg!(min_diff);
        // sum += min_diff as u64;
    }
    println!("rows: {:?}", rows);
    println!("cols: {:?}", cols);
    for l in grid.iter() {
        for c in l.iter() {
            print!("{}", *c as char);
        }
        println!("");
    }
    return sum;
}

pub fn day_11_2() -> u64 {
    let contents = include_str!("../input_11_1");
    let mut grid: Vec<Vec<u8>> = contents.lines().map(|line| line.as_bytes().to_vec()).collect();
    let mut i = 0;
    let mut rows: Vec<u64> = vec![0; grid.len()];
    let mut cols: Vec<u64> = vec![0; grid[0].len()];
    for l in grid.iter() {
        for c in l.iter() {
            print!("{}", *c as char);
        }
        println!("");
    }
    while i < grid.len() {
        for j in 0..grid[i].len() {
            rows[i] += (grid[i][j] != b'.') as u64;
            cols[j] += (grid[i][j] != b'.') as u64;
        }
        i += 1 ;
    }
    println!("rows: {:?}", rows);
    println!("cols: {:?}", cols);
    for i in 0..rows.len() {
        rows[i] = (rows[i] == 0) as u64;
    }
    for i in 0..cols.len() {
        cols[i] = (cols[i] == 0) as u64;
    }
    for i in 1..rows.len() {
        rows[i] = rows[i-1] + rows[i];
    }
    for i in 1..cols.len() {
        cols[i] = cols[i-1] + cols[i];
    }
    println!("rows: {:?}", rows);
    println!("cols: {:?}", cols);
    // create list of points that equal to '#'
    let mut points: Vec<(u64, u64)> = Vec::new();
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == b'#' {
                points.push((i as u64 + (rows[i] * 1000000) as u64 - rows[i] as u64 , j as u64 + (cols[j] * 1000000) as u64 - cols[j] as u64));
            }
        }
    }
    let mut sum = 0;
    // dbg!(&points);

    for (i, a) in points.iter().enumerate() {
        // let mut min_diff = 100000000_u32;
        if i == points.len() - 1 {
            break;
        }
        for b in points.iter().skip(i) {
            if a == b {
                continue;
            }
            let diff = a.0.abs_diff(b.0) + a.1.abs_diff(b.1);
            // min_diff = min(min_diff, diff);
            sum += diff as u64;
        }
        // dbg!(min_diff);
        // sum += min_diff as u64;
    }
    println!("rows: {:?}", rows);
    println!("cols: {:?}", cols);
    for l in grid.iter() {
        for c in l.iter() {
            print!("{}", *c as char);
        }
        println!("");
    }
    return sum;
}