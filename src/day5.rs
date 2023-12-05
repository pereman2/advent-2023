use std::cmp::{max, min};

pub fn day_5_1() -> u64 {
    let contents = include_str!("../input_5_1");
    let mut lines = contents.lines();
    let mut seeds = lines.next().unwrap().split(": ");
    let seeds = seeds.nth(1).unwrap().split(' ');
    let mut seeds: Vec<u64> = seeds.map(|s| s.parse::<u64>().unwrap()).collect();
    lines.next();
    for i in 0..7 {
        let mut ranges: Vec<(u64, u64, u64)> = vec![];
        let line = lines.next().unwrap();
        loop {
            let line = lines.next();
            if line.is_none() {
                break;
            }
            let line = line.unwrap();
            if line.is_empty() {
                break;
            }
            let range: Vec<u64> = line.split(" ").map(|v| v.parse::<u64>().unwrap()).collect();
            ranges.push((range[0], range[1], range[2]));
        }
        for seed in seeds.iter_mut() {
            for &(d, s, l) in &ranges {
                if *seed >= s && *seed < (s + l) {
                    let diff = *seed - s;
                    *seed = diff + d;
                    break;
                }
            }
        }
    }

    return *seeds.iter().min().unwrap();
}


pub fn day_5_2() -> u64 {
    let contents = include_str!("../input_5_1");
    let mut lines = contents.lines();
    let mut seeds = lines.next().unwrap().split(": ");
    let seeds = seeds.nth(1).unwrap().split(' ');
    let mut seeds: Vec<u64> = seeds.map(|s| s.parse::<u64>().unwrap()).collect();
    lines.next();
    for i in 0..7 {
        let mut ranges: Vec<(u64, u64, u64)> = vec![];
        lines.next();
        loop {
            let line = lines.next();
            if line.is_none() {
                break;
            }
            let line = line.unwrap();
            if line.is_empty() {
                break;
            }
            let range: Vec<u64> = line.split(" ").map(|v| v.parse::<u64>().unwrap()).collect();
            ranges.push((range[0], range[1], range[2]));
        }
        let mut prev_seeds = seeds;
        seeds = Vec::new();
        while prev_seeds.len() > 1 {
            let length = prev_seeds.pop().unwrap();
            let offset = prev_seeds.pop().unwrap();
            let mut pushed = false;
            for &(d, s, l) in &ranges {
                let overlap = offset < s+l && s < offset+length;
                if overlap {
                    let overlap_offset = max(offset, s); 
                    let overlap_end = min(offset+length, s+l); 
                    if overlap_offset > offset {
                        // head
                        prev_seeds.push(offset);
                        prev_seeds.push(overlap_offset - offset);
                    }
                    if overlap_end < offset+length {
                        prev_seeds.push(overlap_end);
                        prev_seeds.push(offset+length-overlap_end);
                    }
                    seeds.push(overlap_offset - s + d);
                    seeds.push(overlap_end - overlap_offset);
                    pushed = true;
                    break;
                } 
            }
            if !pushed {
                seeds.push(offset);
                seeds.push(length);
            }
        }
    }
    let mut _min: u64 = 1 << 61;
    for a in seeds.chunks(2) {
        _min = min(a[0], _min);
    }
    return _min;
}
