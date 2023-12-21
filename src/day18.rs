use std::{
    cmp::{max, min},
    sync::{Arc, Mutex},
};

#[derive(Debug, Clone)]
struct Point {
    pos: (i64, i64),
    dx: (i64, i64),
}
pub fn day_18_1() -> u64 {
    let DX = vec![(1, 0), (-1, 0)];
    let contents = include_str!("../input_18_1");
    let mut sum = 0;
    let mut painted: Vec<Point> = Vec::new();
    let mut cur = (0, 0);
    let mut max_i = i64::MIN;
    let mut max_j = i64::MIN;
    let mut min_i = i64::MAX;
    let mut min_j = i64::MAX;
    for line in contents.lines() {
        let mut line = line.split(" ");
        let dir = line.next().unwrap();
        let amount = line.next().unwrap().parse::<u64>().unwrap();
        // let color = line.next().unwrap();
        let dx = match dir {
            "U" => (-1, 0),
            "D" => (1, 0),
            "R" => (0, 1),
            "L" => (0, -1),
            _ => panic!("bad dir"),
        };
        painted.push(Point { pos: cur, dx: dx });
        cur.0 += dx.0 * amount as i64;
        cur.1 += dx.1 * amount as i64;
        // println!("{} {}", cur.0, cur.1);
        max_i = max(max_i, cur.0);
        max_j = max(max_j, cur.1);
        min_i = min(min_i, cur.0);
        min_j = min(min_j, cur.1);
    }
    painted.push(painted[0].clone());
    let pi = min_i.abs();
    let pj = min_j.abs();
    let len_i = max_i + pi;
    let len_j = max_j + pj;
    // println!("abs {} {} {} {}", min_i.abs(), max_i.abs(), min_j.abs(), max_j.abs());
    let mut grid = vec![vec![(b'.', (0, 0), false); len_j as usize + 1]; len_i as usize + 1];
    // 72952 too high
    // 71393
    for pair in painted.windows(2) {
        let mut first = pair[0].clone();
        let mut second = pair[1].clone();
        // println!();
        // println!("before {:?} {:?}", first, second);
        first.pos.0 += pi;
        first.pos.1 += pj;
        second.pos.0 += pi;
        second.pos.1 += pj;
        // println!("after {:?} {:?}", first, second);
        for i in first.pos.0..second.pos.0 {
            grid[i as usize][first.pos.1 as usize] = (b'#', first.dx, false);
        }
        for i in second.pos.0 + 1..=first.pos.0 {
            grid[i as usize][first.pos.1 as usize] = (b'#', first.dx, false);
        }
        for j in first.pos.1..second.pos.1 {
            grid[first.pos.0 as usize][j as usize] = (b'#', first.dx, false);
        }
        for j in second.pos.1 + 1..=first.pos.1 {
            grid[first.pos.0 as usize][j as usize] = (b'#', first.dx, false);
        }
        grid[first.pos.0 as usize][first.pos.1 as usize] = (b'O', first.dx, true);
    }
    for line in &grid {
        for (c, dx, vertex) in line {
            print!("{}", *c as char);
        }
        println!();
    }
    println!();
    for i in 0..grid.len() {
        let mut crossings = 0;
        let mut first = true;
        let mut previous_dx = (0, 0);
        for j in 0..grid[i].len() {
            if grid[i][j].0 == b'#' || grid[i][j].0 == b'O' {
                let is_vertex = grid[i][j].2;
                if is_vertex {
                    if first {
                        for dx in &DX {
                            let previous_i = i as i64 + dx.0;
                            let previous_j = j as i64 + dx.1;
                            if previous_i >= 0
                                && previous_i < grid.len() as i64
                                && previous_j >= 0
                                && previous_j < grid[i].len() as i64
                            {
                                if grid[previous_i as usize][previous_j as usize].0 == b'#' {
                                    if grid[previous_i as usize][previous_j as usize].1 == (1, 0)
                                        || grid[previous_i as usize][previous_j as usize].1
                                            == (-1, 0)
                                    {
                                        previous_dx = *dx;
                                        first = false;
                                        break;
                                    }
                                }
                            }
                        }
                    } else {
                        let mut this_dx = grid[i][j].1;
                        for dx in &DX {
                            let previous_i = i as i64 + dx.0;
                            let previous_j = j as i64 + dx.1;
                            if previous_i >= 0
                                && previous_i < grid.len() as i64
                                && previous_j >= 0
                                && previous_j < grid[i].len() as i64
                            {
                                if grid[previous_i as usize][previous_j as usize].0 == b'#' {
                                    if grid[previous_i as usize][previous_j as usize].1 == (1, 0)
                                        || grid[previous_i as usize][previous_j as usize].1
                                            == (-1, 0)
                                    {
                                        this_dx = *dx;
                                        break;
                                    }
                                }
                            }
                        }
                        if previous_dx != this_dx {
                            crossings += 1;
                        }
                        first = true;
                    }
                } else {
                    if grid[i][j].1 == (-1, 0) || grid[i][j].1 == (1, 0) {
                        crossings += 1;
                    }
                }
            }
            if grid[i][j].0 == b'.' && crossings % 2 == 1 {
                grid[i][j].0 = b'#';
            }
        }
    }
    for line in &grid {
        for (c, dx, vertex) in line {
            print!("{}", *c as char);
        }
        println!();
    }
    let sum = grid
        .iter()
        .map(|line| {
            line.iter()
                .map(|(c, _, _)| if *c == b'#' || *c == b'O' { 1 } else { 0 })
                .sum::<usize>()
        })
        .sum::<usize>();
    return sum as u64;
}

pub fn day_18_2() -> u64 {
    let DX = vec![(1, 0), (-1, 0)];
    let contents = include_str!("../input_18_1");
    let mut sum = 0;
    let mut painted: Vec<Point> = Vec::new();
    let mut cur = (0, 0);
    let mut max_i = i64::MIN;
    let mut max_j = i64::MIN;
    let mut min_i = i64::MAX;
    let mut min_j = i64::MAX;
    painted.push(Point { pos: cur, dx: (0, 0) });
    let mut perimeter = 0_u64;
    for line in contents.lines() {
        let mut line = line.split(" ");
        let dir = line.next().unwrap();
        let amount = line.next().unwrap().parse::<u64>().unwrap();
        let color = line.next().unwrap();
        println!("{:?}", color);
        let amount = u64::from_str_radix(&color[2..7], 16).unwrap();
        let dir = u64::from_str_radix(&color[7..8], 16).unwrap();
        println!("{} {}", amount, dir);
        let dx = match dir {
            3 => (-1, 0),
            1 => (1, 0),
            0 => (0, 1),
            2 => (0, -1),
            _ => panic!("bad dir"),
        };
        // let dx = match dir {
        //     "U" => (-1, 0),
        //     "D" => (1, 0),
        //     "R" => (0, 1),
        //     "L" => (0, -1),
        //     _ => panic!("bad dir"),
        // };
        cur.0 += dx.0 * amount as i64;
        cur.1 += dx.1 * amount as i64;
        perimeter += amount;
        painted.push(Point { pos: cur, dx: dx });
        // println!("{} {}", cur.0, cur.1);
        max_i = max(max_i, cur.0);
        max_j = max(max_j, cur.1);
        min_i = min(min_i, cur.0);
        min_j = min(min_j, cur.1);
    }
    let pi = min_i.abs();
    let pj = min_j.abs();
    println!(
        "abs {} {} {} {}",
        min_i.abs(),
        max_i.abs(),
        min_j.abs(),
        max_j.abs()
    );
    let mut area = 0;

    for i in 0..painted.len() - 1 {
        let (x1, y1) = painted[i].pos;
        let (x2, y2) = painted[i + 1].pos;
        area += x1 * y2 - y1 * x2;
    }

    return ((area.abs() + perimeter as i64) / 2) as u64 + 1;
}

