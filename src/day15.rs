use std::collections::BTreeMap;


pub fn day_15_1() -> u64 {
    let contents = include_str!("../input_15_1");
    let mut sum = 0;
    for s in contents.split(',') {
        let mut hash = 0;
        for c in s.as_bytes() {
            hash += *c as u64;
            hash *= 17;
            hash %= 256;
        }
        sum += hash;
    }
    return sum;
}

pub fn day_15_2() -> u64 {
    let contents = include_str!("../input_15_1");
    let mut sum = 0;
    let mut boxes: Vec<Vec<(&[u8], u8)>> = vec![Vec::new(); 256];
    for s in contents.split(',') {
        let mut hash = 0_usize;
        let bytes = s.as_bytes();
        for c in bytes {
            if *c == b'=' || *c == b'-' {
                break;
            }
            hash += *c as usize;
            hash *= 17;
            hash %= 256;
        }
        if *bytes.last().unwrap() == b'-' {
            let name = &bytes[0..bytes.len() - 1];
            for i in 0..boxes[hash].len() {
                if boxes[hash][i].0 == name {
                    boxes[hash].remove(i);
                    break;
                }
            }
        } else {
            let name = &bytes[0..bytes.len() - 2];
            let value = *bytes.last().unwrap();
            let mut found = false;
            for i in 0..boxes[hash].len() {
                if boxes[hash][i].0 == name {
                    boxes[hash][i].1 = value - b'0';
                    found = true;
                    break;
                }
            }
            if !found {
                boxes[hash].push((name, value - b'0'));
            }
        }
        // println!("{}: {:?}", hash, boxes);
    }
    for (bi, b) in boxes.iter().enumerate() {
        for (fi, (_, f)) in b.iter().enumerate() {
            sum += (bi+1) * (fi+1) * *f as usize;
        }
    }
    return sum as u64;
}