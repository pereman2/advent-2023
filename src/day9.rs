pub fn day_9_1() -> i64 {
    let contents = include_str!("../input_9_1");
    let mut res = 0;
    for line in contents.lines() {
        let mut all: Vec<Vec<i64>> = Vec::new();
        all.push(Vec::new());
        all[0] = line.split(' ').map(|v| v.parse::<i64>().unwrap()).collect();
        loop {
            let mut zero = true;
            let last = &all[all.len()-1];
            let mut new: Vec<i64> = Vec::new();
            let mut prev = *last.first().unwrap();
            for v in last.iter().skip(1) {
                if *v != 0 {
                    zero = false;
                }
                new.push(*v - prev);
                prev = *v;
            }
            all.push(new);
            if zero {
                break;
            }
        }
        let mut carry = 0;
        for values in all.iter().rev().skip(1) {
            carry += values.last().unwrap();
        }
        res += carry;
    }
    return res;
}

pub fn day_9_2() -> i64 {
    let contents = include_str!("../input_9_1");
    let mut res = 0;
    for line in contents.lines() {
        let mut all: Vec<Vec<i64>> = Vec::new();
        all.push(Vec::new());
        all[0] = line.split(' ').map(|v| v.parse::<i64>().unwrap()).collect();
        loop {
            let mut zero = true;
            let last = &all[all.len()-1];
            let mut new: Vec<i64> = Vec::new();
            let mut prev = *last.first().unwrap();
            for v in last.iter().skip(1) {
                if *v != 0 {
                    zero = false;
                }
                new.push(*v - prev);
                prev = *v;
            }
            all.push(new);
            if zero {
                break;
            }
        }
        let mut carry = 0;
        for values in all.iter().rev().skip(1) {
            carry = values.first().unwrap() - carry;
        }
        res += carry;
    }
    return res;
}