pub mod day3 {
    use std::collections::{HashMap, HashSet};

    use crate::advent::_read_file;
    pub fn day_3_1() -> u32 {
        let contents = _read_file("input_3_1".to_string());
        let lines: Vec<&str> = contents.lines().collect();
        let mut sum: u32 = 0;
        for (i, line) in lines.iter().enumerate() {
            let mut j: usize = 0;
            let bytes = line.as_bytes();
            while j < bytes.len() {
                if bytes[j].is_ascii_digit() {
                    let repr: &str = "";
                    let mut r = j;
                    let mut possible = false;
                    // consume and check
                    while r < bytes.len() && bytes[r].is_ascii_digit() {
                        // lines[i][r-1]
                        if r > 0 {
                            if let Some(v) = lines[i].as_bytes().get(r.saturating_sub(1)) {
                                possible = possible || (!v.is_ascii_digit() && *v == b'*');
                            }
                        }
                        // lines[i][r+1]
                        if let Some(v) = lines[i].as_bytes().get(r + 1) {
                            possible = possible || (!v.is_ascii_digit() && *v == b'*');
                        }
                        // lines[i-1][r-1]
                        // lines[i-1][r+1]
                        // lines[i-1][r]
                        if i > 0 {
                            if let Some(line_before) = lines.get(i.saturating_sub(1)) {
                                if let Some(v) = line_before.as_bytes().get(r + 1) {
                                    possible = possible || (!v.is_ascii_digit() && *v == b'*');
                                }
                                if let Some(v) = line_before.as_bytes().get(r.saturating_sub(1)) {
                                    possible = possible || (!v.is_ascii_digit() && *v == b'*');
                                }
                                let v = line_before.as_bytes()[r];
                                possible = possible || (!v.is_ascii_digit() && v == b'*');
                            }
                        }
                        // lines[i+1][r+1]
                        // lines[i+1][r-1]
                        // lines[i+1][r]
                        if let Some(line_after) = lines.get(i + 1) {
                            if let Some(v) = line_after.as_bytes().get(r + 1) {
                                possible = possible || (!v.is_ascii_digit() && *v == b'*');
                            }
                            if r > 0 {
                                if let Some(v) = line_after.as_bytes().get(r.saturating_sub(1)) {
                                    possible = possible || (!v.is_ascii_digit() && *v == b'*');
                                }
                            }
                            let v = line_after.as_bytes()[r];
                            possible = possible || (!v.is_ascii_digit() && v == b'*');
                        }
                        r += 1;
                    }
                    if possible {
                        let mut a = 0;
                        for (idx, n) in line.as_bytes()[j..r].iter().rev().enumerate() {
                            a += (*n - b'0') as u32 * 10_u32.pow(idx as u32);
                        }
                        sum += a;
                    }
                    j = r;
                } else {
                    j += 1;
                }
            }
        }
        return sum;
    }

    pub fn day_3_2() -> u32 {
        let contents = _read_file("input_3_1".to_string());
        let lines: Vec<&str> = contents.lines().collect();
        let mut sum: u32 = 0;
        let mut adj: HashMap<(usize, usize), Vec<u32>> = HashMap::new();
        for (i, line) in lines.iter().enumerate() {
            let mut j: usize = 0;
            let bytes = line.as_bytes();
            while j < bytes.len() {
                if bytes[j].is_ascii_digit() {
                    let repr: &str = "";
                    let mut r = j;
                    let mut possible = false;
                    let mut value_adjs = HashSet::new();
                    // consume and check
                    while r < bytes.len() && bytes[r].is_ascii_digit() {
                        // lines[i][r-1]
                        if r > 0 {
                            if let Some(v) = lines[i].as_bytes().get(r.saturating_sub(1)) {
                                if !v.is_ascii_digit() && *v == b'*' {
                                    possible = true;
                                    value_adjs.insert((i, r - 1));
                                }
                            }
                        }
                        // lines[i][r+1]
                        if let Some(v) = lines[i].as_bytes().get(r + 1) {
                            if !v.is_ascii_digit() && *v == b'*' {
                                possible = true;
                                value_adjs.insert((i, r + 1));
                            }
                        }
                        // lines[i-1][r-1]
                        // lines[i-1][r+1]
                        // lines[i-1][r]
                        if i > 0 {
                            if let Some(line_before) = lines.get(i.saturating_sub(1)) {
                                if let Some(v) = line_before.as_bytes().get(r + 1) {
                                    if !v.is_ascii_digit() && *v == b'*' {
                                        possible = true;
                                        value_adjs.insert((i - 1, r + 1));
                                    }
                                }
                                if let Some(v) = line_before.as_bytes().get(r.saturating_sub(1)) {
                                    if !v.is_ascii_digit() && *v == b'*' {
                                        possible = true;
                                        value_adjs.insert((i - 1, r - 1));
                                    }
                                }
                                let v = line_before.as_bytes()[r];
                                if !v.is_ascii_digit() && v == b'*' {
                                    possible = true;
                                    value_adjs.insert((i - 1, r));
                                }
                            }
                        }
                        // lines[i+1][r+1]
                        // lines[i+1][r-1]
                        // lines[i+1][r]
                        if let Some(line_after) = lines.get(i + 1) {
                            if let Some(v) = line_after.as_bytes().get(r + 1) {
                                if !v.is_ascii_digit() && *v == b'*' {
                                    possible = true;
                                    value_adjs.insert((i+1, r+1));
                                }
                            }
                            if r > 0 {
                                if let Some(v) = line_after.as_bytes().get(r.saturating_sub(1)) {
                                    if !v.is_ascii_digit() && *v == b'*' {
                                        possible = true;
                                        value_adjs.insert((i+1, r-1));
                                    }
                                }
                            }
                            let v = line_after.as_bytes()[r];
                            possible = possible || (!v.is_ascii_digit() && v == b'*');
                            if !v.is_ascii_digit() && v == b'*' {
                                possible = true;
                                value_adjs.insert((i+1, r));
                            }
                        }
                        r += 1;
                    }
                    if possible {
                        let mut a = 0;
                        for (idx, n) in line.as_bytes()[j..r].iter().rev().enumerate() {
                            a += (*n - b'0') as u32 * 10_u32.pow(idx as u32);
                        }
                        for adjacent in value_adjs {
                            if !adj.contains_key(&adjacent) {
                                adj.insert(adjacent.clone(), Vec::new());
                            }
                            let d: &mut Vec<u32> = adj.get_mut(&adjacent).unwrap();
                            d.push(a);
                        }
                    }
                    j = r;
                } else {
                    j += 1;
                }
            }
        }
        for (&(i, j), vec) in adj.iter() {
            if vec.len() == 2 {
                sum += vec[0] * vec[1];
            }

        }
        return sum;
    }
}
