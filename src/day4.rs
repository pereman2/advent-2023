use std::collections::HashMap;

use crate::advent::_read_file;

pub fn day_4_1() -> u32 {
    let contents = _read_file("day_4_1".to_string());
    let mut sum = 0;
    for line in contents.lines() {
        let values: &str = line.split(":").nth(1).unwrap().trim();
        let mut values = values.split("| ");
        let a = values.next().unwrap().split_whitespace();
        let mut seen: HashMap<String, bool> = HashMap::new();
        let b = values.next().unwrap().split_whitespace();
        let mut p = 0;
        for v in a {
            seen.insert(v.to_string(), true);
        } 
        for v in b {
            if seen.contains_key(v) {
                p += 1;
            }
        } 
        if p > 0 {
            sum += 1 << (p-1);
        }
    }

    return sum;
}

pub fn day_4_2() -> u32 {
    let contents = _read_file("day_4_1".to_string());
    let mut sum = 0;
    let mut copies: Vec<u32> = vec![0; 500];
    for (i, line) in contents.lines().enumerate() {
        let values: &str = line.split(":").nth(1).unwrap().trim();
        let mut values = values.split("| ");
        let a = values.next().unwrap().split_whitespace();
        let mut seen: HashMap<String, bool> = HashMap::new();
        let b = values.next().unwrap().split_whitespace();
        let mut p = 0;
        for v in a {
            seen.insert(v.to_string(), true);
        } 
        for v in b {
            if seen.contains_key(v) {
                p += 1;
            }
        } 
        copies[i] += 1;
        sum += copies[i];
        if p == 0 {
            continue;
        }
        for c in i+1..=i+p {
            copies[c] += copies[i];
        }
    }
    sum
}