
use std::{cmp::{min, max}, collections::HashSet};

#[derive(Debug)]
#[derive(Clone)]
struct Brick {
    x: i64,
    y: i64,
    z: i64,
    x2: i64,
    y2: i64,
    z2: i64,
}

impl Brick {
    fn min_z(&self) -> i64 {
        min(self.z, self.z2)
    }
    
    fn max_z(&self) -> i64 {
        max(self.z, self.z2)
    }
}

fn cube_intersect_all(c1: &Brick, c2: &Brick) -> bool {
    // they intersect if x x2 and y y2 overlap
    let x_overlap = c1.x <= c2.x2 && c2.x <= c1.x2;
    let y_overlap = c1.y <= c2.y2 && c2.y <= c1.y2;
    let z_overlap = c1.z <= c2.z2 && c2.z <= c1.z2;
    return x_overlap && y_overlap && z_overlap;
}

fn cube_intersect_xy(c1: &Brick, c2: &Brick) -> bool {
    // they intersect if x x2 and y y2 overlap
    let x_overlap = c1.x <= c2.x2 && c2.x <= c1.x2;
    let y_overlap = c1.y <= c2.y2 && c2.y <= c1.y2;
    let z_overlap = c1.z <= c2.z2 && c2.z <= c1.z2;
    return x_overlap && y_overlap;
}

pub fn day_22_1() -> u64 {
    let contents = include_str!("../input_22_1");
    let mut points = Vec::new();
    let mut count = 1;
    let mut max_z = 0;
    for line in contents.lines() {
        let mut f = line.split('~');
        let mut p1 = f.next().unwrap().split(',');
        let x = p1.next().unwrap().parse::<i64>().unwrap();
        let y = p1.next().unwrap().parse::<i64>().unwrap(); 
        let z = p1.next().unwrap().parse::<i64>().unwrap(); 
        let mut p2 = f.next().unwrap().split(',');
        let x2 = p2.next().unwrap().parse::<i64>().unwrap();
        let y2 = p2.next().unwrap().parse::<i64>().unwrap(); 
        let z2 = p2.next().unwrap().parse::<i64>().unwrap(); 
        let cube = Brick { x, y, z, x2, y2, z2 };
        max_z = max(max_z, cube.max_z());

        points.push(cube);
    }

    points.sort_by(|a, b| min(a.z, a.z2).cmp(&min(b.z, b.z2)));

    let mut fall = 0;
    let mut it = 0;
    while fall != points.len() {
        it += 1;
        println!("it {}", it);
        fall = 0;
        for i in 0..points.len() {
            let cube = &points[i];
            let mut _min = 1;
            let mut _max_intersect = 0;
            let mut no_move = false;
            if cube.min_z() == 1 {
                fall += 1;
                continue;
            }
            for j in 0..points.len() {
                let other = &points[j];
                // need to check it is the closest one
                // maybe one falls back more than others
                // check all intersecting cubes
                // find the closest one to move close to it
                // iterte through z ?
                if i == j {
                    continue;
                }
                assert!(!cube_intersect_all(cube, other));
                if cube.min_z() == other.max_z() + 1 && cube_intersect_xy(&cube, &other) {
                    // move down
                    // println!("cube {:?} intersects with {:?}", cube, other);
                    no_move = true;
                    break;
                }
            }
            if no_move {
                fall += 1;
                continue;
            }
            let cube = &mut points[i];
            cube.z -= 1;
            cube.z2 -= 1;
            // println!("cube {:?} to {:?} with dist {}", cube, previous, dist);
        }
    }

    points.sort_by(|a, b| min(a.z, a.z2).cmp(&min(b.z, b.z2)));
    let mut count = 0;

    // 0: nodes that support this
    // 1: supported nodes by this
    let mut support_matrix: Vec<(HashSet<usize>, HashSet<usize>)> = vec![(HashSet::new(), HashSet::new()); points.len()];
    for (i, cube) in points.iter().enumerate() {
        for (j, cube2) in points.iter().enumerate() {
            if j == i {
                continue;
            }
            if cube2.min_z() == cube.max_z() + 1 && cube_intersect_xy(cube, cube2) {
                support_matrix[i].1.insert(j);
            }
            if cube2.max_z() == cube.min_z() - 1 && cube_intersect_xy(cube, cube2) {
                support_matrix[i].0.insert(j);
            }
        }
    }


    for (i, (supports, supported)) in support_matrix.iter().enumerate() {
        if supported.len() == 0 {
            println!("removed {:?}", points[i]);
            count += 1;
            continue;
        } 

        let mut valid = true;
        for do_support in supported {
            if support_matrix[*do_support].0.len() == 1 {
                valid = false;
                break;
            }
        }
        if valid {
            println!("removed {:?}", points[i]);
            count += 1;
        }
    }

    return count;
}

pub fn day_22_2() -> u64 {
    let contents = include_str!("../input_22_1");
    let mut points = Vec::new();
    let mut count = 1;
    let mut max_z = 0;
    for line in contents.lines() {
        let mut f = line.split('~');
        let mut p1 = f.next().unwrap().split(',');
        let x = p1.next().unwrap().parse::<i64>().unwrap();
        let y = p1.next().unwrap().parse::<i64>().unwrap(); 
        let z = p1.next().unwrap().parse::<i64>().unwrap(); 
        let mut p2 = f.next().unwrap().split(',');
        let x2 = p2.next().unwrap().parse::<i64>().unwrap();
        let y2 = p2.next().unwrap().parse::<i64>().unwrap(); 
        let z2 = p2.next().unwrap().parse::<i64>().unwrap(); 
        let cube = Brick { x, y, z, x2, y2, z2 };
        max_z = max(max_z, cube.max_z());

        points.push(cube);
    }

    points.sort_by(|a, b| min(a.z, a.z2).cmp(&min(b.z, b.z2)));
    // println!("{:?}", points);
    // println!("start points");
    // for point in &points {
    //     println!("{} {} {}", point.x, point.y, point.z);
    //     println!("{} {} {}", point.x2, point.y2, point.z2);
    //     println!("");
    //     println!("");

    // }
    // println!("end points");

    let mut fall = 0;
    let mut it = 0;
    while fall != points.len() {
        it += 1;
        fall = 0;
        for i in 0..points.len() {
            let cube = &points[i];
            let mut _min = 1;
            let mut _max_intersect = 0;
            let mut no_move = false;
            if cube.min_z() == 1 {
                fall += 1;
                continue;
            }
            for j in 0..points.len() {
                let other = &points[j];
                // need to check it is the closest one
                // maybe one falls back more than others
                // check all intersecting cubes
                // find the closest one to move close to it
                // iterte through z ?
                if i == j {
                    continue;
                }
                assert!(!cube_intersect_all(cube, other));
                if cube.min_z() == other.max_z() + 1 && cube_intersect_xy(&cube, &other) {
                    // move down
                    // println!("cube {:?} intersects with {:?}", cube, other);
                    no_move = true;
                    break;
                }
            }
            if no_move {
                fall += 1;
                continue;
            }
            let cube = &mut points[i];
            cube.z -= 1;
            cube.z2 -= 1;
            // println!("cube {:?} to {:?} with dist {}", cube, previous, dist);
        }
    }

    points.sort_by(|a, b| min(a.z, a.z2).cmp(&min(b.z, b.z2)));
    let mut count = 0;

    // 0: nodes that support this
    // 1: supported nodes by this
    let mut support_matrix: Vec<(HashSet<usize>, HashSet<usize>)> = vec![(HashSet::new(), HashSet::new()); points.len()];
    for (i, cube) in points.iter().enumerate() {
        for (j, cube2) in points.iter().enumerate() {
            if j == i {
                continue;
            }
            if cube2.min_z() == cube.max_z() + 1 && cube_intersect_xy(cube, cube2) {
                support_matrix[i].1.insert(j);
            }
            if cube2.max_z() == cube.min_z() - 1 && cube_intersect_xy(cube, cube2) {
                support_matrix[i].0.insert(j);
            }
        }
    }

    let mut res = 0;
    for (i, (supports, supported)) in support_matrix.iter().enumerate() {
        let mut fall = 0;
        let mut stack = Vec::new();
        let mut do_fall = HashSet::new();
        for do_support in &support_matrix[i].1 {
            if support_matrix[*do_support].0.len() == 1 {
                fall += 1;
                do_fall.insert(*do_support);
                stack.push(*do_support);
            }
        }
        while stack.len() > 0 {
            let current = stack.pop().unwrap();
            for do_support in &support_matrix[current].1 {
                let mut supporting = 0;
                for supported in &support_matrix[*do_support].0 {
                    if !do_fall.contains(supported) {
                        supporting += 1;
                    }
                }
                if supporting == 0 {
                    fall += 1;
                    do_fall.insert(*do_support);
                    stack.push(*do_support);
                }
            }
        }
        res += do_fall.len() as u64;
    }

    return res;
}